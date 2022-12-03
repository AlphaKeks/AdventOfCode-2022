use std::collections::HashSet;

fn calc_priority(char: char) -> usize {
	if char.is_lowercase() {
		char as usize - 96
	} else {
		char as usize - 38
	}
}

struct PartA;
impl PartA {
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
				// filter out all non-common letters
				let b: HashSet<char> = group[1].chars().filter(|char| a.contains(char)).collect();
				// find the remaining overlap
				let overlap = group[2].chars().find(|char| b.contains(char)).unwrap();
				calc_priority(overlap)
			})
			.sum();
	}
}

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("input.txt")?;

	let solution_a = PartA::solve(&input);
	println!("Part A: {}", solution_a);

	let solution_b = PartB::solve(&input);
	println!("Part B: {}", solution_b);

	return Ok(());
}
