use std::collections::HashMap;

pub fn nucleotide_counts(seq: &str) -> HashMap<char, usize>{
	let mut map = HashMap::new();
	map.insert('A', 0);
	map.insert('T', 0);
	map.insert('C', 0);
	map.insert('G', 0);
	
	for c in seq.chars() {
		let counter = map.entry(c).or_insert(1);
		*counter += 1;
	}
	map
}

pub fn count(c: char, seq: &str) -> usize {
	seq.chars().filter(|&x| x == c).count()
}