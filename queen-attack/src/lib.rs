#[derive(Debug)]
pub struct Queen(i32,i32);

impl Queen {
	pub fn new((x, y) : (i32, i32)) -> Result<Queen, String>{
		match (x, y) {
			_ if x >= 0 && x < 8 && y >= 0 && y < 8 => Ok(Queen(x, y)),
			_ => Err("Error".to_string())
		}
	}

	pub fn can_attack(&self, enemy : &Queen) -> bool {
		self.0 == enemy.0 || self.1 == enemy.1 || (self.0 - enemy.0).abs() == (self.1 - enemy.1).abs()
	}
}