use std::{borrow::BorrowMut, cell::{Ref, RefCell}, cmp::Ordering, collections::{BinaryHeap, HashMap}, hash::{Hash, Hasher}, ops::Deref, rc::Rc};

use crate::{board::Board, position::Position, sorted_set::SortedSet};

#[derive(Clone, Debug, PartialEq, Eq)]
enum NodePosition {
	OPEN,
	CLOSED
}

#[derive(Clone, Debug)]
pub struct Node {
	pub state: NodePosition,
	pub board: Board,
	pub h: usize,
	pub g: usize,
	pub f: usize,
	pub parent: Option<Rc<Node>>,
	pub deleted: bool
}

impl Node {
	pub fn get_permutations(&self, heuristic: &'static dyn Fn(&Board) -> usize) -> Vec<Node> {
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
				state: NodePosition::OPEN,
				board: new_board,
				h: score,
				g: self.g + 1,
				f: score + self.g + 1,
				parent: Some(Rc::from(self.clone())),
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
	closed_set: HashMap<Board, Rc<RefCell<Node>>>
}

impl Solver {
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
			open_set: SortedSet::new()
		};

		solver.open_set.insert(&Rc::new(RefCell::new(Node {
			state: NodePosition::OPEN,
			board: solver.board.clone(),
			h: 0,
			g: 0,
			f: usize::MAX,
			parent: None,
			deleted: false
		})));

		return solver;
	}

	pub fn solve(&mut self) {
		let mut i = 0;

		if !self.is_solvable() {
			println!("n-puzzle: error: unsolvable");
			return;
		}

		while self.open_set.len() != 0 && i < 30605822 {

			let current_ref = &self.open_set.pop();
			let current = current_ref.borrow();
 			
			// println!("\n----- Step #{}\t Score: {} = {} + {}:\n{}", i, current.f, current.g, current.h, current.board);

			if (self.heuristic)(&current.board) == 0 {
				println!("FOUND ({})!\n{}", current.f, current.board);
				break;
			}

			if (i % 1_000_000 == 0) {
				println!("{}, open: {}, closed: {}", i, self.open_set.len(), self.closed_set.len());
			}

			let mut inserted = 0;
			let mut needs_insert = false;

			for permutation in current.get_permutations(self.heuristic) {
				// println!("PERM: {} = {} + {}\n{}", permutation.f, permutation.g, permutation.h, permutation.board);

				// check for duplicates
				if let Some(found_node) = self.closed_set.remove(&permutation.board) {
					if permutation.f < found_node.borrow().f {
						self.open_set.insert(&Rc::clone(&found_node));

						inserted += 1;		
					}
					self.closed_set.insert(permutation.board.clone(), found_node);

				} else if let Some(found_node) = self.open_set.find(&permutation.board) {
					if permutation.f < found_node.borrow().f {
						let mut f = (**found_node).borrow_mut();

						f.deleted = true;

						needs_insert = true;
						inserted += 1;		
					}
				} else {
					needs_insert = true;
				}
				if (needs_insert) {
					inserted += 1;
					self.open_set.insert(&Rc::new(RefCell::new(permutation)));
				}
			}

			self.closed_set.insert(current.board.clone(), Rc::clone(current_ref));

			// println!("Inserted: {}", inserted);

			i += 1;
		}

		println!("Summary: Open: {} Closed: {} Total: {}", self.open_set.len(), self.closed_set.len(), self.closed_set.len() + self.open_set.len());
	}
}