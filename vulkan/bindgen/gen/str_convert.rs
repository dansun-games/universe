pub fn convert_identifier(ident: &str) -> String {
	let mut name = String::new();
	name.reserve(ident.len() + 5); // extra to avoid having to reallocate for inserting _

	let mut upper_flag = false;
	for ch in ident.chars() {
		if ch.is_uppercase() {
			//if the uppercase is a continuation then don't seperate
			if !upper_flag {
				name.insert(name.len(), '_');
			}
			name.insert(name.len(), ch.to_ascii_lowercase());
			upper_flag = true;
			continue;
		}
		upper_flag = false;

		//underscores get ignored otherwise we have double underscore
		if ch != '_' {
			//i think this could break if you have lowercase after underscore.
			//else with upperflag = true would fix i think?
			//not worried about it right now
			name.insert(name.len(), ch);
		}
	}

	if name == "type" {
		name.insert_str(0, "r#");
	}

	name
}
