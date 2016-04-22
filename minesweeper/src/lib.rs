pub fn annotate(board: &Vec<&str>) -> Vec<String> {
	let mut newboard : Vec<Vec<char>> = board.iter().cloned().map(|r|r.chars().collect()).collect();

	for (i, row) in board.iter().enumerate() {
		for (j, ele) in row.chars().enumerate() {
			if ele == '*' {
				for dx in -1..2 {
					for dy in -1..2 {
						if let (Some(x), Some(y)) = (i.checked_add(dx), j.checked_add(dy)){
							let counter = newboard[x][y];
							newboard[x][y] = if counter == ' ' {'1'} else {(counter as u8 + 1) as char};
						}
					}
				}
			}
		}
	}

	newboard.iter().map(|r| r.iter().cloned().collect()).collect()
}