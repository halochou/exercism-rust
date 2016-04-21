pub fn raindrops(n :i32) -> String {
	let res = [("Pling", 3), ("Plang", 5), ("Plong", 7)].iter()
		.filter(|x| n % x.1 == 0)
		.fold("".to_string(), |acc, x| acc + x.0 );

	if res.is_empty() {
		n.to_string()
	} else {
		res
	}
}