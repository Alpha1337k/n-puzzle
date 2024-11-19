use std::{collections::{BinaryHeap, HashMap, HashSet}, rc::Rc};

use crate::{board::Board, solver::Node};


type WrappedNode = Rc<Node>;

pub struct SortedSet {
	pub sorted: BinaryHeap<WrappedNode>,
	pub store: HashMap<Board, WrappedNode>,
	deleted: HashSet<usize>,
}

impl SortedSet {
	pub fn new() -> SortedSet {
		SortedSet {
			sorted: BinaryHeap::new(),
			store: HashMap::new(),
			deleted: HashSet::new()
		}
	}

	pub fn insert(&mut self, n: &WrappedNode) {

		self.sorted.push(Rc::clone(&n));
		self.store.insert(n.board.clone(),Rc::clone(&n));
	}

	pub fn pop(&mut self) -> WrappedNode {

		let item = self.sorted.pop().unwrap();

		self.store.remove(&item.board);

		if self.deleted.remove(&item.id) {
			return self.pop();
		}

		return item;
	}

	pub fn remove(&mut self, id: usize) {
		self.deleted.insert(id);
	}

	pub fn find(&mut self, board: &Board) -> Option<&WrappedNode> {
		self.store.get(&board)
	}

	pub fn len(&self) -> usize {
		return self.store.len();
	}

	pub fn clear_1(&mut self) {
		self.deleted.clear();
		self.sorted.clear();
	}

	pub fn clear_2(&mut self) {
		let rand: Vec<&Board> = self.store.keys().collect();

		let stored = self.store.get(&rand.first().unwrap()).unwrap();

		dbg!(stored);

		self.store.clear();
	}

	pub fn sorted_len(&self) -> usize {
		return self.sorted.len();
	}

	pub fn deleted_len(&self) -> usize {
		return self.deleted.len();
	}
}