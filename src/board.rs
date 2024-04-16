use std::fs;


#[derive(Debug)]
pub struct Board {
	data: Vec<usize>,
}

impl Board {
	
	pub fn from_path(path: &str) -> Result<Board, ()> {
		let binding = fs::read_to_string(path)
			.expect("n-puzzle: error: could not open file.");
		let lines: Vec<&str> = binding.split('\n').collect();

		let mut i = 0;

		while lines[i].starts_with('#') {
			i += 1;
		}

		let n = lines[i].parse::<usize>().expect("n-puzzle: error: could not parse size");

		i += 1;

		let mut board = Board{
			data: Vec::with_capacity(n * n)
		};

		while i < lines.len() {
			let splitted: Vec<&str> = lines[i].split('#').collect();

			if (splitted.first().is_none()) {
				i += 1;
				continue;
			}

			let no_comments = splitted.first().unwrap();

			if (no_comments.len() == 0) {
				break;
			}

			let mut numbers: Vec<usize> = no_comments
				.split(' ').filter(|f| f != &"")
				.map(|v| v.parse::<usize>().unwrap()).collect();

			dbg!(&numbers);

			if numbers.len() != n {
				println!("n-puzzle: error: line length mismatch {} vs {}", numbers.len(), n);

				return Err(())
			}
			board.data.append(&mut numbers);

			i += 1;
		}

		Ok(board)
	}
}