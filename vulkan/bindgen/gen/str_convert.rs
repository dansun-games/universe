pub fn strip_vk(ident: &str) -> &str {
	println!("{}", ident);
	ident.strip_prefix("Vk").expect("missing vk prefix")
}

pub fn fix_pascal(ident: &str) -> String {
	let mut name = String::new();
	name.reserve_exact(name.len());
	
	let mut prev_upper = false;
	for ch in ident.chars() {
		if ch.is_uppercase() && prev_upper { //upper continuation
			name.push(ch.to_ascii_lowercase());
		} else if ch != '_' {
			name.push(ch)
		}
		prev_upper = ch.is_uppercase();
	}
	
	name
}

pub fn convert_const_name(name: &str) -> String {
	let name = name.trim_start_matches("VK_");
	name.to_owned()
}