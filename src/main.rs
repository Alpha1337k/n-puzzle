use std::{env, process::ExitCode};
use board::{Board};

use crate::{heuristics::manhattan_distance, solver::Solver};

mod heuristics;
mod solver;
mod board;
mod position;

fn main() -> ExitCode {
    if env::args().len() == 1 {
		println!("n-puzzle: error: no input file defined.");

		return ExitCode::from(1)
	}

	let path = env::args().nth(1).unwrap();
	let b = Board::from_path(&path).unwrap();

	manhattan_distance(&b);

	println!("{}", b);

	let mut solver = Solver::from_base(&b, &manhattan_distance);

	solver.solve();
	
	ExitCode::from(0)
}
