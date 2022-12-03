#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl std::str::FromStr for Move {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use crate::Move::*;

		return match s {
			"A" | "X" => Ok(Rock),
			"B" | "Y" => Ok(Paper),
			"C" | "Z" => Ok(Scissors),
			_ => Err("Expected valid move."),
		};
	}
}

impl Move {
	fn beats(&self) -> Move {
		use crate::Move::*;

		return match self {
			Rock => Scissors,
			Paper => Rock,
			Scissors => Paper,
		};
	}

	fn get_outcome(&self, opponent: Move) -> Outcome {
		use crate::Outcome::*;

		if self == &opponent {
			return Draw;
		}

		if self.beats() == opponent {
			return Win;
		}

		return Loss;
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
	Loss = 0,
	Draw = 3,
	Win = 6,
}

impl std::str::FromStr for Outcome {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use crate::Outcome::*;

		return match s {
			"X" => Ok(Loss),
			"Y" => Ok(Draw),
			"Z" => Ok(Win),
			_ => Err("Expected valid outcome."),
		};
	}
}

impl Outcome {
	fn get_move(&self, opponent: Move) -> Move {
		use crate::Move::*;

		for r#move in [Rock, Paper, Scissors] {
			if self == &r#move.get_outcome(opponent) {
				return r#move;
			}
		}
		unreachable!()
	}
}

struct PartA;
impl PartA {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut result = 0;
		for game in input.lines() {
			let (opponent, me) = game.split_once(" ").unwrap();
			let opponent = opponent.parse::<Move>().unwrap();
			let me = me.parse::<Move>().unwrap();
			let outcome = me.get_outcome(opponent);
			result += me as usize + outcome as usize;
		}

		return result;
	}
}

struct PartB;
impl PartB {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut result = 0;
		for game in input.lines() {
			let (opponent, outcome) = game.split_once(" ").unwrap();
			let opponent = opponent.parse::<Move>().unwrap();
			let outcome = outcome.parse::<Outcome>().unwrap();
			let me = outcome.get_move(opponent);
			result += me as usize + outcome as usize;
		}

		return result;
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

		assert_eq!(15, result);

		return Ok(());
	}

	#[test]
	fn part_b() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartB::solve(&input);

		assert_eq!(12, result);

		return Ok(());
	}
}
