fn to_int(c : char) -> Option<u32> {
	if c.is_numeric() {
		Some(c as u32 - 48)
	} else if c.is_alphabetic() {
		Some(c as u32 - 87)
	} else {
		None
	}
}

fn validate(s :&str) -> bool {
	s.to_string().chars().all(|c| c.is_numeric() || (c as u8 >= b'a' && c as u8 <= b'f') )
}

pub fn hex_to_int(hex_str : &str) -> Option<u32> {
	if !validate(hex_str) {
		return None;
	}
	let factors = (0u32..).map(|n| 16u32.pow(n));
	let num = hex_str.chars().rev().map(|x| to_int(x));
	let sum = num.zip(factors)
		   		 .fold(0, |acc, x| acc + x.0.unwrap() * x.1);
	Some(sum)
}