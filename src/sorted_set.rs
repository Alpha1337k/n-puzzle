use std::{borrow::Borrow, cell::{Cell, RefCell}, collections::{BinaryHeap, HashMap}, rc::Rc};

use crate::{board::Board, solver::Node};


type WrappedNode = Rc<RefCell<Node>>;

pub struct SortedSet {
	sorted: BinaryHeap<WrappedNode>,
	store: HashMap<String, WrappedNode>
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
		self.store.insert(n.borrow_mut().board.to_string(),Rc::clone(&n));
	}

	pub fn pop(&mut self) -> WrappedNode {

		let item = self.sorted.pop().unwrap();

		self.store.remove(&item.borrow_mut().board.to_string());

		if item.borrow_mut().deleted == true {
			return self.pop();
		}

		return item;
	}

	pub fn find(&mut self, board: &Board) -> Option<&mut WrappedNode> {
		self.store.get_mut(&board.to_string())
	}

	pub fn len(&self) -> usize {
		return self.store.len();
	}
}