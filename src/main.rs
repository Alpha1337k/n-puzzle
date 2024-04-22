use std::{cell::RefCell, env, process::ExitCode, rc::Rc};
use board::{Board};


use crate::{heuristics::manhattan_distance, solver::{Node, Solver}};

pub mod sorted_set;
pub mod heuristics;
pub mod solver;
pub mod board;
pub mod position;

pub fn main() -> ExitCode {
    if env::args().len() == 1 {
		println!("n-puzzle: error: no input file defined.");

		return ExitCode::from(1)
	}

	let path = env::args().nth(1).unwrap();
	let b = Board::from_path(&path).unwrap();

	// for i in 0..b.desired_positions.len() {
	// 	println!("{}={}", i, b.desired_positions[i]);
	// }

	let mut solver = Solver::from_base(&b, &manhattan_distance);

	solver.solve();
	
	ExitCode::from(0)
}
