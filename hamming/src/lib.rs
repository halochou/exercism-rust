pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
	if a.len() != b.len() {
		Err("inputs of different length")
	} else {
		Ok(a.chars().zip(b.chars()).filter(|&(x, y)| x != y).count())

	}
}