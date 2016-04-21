use std::collections::BTreeMap;

pub fn transform(input : &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
	let mut res = BTreeMap::new();
	for (score, letters) in input.into_iter() {
		for letter in letters {
			res.insert(letter.to_lowercase().to_string(), *score);
		}
	}
	res
}