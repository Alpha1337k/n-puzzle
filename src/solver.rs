use std::{cell::RefCell, cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, hash::{Hash, Hasher}, rc::{Rc, Weak}};

use crate::{board::Board, position::Position};

#[derive(Clone, Debug, PartialEq, Eq)]
enum NodePosition {
	OPEN,
	CLOSED
}

#[derive(Clone, Debug)]
struct Node {
	state: NodePosition,
	board: Board,
	h: usize,
	g: usize,
	f: usize,
	parent: Option<Rc<Node>>
}

impl Node {
	pub fn get_permutations(&self, heuristic: &'static dyn Fn(&Board) -> usize) -> Vec<Node> {
		let empty_idx = self.board.data.iter().position(|v| v == &0).unwrap();

		let empty_pos = Position::from_u64(empty_idx, self.board.n);

		let mut directions: [(i32, i32); 4] = [
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

			let mut new_board = Board::with_swap(&self.board, empty_idx, swap_pos.to_usize(self.board.n));

			let score = heuristic(&new_board);

			permutations.push(Node {
				state: NodePosition::OPEN,
				board: new_board,
				h: score,
				g: self.g + 1,
				f: score + self.g + 1,
				parent: Some(Rc::from(self.clone())),
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
	open_set: BinaryHeap<Rc<Node>>,
	closed_set: Vec<Rc<Node>>,
	nodes: HashSet<Rc<Node>>
}

impl Solver {
	pub fn from_base(board: &Board, heuristic: &'static dyn Fn(&Board) -> usize) -> Solver {
		let mut solver = Solver {
			board: board.clone(),
			heuristic,
			nodes: HashSet::new(),
			closed_set: Vec::new(),
			open_set: BinaryHeap::new()
		};

		let start = Rc::new(Node {
			state: NodePosition::OPEN,
			board: solver.board.clone(),
			h: 0,
			g: 0,
			f: usize::MAX,
			parent: None
		});
		let extra = Rc::clone(&start);

		solver.open_set.push(start);
		solver.nodes.insert(extra);

		return solver;
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

		dbg!(&inversions);

		if (self.board.n & 1 == 1) {
			return inversions & 1 == 1;
		} else {
			let empty_idx = self.board.data.iter().position(|v| v == &0).unwrap();

			let pos = Position::from_u64(empty_idx, self.board.n);

			if (pos.x & 1 == 0) {
				return inversions & 1 == 0;
			}
			return inversions & 1 == 1;
		}
	}

	pub fn solve(&mut self) {
		let mut i = 0;

		if (!self.is_solvable()) {
			println!("n-puzzle: error: unsolvable");
			return;
		}

		while self.open_set.len() != 0 && i < 1000000 {

			let current = &self.open_set.pop().unwrap();
			
			if (self.heuristic)(&current.board) == 0 {
				println!("FOUND ({})!\n{}", current.f, current.board);
				break;
			}

			println!("CUR Score: {} = {} + {}:\n{}\n-----", current.f, current.g, current.h, current.board);

			for permutation in current.get_permutations(self.heuristic) {
				// println!("PERM: {} = {} + {}\n{}", permutation.f, permutation.g, permutation.h, permutation.board);

				if let Some(found_node) = self.nodes.get(&permutation) {
					if found_node.f > permutation.f && found_node.state == NodePosition::OPEN {

					}
					// println!("FOUND COPY HAHAHAH")
				} else {
					let reference = Rc::new(permutation); 
					self.nodes.insert(Rc::clone(&reference.clone()));
					self.open_set.push(reference);
				}
			}

			self.closed_set.push(Rc::clone(current));

			i += 1;
		}

		println!("Did not find. Open: {} Total: {}", self.open_set.len(), self.nodes.len());
	}
}