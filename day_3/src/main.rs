fn main() -> anyhow::Result<()> {
	const INPUT: &str = include_str!("../input.txt");

	let solution_a = PartA::solve(INPUT);
	println!("Part A: {}", solution_a);

	let solution_b = PartB::solve(INPUT);
	println!("Part B: {}", solution_b);

	return Ok(());
}

use std::collections::HashSet;

fn get_priority(char: char) -> usize {
	if char.is_lowercase() {
		return char as usize - 96;
	} else {
		return char as usize - 38;
	}
}

struct PartA;
impl PartA {
	pub fn solve(input: &str) -> usize {
		return input
			.lines()
			.map(|rucksack| rucksack.split_at(rucksack.len() / 2))
			.map(|(left, right)| {
				let left: HashSet<char> = left.chars().collect();
				right.chars().find(|char| left.contains(&char)).unwrap()
			})
			.map(get_priority)
			.sum();
	}
}

struct PartB;
impl PartB {
	pub fn solve(input: &str) -> usize {
		let mut groups: Vec<[&str; 3]> = Vec::new();
		let mut current = ["", "", ""];

		// divide all rucksacks into groups of 3
		for (i, line) in input.lines().enumerate() {
			if i % 3 == 2 {
				current[i % 3] = line;
				groups.push(current);
				current = ["", "", ""];
			}
			current[i % 3] = line;
		}
		drop(current);

		return groups
			.into_iter()
			.map(|group| {
				// first rucksack
				let a: HashSet<char> = group[0].chars().collect();
				// filter out all the non-common letters
				let b: HashSet<char> = group[1].chars().filter(|char| a.contains(char)).collect();
				// find the remaining overlap
				group[2].chars().find(|char| b.contains(char)).unwrap()
			})
			.map(get_priority)
			.sum();
	}
}
