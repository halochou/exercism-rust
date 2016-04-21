use std::collections::HashMap;

pub fn annotate(board: &Vec<&str>) -> Vec<String> {
	let mine_pos = get_mine_pos(board);
	mine_pos.iter().fold(HashMap::new(), |acc, &(i,j)| {
		if let Some(x) = i.checked_sub

	})

}

fn get_mine_pos(board: &Vec<&str>) -> Vec<(usize,usize)> {
	let mut res = Vec<_>::new();
	for (i, row) in board.iter().enumerate() {
		for (j, ele) in row.chars() {
			if ele == '*' {
				res.push((i,j));
			}
		}
	}
	res
}

fn hashmap_to_board() {

}