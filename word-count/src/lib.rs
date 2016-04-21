use std::collections::HashMap;

pub fn word_count(txt: &str) -> HashMap<String, u32>{
	let words = txt.to_lowercase().chars()
		.map(|c| if !c.is_alphanumeric() {' '} else { c })
		.collect::<String>();

	let mut book = HashMap::new();
	for word in words.split_whitespace(){
		let counter = book.entry(word.to_string()).or_insert(0);
		*counter += 1;
	}
	book
}