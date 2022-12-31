#![allow(dead_code)]

const AOC: &str = "AOC rules";

#[derive(Clone, Copy, PartialEq)]
enum Move {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

use Move::*;

impl std::str::FromStr for Move {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" | "X" => Ok(Rock),
			"B" | "Y" => Ok(Paper),
			"C" | "Z" => Ok(Scissors),
			_ => unreachable!("{AOC}"),
		}
	}
}

impl Move {
	fn beats(&self) -> Move {
		match self {
			Rock => Scissors,
			Paper => Rock,
			Scissors => Paper,
		}
	}

	fn play(&self, opponent: Move) -> Outcome {
		if self == &opponent {
			return Draw;
		} else if self.beats() == opponent {
			return Win;
		}

		Loss
	}
}

#[derive(PartialEq)]
enum Outcome {
	Loss = 0,
	Draw = 3,
	Win = 6,
}

use Outcome::*;

impl std::str::FromStr for Outcome {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Loss),
			"Y" => Ok(Draw),
			"Z" => Ok(Win),
			_ => unreachable!("{AOC}"),
		}
	}
}

impl Outcome {
	fn compute_move(&self, opponent: Move) -> Move {
		for potential_move in [Rock, Paper, Scissors] {
			if &potential_move.play(opponent) == self {
				return potential_move;
			}
		}
		unreachable!("{AOC}");
	}
}

struct PartA;
impl PartA {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut result = 0;

		for game in input.lines() {
			let (opponent, me) = game.split_once(' ').expect("{AOC}");
			let opponent: Move = opponent.parse().expect("{AOC}");
			let me: Move = me.parse().expect("{AOC}");
			let outcome = me.play(opponent);
			result += me as usize + outcome as usize;
		}

		result
	}
}

struct PartB;
impl PartB {
	#[allow(unused_variables)]
	pub fn solve(input: &str) -> usize {
		let mut result = 0;

		for game in input.lines() {
			let (opponent, outcome) = game.split_once(' ').expect("{AOC}");
			let opponent: Move = opponent.parse().expect("{AOC}");
			let outcome: Outcome = outcome.parse().expect("{AOC}");
			let me = outcome.compute_move(opponent);
			result += me as usize + outcome as usize;
		}

		result
	}
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("input.txt")?;

	let result_a = PartA::solve(&input);
	println!("PartA: {:?}", result_a);

	let result_b = PartB::solve(&input);
	println!("PartB: {:?}", result_b);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_a() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartA::solve(&input);

		assert_eq!(15, result);

		Ok(())
	}

	#[test]
	fn part_b() -> anyhow::Result<()> {
		let input = std::fs::read_to_string("test_input.txt")?;
		let result = PartB::solve(&input);

		assert_eq!(12, result);

		Ok(())
	}
}
