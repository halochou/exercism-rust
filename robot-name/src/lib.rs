extern crate rand;

#[derive(Debug)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Robot {
		use rand::{thread_rng, Rng};
		let mut rng = thread_rng();

		let letters : Vec<u8> = (b'A'..b'Z'+1).collect();
		let letter_fst = *rng.choose(&letters).unwrap() as char;
		let letter_snd = *rng.choose(&letters).unwrap() as char;
		let sn = rng.gen_range(0, 1000);

		let name = format!("{}{}{:03}", letter_fst, letter_snd, sn);
		Robot { name: name}
    }

    pub fn name<'a>(&'a self) -> &'a str {
    	&self.name
    }

    pub fn reset_name(&mut self) {
    	*self = Robot::new();
    }
}