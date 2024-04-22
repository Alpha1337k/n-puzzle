use crate::{board::Board, position::Position};



pub fn manhattan_distance(board: &Board) -> usize {
	let mut total_offset = 0;
	
	for pos in board {
		if board[&pos] == 0 {
			continue;
		}

		let desired = board.desired_positions[board[&pos] as usize];

		let distance = usize::abs_diff(pos.x, desired.x) + 
			usize::abs_diff(pos.y, desired.y);
		// println!("Position: {} Value:{} Goal:{} = {}dist.", pos, board[&pos], board.desired_positions[board[&pos]], distance);

		total_offset += distance;
	}

	return total_offset;
}

pub fn roundtrip_manhattan_distance(board: &Board) -> usize {
	let mut total_offset = 0;

	let blank_pos = Position::from_u64(board.data.iter().position(|&r| r == 0).unwrap(), board.n);
	
	for pos in board {
		if board[&pos] == 0 {
			continue;
		}

		let desired = board.desired_positions[board[&pos] as usize];

		let mut distance = usize::abs_diff(pos.x, desired.x) + 
			usize::abs_diff(pos.y, desired.y)
			;

		if distance != 0 {
			distance += usize::abs_diff(pos.x, blank_pos.x) + 
				usize::abs_diff(pos.y, blank_pos.y)
		}

		// println!("Position: {} Value:{} Goal:{} = {}dist.", pos, board[&pos], board.desired_positions[board[&pos]], distance);

		total_offset += distance;
	}

	return total_offset;
}

pub fn euclidean_distance(board: &Board) -> usize {
	let mut total_offset = 0.0;
	
	for pos in board {
		if board[&pos] == 0 {
			continue;
		}

		let desired = board.desired_positions[board[&pos] as usize];

		let distance = f64::sqrt((
			(pos.x as i64 - desired.x as i64).pow(2) +
			(pos.y as i64 - desired.y as i64).pow(2)
		) as f64);
		// println!("Position: {} Value:{} Goal:{} = {}dist.", pos, board[&pos], board.desired_positions[board[&pos]], distance);

		total_offset += distance;
	}

	return total_offset as usize;
}

pub fn wrong_positions(board: &Board) -> usize {
	let mut total_offset = 0;
	
	for pos in board {
		if board[&pos] == 0 {
			continue;
		}

		let desired = board.desired_positions[board[&pos] as usize];

		if desired != pos {
			total_offset += 1;
		}
		// println!("Position: {} Value:{} Goal:{} = {}dist.", pos, board[&pos], board.desired_positions[board[&pos]], distance);
	}

	return total_offset as usize;
}