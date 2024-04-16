use crate::board::Board;


pub struct Solver {
	board: Board,
	depth: usize,
	score: usize,
	heurisitic: &'static dyn Fn(&Board) -> usize
}

impl Solver {
	pub fn from_base(board: &Board, heuristic: &'static dyn Fn(&Board) -> usize) -> Solver {
		Solver {
			board: board.clone(),
			depth: 0,
			score: heuristic(board),
			heurisitic: heuristic
		}
	}

	pub fn solve() {
		
	}
}