use std::{cell::RefCell, cmp::Ordering, collections::HashMap, hash::{Hash, Hasher}, rc::Rc, time::SystemTime};

use anyhow::{Error, Ok, Result};
use num_format::{Locale, ToFormattedString};

use crate::{board::Board, position::Position, sorted_set::SortedSet};

#[derive(Clone, Debug)]
pub struct Node {
	pub board: Board,
	pub h: usize,
	pub g: usize,
	pub f: usize,
	pub parent: Option<Rc<RefCell<Node>>>,
	pub deleted: bool
}

impl Node {
	pub fn get_permutations(&self, heuristic: &'static dyn Fn(&Board) -> usize, parent: &Rc<RefCell<Node>>) -> Vec<Node> {
		let empty_idx = self.board.data.iter().position(|v| v == &0).unwrap();

		let empty_pos = Position::from_u64(empty_idx, self.board.n);

		let directions: [(i32, i32); 4] = [
			(1, 0),
			(0, 1),
			(-1, 0),
			(0, -1)
		];

		let mut permutations: Vec<Node> = Vec::with_capacity(4);

		for direction in directions {
			let swap_pos = Position {
				x: empty_pos.x.wrapping_add(direction.0 as usize),
				y: empty_pos.y.wrapping_add(direction.1 as usize)
			};

			if	swap_pos.x >= self.board.n ||
				swap_pos.y >= self.board.n {
					continue;
			}

			let new_board = Board::with_swap(&self.board, empty_idx, swap_pos.to_usize(self.board.n));

			let score = heuristic(&new_board);

			permutations.push(Node {
				board: new_board,
				h: score,
				g: self.g + 1,
				f: score + self.g + 1,
				parent: Some(Rc::clone(parent)),
				deleted: false
			})
		}

		return permutations;
	}
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.f.cmp(&self.f)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Node {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
		(&self.board).hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Eq for Node {}

pub struct Solver {
	board: Board,
	heuristic: &'static dyn Fn(&Board) -> usize,
	open_set: SortedSet,
	closed_set: HashMap<Board, Rc<RefCell<Node>>>,
	timer: SystemTime,
	eval_count: usize,
	max_total_states: usize,
}

impl Solver {
	fn print_progress(&self, i: usize) {
		println!("\x1b[1A{esc};[2K;\rindex: {:>12} | open: {:>12} | closed: {:>12}",
			i.to_formatted_string(&Locale::en),
			self.open_set.len().to_formatted_string(&Locale::en), 
			self.closed_set.len().to_formatted_string(&Locale::en),
			esc = 0x27 as char, 
		);
	}

	fn rewind_steps(&self, solution: Rc<RefCell<Node>>) -> Vec<Node> {
		let mut nodes = Vec::new();
		let mut iter = solution;
		loop {
			nodes.insert(0, iter.borrow().clone());

			let parent = &iter.as_ref().clone();

			match &parent.borrow().parent {
				Some(v) => iter = v.clone(),
				None => break 
			};
		}

		nodes
	}
	fn print_result(&self, solution: Rc<RefCell<Node>>) {
		println!("----- Results -----");
		println!("States evalled:       {:>12}", self.eval_count.to_formatted_string(&Locale::en));
		println!("Max states in memory: {:>12}", self.max_total_states.to_formatted_string(&Locale::en));
		println!("X moves needed:       {:>12}", solution.borrow().g.to_formatted_string(&Locale::en));
		println!("Visualization: \n");

		let steps = self.rewind_steps(solution);
		
		for step in steps {
			println!("f({}) = h({}) + g({})\n{}\n", step.f, step.g, step.h, step.board);
		}
	
	}

	fn get_inversion_count(&self) -> i32 {
		let mut count = 0;

		for i in 0..((self.board.n * self.board.n) - 1) {
			for j in (i + 1)..(self.board.n * self.board.n) {
				if self.board.data[i] != 0 && self.board.data[j] != 0 && 
					self.board.data[i] > self.board.data[j] {
						count += 1;
					}
			}
		}

		count
	}

	pub fn is_solvable(&self) -> bool {
		let inversions = self.get_inversion_count();

		if self.board.n & 1 == 1 {
			return inversions & 1 == 1;
		} else {
			let empty_idx = self.board.data.iter().position(|v| v == &0).unwrap();

			let pos = Position::from_u64(empty_idx, self.board.n);

			if pos.x & 1 == 0 {
				return inversions & 1 == 0;
			}
			return inversions & 1 == 1;
		}
	}

	pub fn from_base(board: &Board, heuristic: &'static dyn Fn(&Board) -> usize) -> Solver {
		let mut solver = Solver {
			board: board.clone(),
			heuristic,
			closed_set: HashMap::new(),
			open_set: SortedSet::new(),
			timer: SystemTime::now(),
			eval_count: 0,
			max_total_states: 0,
		};

		solver.open_set.insert(&Rc::new(RefCell::new(Node {
			board: solver.board.clone(),
			h: 0,
			g: 0,
			f: usize::MAX,
			parent: None,
			deleted: false
		})));

		return solver;
	}

	pub fn solve(&mut self) -> Result<()> {
		if !self.is_solvable() {
			return Err(Error::msg("n-puzzle: error: unsolvable"));
		}

		println!();

		let mut result: Option<Rc<RefCell<Node>>> = None;

		self.timer = SystemTime::now();

		while self.open_set.len() != 0 {

			let current_ref = &self.open_set.pop();
			let current = current_ref.borrow();
 			
			// println!("\n----- Step #{}\t Score: {} = {} + {}:\n{}", i, current.f, current.g, current.h, current.board);

			if (self.heuristic)(&current.board) == 0 {
				result = Some(current_ref.clone());
				break;
			}

			if self.eval_count % 10_000 == 0 && self.timer.elapsed().unwrap().as_millis() >= 500 {
				self.print_progress(self.eval_count);
				self.timer = SystemTime::now();
			}

			let mut needs_insert = false;

			for permutation in current.get_permutations(self.heuristic, current_ref) {
				// println!("PERM: {} = {} + {}\n{}", permutation.f, permutation.g, permutation.h, permutation.board);

				// check for duplicates
				if let Some(found_node) = self.closed_set.remove(&permutation.board) {
					if permutation.f < found_node.borrow().f {
						self.open_set.insert(&Rc::clone(&found_node));
					} else {
						self.closed_set.insert(permutation.board.clone(), found_node);
					}

				} else if let Some(found_node) = self.open_set.find(&permutation.board) {
					if permutation.f < found_node.borrow().f {
						let mut f = (**found_node).borrow_mut();

						f.deleted = true;
						needs_insert = true;
					}
				} else {
					needs_insert = true;
				}

				if needs_insert {
					self.open_set.insert(&Rc::new(RefCell::new(permutation)));
				}
			}

			self.max_total_states = usize::max(self.max_total_states, self.open_set.len() + self.closed_set.len());

			self.closed_set.insert(current.board.clone(), Rc::clone(current_ref));

			self.eval_count += 1;
		}

		print!("\x1b[1A\x1b[2K\r");
		self.print_result(result.unwrap());
	
		Ok(())
	}
}