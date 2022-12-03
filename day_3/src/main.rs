#![allow(dead_code)]

use std::collections::HashSet;

fn calc_priority(char: char) -> usize {
	if char.is_uppercase() {
		char as usize - 38
	} else {
		char as usize - 96
	}
}

struct PartA;
impl PartA {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		return input
			.lines()
			.map(|rucksack| {
				let (left, right) = rucksack.split_at(rucksack.len() / 2);
				let left: HashSet<char> = left.chars().collect();
				// find the overlap
				let overlap = right.chars().find(|char| left.contains(&char)).unwrap();
				calc_priority(overlap)
			})
			.sum();
	}
}

struct PartB;
impl PartB {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut groups: Vec<[&str; 3]> = Vec::new();
		let mut current = ["", "", ""];

		for (i, line) in input.lines().enumerate() {
			let i = i % 3;
			if i == 2 {
				current[i] = line;
				groups.push(current);
				current = ["", "", ""];
				continue;
			}
			current[i] = line;
		}

		return groups
			.into_iter()
			.map(|[a, b, c]| {
				// first rucksack
				let a: HashSet<char> = a.chars().collect();
				// filter out non-common letters
				let b: HashSet<char> = b.chars().filter(|char| a.contains(char)).collect();
				// find the remaining overlap
				let overlap = c.chars().find(|char| b.contains(char)).unwrap();
				calc_priority(overlap)
			})
			.sum();
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

		assert_eq!(157, result);

		return Ok(());
	}

	#[test]
	fn part_b() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartB::solve(&input);

		assert_eq!(70, result);

		return Ok(());
	}
}
