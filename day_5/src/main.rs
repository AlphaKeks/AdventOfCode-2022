#![allow(dead_code)]

fn parse_crates(input: &str) -> (Vec<Vec<char>>, String) {
	let n_rows = input.split_once("\n").map(|(h, _)| (h.len() + 1) / 4).unwrap();

	let (lines, instructions) = input
		.split_once("\n\n")
		.map(|(crates, instructions)| {
			(crates.replace("[", "  ").replace("]", " "), instructions.to_owned())
		})
		.unwrap();

	let mut crates: Vec<Vec<char>> = (0..n_rows).map(|_| Vec::new()).collect();

	let mut current_index = 0;
	let mut whitespaces = 0;

	for line in lines.lines().rev().skip(1) {
		for char in line.chars() {
			if char.is_whitespace() {
				whitespaces += 1;
				if whitespaces != 0 && whitespaces % 5 == 0 {
					current_index += 1;
					whitespaces = 0;
				}
			} else if char.is_alphabetic() {
				crates[current_index].push(char);
				current_index += 1;
				whitespaces = 0;
			}
		}
		current_index = 0;
		whitespaces = 0;
	}

	return (crates, instructions);
}

struct PartA;
impl PartA {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> String {
		let (mut crates, instructions) = parse_crates(input);

		let instructions = instructions.lines().map(|ln| {
			ln.replace("move ", "")
				.replace(" from", "")
				.replace(" to", "")
				.split(" ")
				.collect::<Vec<_>>()
				.into_iter()
				.map(|i| i.parse::<usize>().unwrap())
				.collect::<Vec<_>>()
		});

		for instruction in instructions {
			let (count, from, to) =
				(instruction[0].clone(), instruction[1].clone(), instruction[2].clone());

			for i in 0..count {
				let temp = crates[from - 1].pop().unwrap();
				crates[to - 1].push(temp);
			}
		}

		let result = crates.iter_mut().map(|c| c.pop().unwrap());

		return String::from_iter(result);
	}
}

struct PartB;
impl PartB {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> String {
		let (mut crates, instructions) = parse_crates(input);

		let instructions = instructions.lines().map(|ln| {
			ln.replace("move ", "")
				.replace(" from", "")
				.replace(" to", "")
				.split(" ")
				.collect::<Vec<_>>()
				.into_iter()
				.map(|i| i.parse::<usize>().unwrap())
				.collect::<Vec<_>>()
		});

		for instruction in instructions {
			let (count, from, to) =
				(instruction[0].clone(), instruction[1].clone() - 1, instruction[2].clone() - 1);

			let range = crates[from].len() - count..crates[from].len();
			let mut elements: Vec<char> = crates[from].drain(range).collect();

			crates[to].append(&mut elements);
		}

		let result = crates.iter_mut().map(|c| c.pop().unwrap());
		return String::from_iter(result);
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

		assert_eq!(String::from("CMZ"), result);

		return Ok(());
	}

	#[test]
	fn part_b() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartB::solve(&input);

		assert_eq!(String::from("MCD"), result);

		return Ok(());
	}
}
