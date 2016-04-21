#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}

pub struct Allergies(i32,);

impl Allergies {
	pub fn new(n: i32) -> Allergies{
		Allergies(n,)
	}

	pub fn is_allergic_to(&self, sth : Allergen) -> bool{
		self.0 & sth as i32 != 0 
	}

	pub fn allergies(&self) -> Vec<Allergen> {
		use self::Allergen::*;
		let total = vec![Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate,Pollen, Cats];
		total.into_iter()
			.filter(|&x| x as i32 & self.0 != 0 )
			.collect()
	}
}

