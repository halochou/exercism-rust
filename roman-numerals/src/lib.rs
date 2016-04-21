pub struct Roman;

fn gen_thousand(n: i32) -> String {
	match n / 1000 {
		3 => "MMM".to_string(),
		2 => "MM".to_string(),
		1 => "M".to_string(),
		_ => "".to_string()
	}
}

fn gen_hundred(n: i32)-> String {
	match n % 1000 / 100 {
		9 => "CM".to_string(),
		8 => "DCCC".to_string(),
		7 => "DCC".to_string(),
		6 => "DC".to_string(),
		5 => "D".to_string(),
		4 => "CD".to_string(),
		3 => "CCC".to_string(),
		2 => "CC".to_string(),
		1 => "C".to_string(),
		_ => "".to_string(),
	}
}

fn gen_dec(n: i32) -> String {
	match n % 100 / 10 {
		9 => "XC".to_string(),
		8 => "LXXX".to_string(),
		7 => "LXX".to_string(),
		6 => "LX".to_string(),
		5 => "L".to_string(),
		4 => "XL".to_string(),
		3 => "XXX".to_string(),
		2 => "XX".to_string(),
		1 => "X".to_string(),
		_ => "".to_string()
	}
}

fn gen_digit(n: i32) -> String {
	match n % 10 {
		9 => "IX".to_string(),
		8 => "VIII".to_string(),
		7 => "VII".to_string(),
		6 => "VI".to_string(),
		5 => "V".to_string(),
		4 => "IV".to_string(),
		3 => "III".to_string(),
		2 => "II".to_string(),
		1 => "I".to_string(),
		_ => "".to_string()
	}
}

impl Roman {
	pub fn from(n: i32) -> String {
		gen_thousand(n) + &gen_hundred(n) + &gen_dec(n) + &gen_digit(n)
	}


}