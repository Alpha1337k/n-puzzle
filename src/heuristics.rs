use crate::board::Board;



pub fn manhattan_distance(board: &Board) -> usize {
	let mut total_offset = 0;
	
	for pos in board {
		let desired = board.desired_positions[board[&pos]];

		let distance = usize::abs_diff(pos.x, desired.x) + 
			usize::abs_diff(pos.y, desired.y);
		// println!("{} V:{} D:{} = {}", pos, board[&pos], board.desired_positions[board[&pos]], distance);

		total_offset += distance;
	}

	return total_offset;
}