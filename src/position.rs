use std::fmt;



#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
	pub x: usize,
	pub y: usize,
}

impl Position {
	pub fn from_u64(pos: usize, n: usize) -> Position {
		return Position {
			x: pos % n,
			y: pos.div_euclid(n)
		};
	}

	pub fn to_usize(&self, n: usize) -> usize {
		return self.y * n + self.x;
	}
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}-{}", self.x, self.y)
    }
}