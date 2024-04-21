use crate::board::Board;



pub fn manhattan_distance(board: &Board) -> usize {
	let mut total_offset = 0;
	
	for pos in board {
		if board[&pos] == 0 {
			continue;
		}

		let desired = board.desired_positions[board[&pos]];

		let distance = usize::abs_diff(pos.x, desired.x) + 
			usize::abs_diff(pos.y, desired.y);
		// println!("Position: {} Value:{} Goal:{} = {}dist.", pos, board[&pos], board.desired_positions[board[&pos]], distance);

		total_offset += distance;
	}

	return total_offset;
}