#![allow(dead_code)]

fn line_to_ranges(
	line: &str,
) -> (std::ops::RangeInclusive<usize>, std::ops::RangeInclusive<usize>) {
	let (left, right) = line.split_once(",").unwrap();
	let left = {
		let (start, end) = left.split_once("-").unwrap();
		start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
	};
	let right = {
		let (start, end) = right.split_once("-").unwrap();
		start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
	};
	return (left, right);
}

struct PartA;
impl PartA {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut count = 0;
		for line in input.lines() {
			let (left, right) = line_to_ranges(line);

			if left.contains(&right.start()) && left.contains(&right.end())
				|| right.contains(&left.start()) && right.contains(&left.end())
			{
				count += 1;
			}
		}
		return count;
	}
}

struct PartB;
impl PartB {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut count = 0;
		for (i, line) in input.lines().enumerate() {
			let (left, right) = line_to_ranges(line);

			if (left.start() < right.start() && left.end() < right.start())
				|| (right.start() < left.start() && right.end() < left.start())
			{
				continue;
			}

			if left == right
				|| left.start() == right.start()
				|| left.end() == right.end()
				|| left.end() <= right.start()
				|| left.end() <= right.end()
				|| right.end() <= left.start()
				|| right.end() <= left.end()
			{
				count += 1;
			}
		}
		return count;
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

		assert_eq!(2, result);

		return Ok(());
	}

	#[test]
	fn part_b() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartB::solve(&input);

		assert_eq!(4, result);

		return Ok(());
	}
}
