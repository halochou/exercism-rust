#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid(String,);

impl RibonucleicAcid {
	pub fn new<'a>(from: &'a str) -> RibonucleicAcid {
		RibonucleicAcid(from.to_string(),)
	}
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid(String,);

impl DeoxyribonucleicAcid {
	pub fn new(from: &str) -> DeoxyribonucleicAcid {
		DeoxyribonucleicAcid(from.to_string(),)
	}

	pub fn to_rna(&self) -> RibonucleicAcid {
		let rna: String = self.0.chars()
			.map(|c| {
				match c {
					'G' => 'C',
					'C' => 'G',
					'T' => 'A',
					'A' => 'U',
					_ => ' '
				}
			})
			.collect();
		RibonucleicAcid::new(&rna)
	}
}

