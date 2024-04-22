use std::{process::ExitCode};
use board::{Board};
use heuristics::{euclidean_distance, manhattan_distance, roundtrip_manhattan_distance};


use crate::{solver::{Solver}};

pub mod sorted_set;
pub mod heuristics;
pub mod solver;
pub mod board;
pub mod position;

use clap::Parser;

/// Solve n-puzzles of any size.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use manhattan as heuristic.
    #[arg(short, long)]
    manhattan: bool,

    /// Use euclidean as heuristic.
    #[arg(short, long)]
    euclidean: bool,

    /// Use roundtrip as heuristic.
    #[arg(short, long)]
    roundtrip: bool,

	// Input file.
	#[arg(value_name = "FILE")]
	path: String
}

fn get_heuristic_func(args: &Args) -> &'static dyn Fn(&Board) -> usize {
	if args.manhattan {
		return &manhattan_distance;
	} else if args.euclidean {
		return &euclidean_distance;
	} else if args.roundtrip {
		return &roundtrip_manhattan_distance;
	} else {
		return &manhattan_distance;
	}
}

fn main() -> ExitCode {
	let args = Args::parse();

	// let path = env::args().nth(1).unwrap();
	let b = Board::from_path(&args.path).unwrap();
	let heuristic = get_heuristic_func(&args);
	// for i in 0..b.desired_positions.len() {
	// 	println!("{}={}", i, b.desired_positions[i]);
	// }

	let mut solver = Solver::from_base(&b, heuristic);

	solver.solve();
	
	ExitCode::from(0)
}
