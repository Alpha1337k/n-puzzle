use std::{cell::RefCell, collections::{BinaryHeap, HashMap}, rc::Rc};

use crate::{board::Board, solver::Node};


type WrappedNode = Rc<RefCell<Node>>;

pub struct SortedSet {
	pub sorted: BinaryHeap<WrappedNode>,
	pub store: HashMap<Board, WrappedNode>
}

impl SortedSet {
	pub fn new() -> SortedSet {
		SortedSet {
			sorted: BinaryHeap::new(),
			store: HashMap::new()
		}
	}

	pub fn insert(&mut self, n: &WrappedNode) {

		self.sorted.push(Rc::clone(&n));
		self.store.insert(n.borrow_mut().board.clone(),Rc::clone(&n));
	}

	pub fn pop(&mut self) -> WrappedNode {

		let item = self.sorted.pop().unwrap();

		self.store.remove(&item.borrow_mut().board);

		if item.borrow_mut().deleted == true {
			return self.pop();
		}

		return item;
	}

	pub fn find(&mut self, board: &Board) -> Option<&mut WrappedNode> {
		self.store.get_mut(&board)
	}

	pub fn len(&self) -> usize {
		return self.store.len();
	}

	pub fn sorted_len(&self) -> usize {
		return self.sorted.len();
	}
}