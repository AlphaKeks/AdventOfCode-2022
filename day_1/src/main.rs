#![allow(dead_code)]

fn calc_calories(input: &str) -> Vec<usize> {
	return input
		.trim() // just to be sure
		.split("\n\n") // split into elves
		.map(|elf| {
			elf.lines() // iterate over each food item
				.filter_map(|calories| calories.parse::<usize>().ok()) // parse calories into numbers
				.sum() // sum all calories together
		})
		.collect::<Vec<usize>>();
}

struct PartA;
impl PartA {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		return calc_calories(input).into_iter().max().unwrap_or(0); // get the elf with the highest total calorie count
	}
}

struct PartB;
impl PartB {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut all_calories = calc_calories(input);
		all_calories.sort_by(|a, b| b.cmp(a)); // sort in descending order
		return all_calories.into_iter().take(3).sum(); // return top 3 calorie
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

		assert_eq!(24000, result);

		return Ok(());
	}

	#[test]
	fn part_b() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartB::solve(&input);

		assert_eq!(45000, result);

		return Ok(());
	}
}
