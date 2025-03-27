use regex::Regex;

pub fn path_pattern_to_regex(pattern: &str) -> Regex {
	let mut regex_str = "^".to_string();
	let mut param_mode = false;
	let mut param_name = String::new();

	for c in pattern.chars() {
		match c {
			'{' => {
				param_mode = true;
				param_name.clear();
			}
			'}' => {
				regex_str.push_str(&format!("(?P<{}>[^/]+)", param_name));
				param_mode = false;
			}
			_ => {
				if param_mode {
					param_name.push(c);
				} else {
					if r".+*?^$()[]|\\".contains(c) {
						regex_str.push('\\');
					}
					regex_str.push(c);
				}
			}
		}
	}

	regex_str.push('$');
	Regex::new(&regex_str).expect("Invalid path pattern -> regex")
}

pub fn extract_param_names(pattern: &str) -> Vec<String> {
	let mut names = Vec::new();
	let mut in_braces = false;
	let mut buf = String::new();

	for c in pattern.chars() {
		match c {
			'{' => {
				in_braces = true;
				buf.clear();
			}
			'}' => {
				in_braces = false;
				names.push(buf.clone());
				buf.clear();
			}
			_ => {
				if in_braces {
					buf.push(c);
				}
			}
		}
	}
	names
}