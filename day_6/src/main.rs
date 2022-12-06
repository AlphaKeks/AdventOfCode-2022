#![allow(dead_code)]

fn find_unique_sequence(input: &str, window_size: usize) -> usize {
	let mut total = 0;
	for line in input.lines() {
		let chars: Vec<char> = line.chars().collect();
		total += chars
			.windows(window_size)
			.position(|window| {
				window.into_iter().enumerate().all(|(i, char)| !window[..i].contains(char))
			})
			.unwrap() + window_size;
	}
	return total;
}

struct PartA;
impl PartA {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		return find_unique_sequence(input, 4);
	}
}

struct PartB;
impl PartB {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		return find_unique_sequence(input, 14);
	}
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("input.txt")?;

	let result_a = PartA::solve(&input);
	println!("PartA: {:?}", result_a);

	let result_b = PartB::solve(&input);
	println!("PartB: {:?}", result_b);

	return Ok(());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_a() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartA::solve(&input);

		assert_eq!(39, result);

		return Ok(());
	}

	#[test]
	fn part_b() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartB::solve(&input);

		assert_eq!(120, result);

		return Ok(());
	}
}
