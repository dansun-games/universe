use std::vec;

use vk_parse as vk;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VarDescriptor {
	/// spec name
	pub name: String,
	// The c_type with all ptr and array removed
	pub c_type: String,
	/// Whether or not the c_type is optional. This is seperate from ptr optionality.
	/// For Handles, this means you can pass NULL_HANDLE
	/// For primitive types, this means you can the 0 value
	pub optional: bool,
	/// Sized array length
	pub arr_lens: Vec<StaticArrayLen>,
	/// Pointers to the c_type listed in outer->inner order (rust style).
	pub ptrs: Vec<PtrDescriptor>,
	/// Various
	pub meta: Option<VariableMeta>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VariableMeta {
	ConstValueRef(String),
	UnionDescriminator(String),
	HandleType(String),
	StructChainTypes(Vec<String>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VecLen {
	NullTerm,
	LocalRef(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StaticArrayLen {
	GlobalRef(String),
	Value(u32),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PtrDescriptor {
	pub readonly: bool,
	pub optional: bool,
	pub len: Option<VecLen>,
}

impl From<&vk::TypeMemberDefinition> for VarDescriptor {
	fn from(def: &vk::TypeMemberDefinition) -> Self {
		let len = def.altlen.as_ref().or(def.len.as_ref()).map(|s| s.clone());

		let opt_specs: Vec<_> = parse_opt_spec(def.optional.as_ref().map(|v| v.as_str()));
		let len_specs: Vec<_> = len.as_ref().map_or(vec![], |v| v.split(',').collect());

		let mut var_desc = parse_c_type(&def.code, len_specs, opt_specs);

		if let Some(value) = def.values.clone() {
			assert_eq!(var_desc.meta, None);
			var_desc.meta = Some(VariableMeta::ConstValueRef(value));
		}

		if let Some(value) = def.objecttype.clone() {
			assert_eq!(var_desc.meta, None);
			var_desc.meta = Some(VariableMeta::HandleType(value));
		}

		if let Some(value) = def.selector.clone() {
			assert_eq!(var_desc.meta, None);
			var_desc.meta = Some(VariableMeta::UnionDescriminator(value));
		}

		if var_desc.optional {
			println!("{:#?}", var_desc);
		}

		var_desc
	}
}

impl From<&vk::CommandParam> for VarDescriptor {
	fn from(def: &vk::CommandParam) -> Self {
		let len = def.altlen.as_ref().or(def.len.as_ref()).map(|s| s.clone());

		let opt_specs: Vec<_> = parse_opt_spec(def.optional.as_ref().map(|v| v.as_str()));
		let len_specs: Vec<_> = len.as_ref().map_or(vec![], |v| v.split(',').collect());
		
		let mut var_desc = parse_c_type(&def.definition.code, len_specs, opt_specs);

		// We are ignoring this attribute for now, it is only used on two methods.
		// I just dont yet understand the context of why it would be useful because
		// the pointer array its referencing has structures of a fixed size.
		let _stride = &def.stride;

		let valid_structs = def.validstructs.clone();
		if !valid_structs.is_empty() {
			assert_eq!(var_desc.meta, None);
			var_desc.meta = Some(VariableMeta::StructChainTypes(valid_structs));
		}

		if let Some(value) = def.objecttype.clone() {
			assert_eq!(var_desc.meta, None);
			var_desc.meta = Some(VariableMeta::HandleType(value));
		}

		if var_desc.optional {
			println!("{:#?}", var_desc);
		}

		var_desc
	}
}

/// This function "eats" the string from left to right, converting the syntax into VarDescriptor.
/// Really, a ebnf parser would be more reliable here, but this is fine for now.
fn parse_c_type(code: &str, mut len_specs: Vec<&str>, mut opt_specs: Vec<bool>) -> VarDescriptor {
	let mut code = code.trim();

	let readonly = code.starts_with("const");
	if readonly {
		code = code[5..].trim_start();
	}
	code = code.trim_start_matches("struct").trim_start();

	let delim = code
		.find(|c| c == ' ' || c == '*')
		.expect("Could not get delim after type identifier");
	let c_type = code[..delim].trim_start_matches("Vk").to_owned();
	code = code[delim..].trim_start();

	let ptr = code.starts_with("*");
	if ptr {
		code = code[1..].trim_start();
	}

	let double_readonly = code.starts_with("const");
	if double_readonly {
		code = code[5..].trim_start();
	}

	let double_ptr = code.starts_with("*");
	if double_ptr {
		code = code[1..].trim_start();
	}

	let delim = code
		.find(|c| c == '[' || c == ':' || c == ' ')
		.unwrap_or(code.len());
	let name = code[..delim].to_owned();
	code = code[delim..].trim_start();

	let mut arr_lens: Vec<StaticArrayLen> = vec![];
	loop {
		if !code.starts_with('[') {
			break;
		}

		let delim = code
			.find(|c| c == ']')
			.expect("Missing closing array brace");
		let inner = code[1..delim].to_owned();
		code = code[delim + 1..].trim_start();

		arr_lens.push(match inner.parse::<u32>() {
			Ok(v) => StaticArrayLen::Value(v),
			Err(_e) => StaticArrayLen::GlobalRef(inner),
		});
	}

	//We just eat the bit layout field, they only use it for places where c compilers are unrealiable.
	if code.starts_with(":") {
		code = code[1..].trim_start();
		let end = code.find(|c: char| !c.is_numeric()).unwrap_or(code.len());
		code = code[end..].trim_start();
	}

	//Make sure we have processed all the input
	assert_eq!(code.len(), 0);

	//
	//Collect all our properties into the correct structures
	//

	//we reverse because its easier to pop off the end to consume the items
	//items look like "enabledExtensionCount,null-terminated" for a type like "char* const* ppEnabledExtensionNames"
	len_specs.reverse();
	let mut vec_lens: Vec<_> = len_specs
		.iter()
		.map(|s| {
			let s = *s;
			match s {
				"null-term" => VecLen::NullTerm,
				_ => VecLen::LocalRef(s.to_owned()),
			}
		})
		.collect();

	let ptrs: Vec<PtrDescriptor> = if double_ptr {
		assert!(ptr);
		vec![
			PtrDescriptor {
				len: vec_lens.pop(),
				optional: opt_specs.pop().unwrap_or(false),
				readonly,
			},
			PtrDescriptor {
				len: vec_lens.pop(),
				optional: opt_specs.pop().unwrap_or(false),
				readonly: double_readonly,
			},
		]
	} else if ptr {
		vec![PtrDescriptor {
			len: vec_lens.pop(),
			optional: opt_specs.pop().unwrap_or(false),
			readonly,
		}]
	} else {
		vec![]
	};
	assert_eq!(vec_lens.len(), 0);

	//last optional is for things like handle types allowing HANDLE_NULL
	let optional = opt_specs.pop().unwrap_or(false);
	assert_eq!(opt_specs.len(), 0);

	VarDescriptor {
		name,
		c_type,
		optional,
		arr_lens,
		ptrs,
		meta: None,
	}
}

fn parse_opt_spec(spec: Option<&str>) -> Vec<bool> {
	spec.map_or(vec![], |v| {
		v.split(',')
			.map(|v| v.parse().expect("Could not parse bool"))
			.collect()
	})
}