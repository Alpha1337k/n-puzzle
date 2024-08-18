use std::{collections::HashSet, fmt, fs, hash::{Hash, Hasher}, rc::Rc};

use anyhow::{Error, Result};

use crate::position::Position;


#[derive(Debug, Clone)]
pub struct Board {
	pub data: Vec<u8>,
	pub n: usize,
	pub desired_positions: Rc<Vec<Position>>
}

impl Board {

	fn create_desired_positions(n: usize) -> Vec<Position> {
		let mut rval = Vec::with_capacity(n * n + 1);
		let mut idx = 0;

		// dummy
		rval.push(Position {
			x: 100,
			y: 100
		});

		let mut visited_places = HashSet::<Position>::new();

		let directions: [(i32, i32); 4] = [
			(1, 0),
			(0, 1),
			(-1, 0),
			(0, -1)
		];
		let mut pos = Position {
			x: 0,
			y: 0
		};

		let mut dir_idx = 0;
		let mut current_dir = &directions[dir_idx];

		while idx < n * n {
			visited_places.insert(pos);
			rval.push(pos);

			pos.x = pos.x.wrapping_add(current_dir.0 as usize);
			pos.y = pos.y.wrapping_add(current_dir.1 as usize);
			
			if	pos.x.wrapping_add(current_dir.0 as usize) >= n ||
				pos.y.wrapping_add(current_dir.1 as usize) >= n ||
			 visited_places.contains(&Position {
				x: pos.x.wrapping_add(current_dir.0 as usize),
				y: pos.y.wrapping_add(current_dir.1 as usize)
			 }) {
				dir_idx = (dir_idx + 1) % 4;
				current_dir = &directions[dir_idx];
			 }
			idx += 1;
		}

		rval.swap(0, n * n);
		rval.pop();

		rval
	}
	
	pub fn from_path(path: &str) -> Result<Board> {
		let binding = fs::read_to_string(path)?;
		let lines: Vec<&str> = binding.split('\n').collect();

		let mut i = 0;

		while i < lines.len() && lines[i].starts_with('#') {
			i += 1;
		}

		if i == lines.len() {
			return Err(Error::msg("Could not find size."));
		}

		let n = lines[i].parse::<usize>()?;

		i += 1;

		let mut board = Board{
			data: Vec::with_capacity(n * n),
			n,
			desired_positions: Rc::new(Self::create_desired_positions(n))
		};

		while i < lines.len() {
			let splitted: Vec<&str> = lines[i].split('#').collect();

			if splitted.first().is_none() {
				i += 1;
				continue;
			}

			let no_comments = splitted.first().unwrap();

			if no_comments.len() == 0 {
				break;
			}

			let mut numbers: Vec<u8> = no_comments
				.split(' ').filter(|f| f != &"")
				.map(|v| v.parse::<u8>().unwrap()).collect();

			if numbers.len() != n {

				return Err(Error::msg(format!("n-puzzle: error: line length mismatch {} vs {}", numbers.len(), n)))
			}
			board.data.append(&mut numbers);

			i += 1;
		}

		if n * n != board.data.len() {
			Err(Error::msg("InvalidSize"))
		} else {
			Ok(board)
		}

	}

	pub fn with_swap(board: &Board, a: usize, b: usize) -> Board {
		let mut board = Board{
			data: board.data.clone(),
			n: board.n,
			desired_positions: board.desired_positions.clone()
		};

		board.data.swap(a, b);

		return board;
	}
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..self.data.len() {
			if self.data[i] != 0 {
				write!(f, "{:2} ", self.data[i]).unwrap();
			} else {
				write!(f, " - ").unwrap();
			}

			if i % self.n == self.n - 1 && i != self.data.len() - 1 {
				write!(f, "\n").unwrap();
			}
		}

		Ok(())
    }
}

pub struct BoardIterator {
	index: usize,
	n: usize
}

impl<'a> IntoIterator for &'a Board {
    type Item = Position;
    type IntoIter = BoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            index: 0,
			n: self.n
        }
    }
}

impl std::ops::Index<&Position> for Board {
    type Output = u8;

    fn index(&self, idx: &Position) -> &u8 {
		return &self.data[idx.x + idx.y * self.n];
    }
}

impl std::ops::IndexMut<&Position> for Board {
    fn index_mut(&mut self, idx: &Position) -> &mut u8 {
		return &mut self.data[idx.x + idx.y * self.n];
    }
}

impl<'a> Iterator for BoardIterator {
	type Item = Position;

	fn next(&mut self) -> Option<Self::Item> {
		self.index += 1;

		if self.index >= (self.n * self.n) + 1 {
			return None;			
		}

		return Some(Position::from_u64(self.index - 1, self.n));
	}
}

impl Hash for Board {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
		(&self.data).hash(state);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for Board {}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;

	#[test]
	#[should_panic]
	fn read_unknown() {
		let _ = Board::from_path("dna.txt");
	}
}