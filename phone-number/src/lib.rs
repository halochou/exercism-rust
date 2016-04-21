pub fn number(raw : &str) -> Option<String> {
	let clean : String = raw.chars().filter(|c| c.is_numeric()).collect();
	match clean {
		_ if clean.len() == 10 => Some(clean),
		_ if clean.len() == 11 && clean.starts_with("1") => Some((&clean[1..]).to_string()),
		_ => None
	}
}

pub fn area_code(num : &str) -> Option<String> {
	if let Some(n) = number(num) {
		Some( (&n[0..3]).to_string() )
	} else {
		None
	}

}

pub fn pretty_print(num : &str) -> String {
	if let Some(n) = number(num) {
		format!("({}) {}-{}", &n[0..3], &n[3..6], &n[6..]).to_string()
	} else {
		"invalid".to_string()
	}
}