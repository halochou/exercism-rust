use std::collections::HashMap;

pub struct Condons (HashMap<String, String>);

pub fn parse(pairs : Vec<(&'static str, &'static str)>) -> Condons {
	let mut info = HashMap::new();
	for pair in pairs {
		info.insert(pair.0.to_string(), pair.1.to_string());
	}
	Condons(info)
}

impl Condons {
	pub fn name_for(&self, shorthand: &str) -> Result<&str, &str> {
		let mut dict = HashMap::new();
		dict.insert('W','A');
		dict.insert('S','C');
		dict.insert('M','A');
		dict.insert('K','G');
		dict.insert('R','A');
		dict.insert('Y','C');
		dict.insert('B','C');
		dict.insert('D','A');
		dict.insert('H','A');
		dict.insert('V','A');
		dict.insert('N','A');

		let s : String = shorthand.chars()
								.map(|c| if let Some(&v) = dict.get(&c) {v} else {c} ).collect();

		if let Some(res) = self.0.get(&s){
			Ok(res)
		} else {
			Err("err")
		}
	}
}
