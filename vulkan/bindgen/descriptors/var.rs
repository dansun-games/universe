use vk_parse as vk;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VarDescriptor {
	pub name: String,
	pub c_type: String,
	pub readonly: bool,
	pub ptr: bool,
	pub optional: bool,
	pub meta: Option<VariableMeta>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum VariableMeta {
	ConstValueRef(String),
	UnionDescriminator(String),
	HandleType(String),
	ArrayLens(Vec<ArrayLen>),
	StructChainTypes(Vec<String>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ArrayLen {
	NullTerm,
	LocalRef(String),
	GlobalRef(String),
	Value(u32),
}

impl From<&vk::TypeMemberDefinition> for VarDescriptor {
	fn from(def: &vk::TypeMemberDefinition) -> Self {
		let mut var_desc = parse_c_type(def.code.as_str());
		let len = def.altlen.as_ref().or(def.len.as_ref()).map(|s| s.clone());

		// sometimes they do this weird: true,false / false,true / true,true... we dont really care about all that.
		var_desc.optional = def
			.optional
			.as_ref()
			.map(|v| v.contains("true"))
			.unwrap_or_default();

		if let Some(value) = len {
			assert!(var_desc.ptr); //must be a ptr type if len is specified like this
			assert_eq!(var_desc.meta, None);
			let items = value
				.split(',')
				.map(|s| match s {
					"null-term" => ArrayLen::NullTerm,
					_ => ArrayLen::LocalRef(s.to_owned()),
				})
				.collect();
			var_desc.meta = Some(VariableMeta::ArrayLens(items));
		}

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

		var_desc
	}
}

impl From<&vk::CommandParam> for VarDescriptor {
	fn from(def: &vk::CommandParam) -> Self {
		let mut var_desc = parse_c_type(def.definition.code.as_str());
		let len = def.altlen.as_ref().or(def.len.as_ref()).map(|s| s.clone());

		if let Some(value) = len {
			assert!(var_desc.ptr); //must be a ptr type if len is specified like this
			assert_eq!(var_desc.meta, None);
			let items = value
				.split(',')
				.map(|s| match s {
					"null-term" => ArrayLen::NullTerm,
					_ => ArrayLen::LocalRef(s.to_owned()),
				})
				.collect();
			var_desc.meta = Some(VariableMeta::ArrayLens(items));

			// We are ignoring this attribute for now, it is only used on two methods.
			// I just dont yet understand the context of why it would be useful because
			// the pointer array its referencing has structures of a fixed size.
			let _stride = &def.stride;
		}

		//see note from struct param on this section
		var_desc.optional = def
			.optional
			.as_ref()
			.map(|v| v.contains("true"))
			.unwrap_or_default();

		let valid_structs = def.validstructs.clone();
		if !valid_structs.is_empty() {
			assert_eq!(var_desc.meta, None);
			var_desc.meta = Some(VariableMeta::StructChainTypes(valid_structs));
		}

		if let Some(value) = def.objecttype.clone() {
			assert_eq!(var_desc.meta, None);
			var_desc.meta = Some(VariableMeta::HandleType(value));
		}

		var_desc
	}
}

/// This function "eats" the string from left to right, converting the syntax into VarDescriptor.
/// Really, a ebnf parser would be more reliable here, but this is fine for now.
fn parse_c_type(code: &str) -> VarDescriptor {
	let mut code = code.trim();

	let readonly = code.starts_with("const");
	if readonly {
		code = code[5..].trim_start();
	}
	code = code.trim_start_matches("struct").trim_start();

	let delim = code
		.find(|c| c == ' ' || c == '*')
		.expect("Could not get delim after type identifier");
	let c_type = code[..delim].to_string();
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

	//Assuming the data ptr is always readonly if the first one is.
	if double_ptr {
		assert_eq!(double_readonly, readonly);
	}

	let delim = code
		.find(|c| c == '[' || c == ':' || c == ' ')
		.unwrap_or(code.len());
	let name = code[..delim].to_string();
	code = code[delim..].trim_start();

	let mut arr_lengths: Vec<ArrayLen> = vec![];
	loop {
		if !code.starts_with('[') {
			break;
		}

		let delim = code
			.find(|c| c == ']')
			.expect("Missing closing array brace");
		let inner = code[1..delim].to_owned();
		code = code[delim + 1..].trim_start();

		arr_lengths.push(match inner.parse::<u32>() {
			Ok(v) => ArrayLen::Value(v),
			Err(_e) => ArrayLen::GlobalRef(inner),
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

	let meta = if !arr_lengths.is_empty() {
		Some(VariableMeta::ArrayLens(arr_lengths))
	} else {
		None
	};

	VarDescriptor {
		name,
		c_type,
		readonly,
		ptr,
		optional: false,
		meta,
	}
}
