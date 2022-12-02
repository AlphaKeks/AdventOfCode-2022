#![allow(dead_code)]

#[derive(Clone, Copy)]
enum Move {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl std::str::FromStr for Move {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use crate::Move::*;
		return match s {
			"A" | "X" => Ok(Rock),
			"B" | "Y" => Ok(Paper),
			"C" | "Z" => Ok(Scissors),
			_ => Err("Expected valid move.".into()),
		};
	}
}

impl Move {
	fn play(&self, opponent: Move) -> Outcome {
		use crate::{Move::*, Outcome::*};
		return match self {
			Rock => match opponent {
				Rock => Draw,
				Paper => Loss,
				Scissors => Win,
			},
			Paper => match opponent {
				Rock => Win,
				Paper => Draw,
				Scissors => Loss,
			},
			Scissors => match opponent {
				Rock => Loss,
				Paper => Win,
				Scissors => Draw,
			},
		};
	}

	fn from_outcome(outcome: Outcome, opponent: Move) -> Move {
		use crate::{Move::*, Outcome::*};
		match outcome {
			Loss => match opponent {
				Rock => Scissors,
				Paper => Rock,
				Scissors => Paper,
			},
			Draw => match opponent {
				Rock => Rock,
				Paper => Paper,
				Scissors => Scissors,
			},
			Win => match opponent {
				Rock => Paper,
				Paper => Scissors,
				Scissors => Rock,
			},
		}
	}
}

#[derive(Clone, Copy)]
enum Outcome {
	Loss = 0,
	Draw = 3,
	Win = 6,
}

impl std::str::FromStr for Outcome {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use crate::Outcome::*;
		return match s {
			"X" => Ok(Loss),
			"Y" => Ok(Draw),
			"Z" => Ok(Win),
			_ => Err("Expected valid outcome.".into()),
		};
	}
}

fn main() -> anyhow::Result<()> {
	let input = include_str!("../input.txt");

	let solution_a = PartA::solve(input);
	println!("Part A: {}", solution_a);

	let solution_b = PartB::solve(input);
	println!("Part B: {}", solution_b);

	return Ok(());
}

struct PartA;
impl PartA {
	fn solve(input: &str) -> usize {
		let mut result = 0;
		for game in input.lines() {
			let (opponent, me) = game.split_once(" ").unwrap();
			let opponent = opponent.parse::<Move>().unwrap();
			let me = me.parse::<Move>().unwrap();
			let outcome = me.play(opponent);
			result += me as usize + outcome as usize;
		}
		return result;
	}
}

struct PartB;
impl PartB {
	fn solve(input: &str) -> usize {
		let mut result = 0;
		for game in input.lines() {
			let (opponent, outcome) = game.split_once(" ").unwrap();
			let opponent = opponent.parse::<Move>().unwrap();
			let outcome = outcome.parse::<Outcome>().unwrap();
			let me = Move::from_outcome(outcome, opponent);
			result += me as usize + outcome as usize;
		}
		return result;
	}
}
