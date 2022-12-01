fn main() -> anyhow::Result<()> {
	const INPUT: &str = include_str!("../input.txt");

	let result_a = PartA::solution(INPUT);
	println!("Part A: {:?}", result_a);

	let result_b = PartB::solution(INPUT);
	println!("Part B: {:?}", result_b);

	return Ok(());
}

struct PartA;
impl PartA {
	pub fn solution(input: &str) -> i32 {
		return input
			.trim() // remove whitespaces
			.split("\n\n") // split at empty lines
			.map(|elf| elf.lines().map(|num| num.parse::<i32>().unwrap()).sum()) // sum all numbers per elf
			.max() // get the highest number
			.unwrap();
	}
}

struct PartB;
impl PartB {
	pub fn solution(input: &str) -> i32 {
		let mut all_nums = input
			.trim() // remove whitespaces
			.split("\n\n") // split at empty lines
			.map(|elf| elf.lines().map(|num| num.parse::<i32>().unwrap()).sum()) // sum all numbers per elf
			.collect::<Vec<i32>>();
		all_nums.sort_by(|a, b| b.cmp(a)); // sort from highest to lowest
		return all_nums.into_iter().take(3).sum(); // take the largest 3 elements and sum them up
	}
}

/* Part A
	pub fn solution_naive(input: &str) -> i32 {
		let mut max = 0;
		let mut tmp = 0;

		input.lines().map(|ln| ln.trim()).for_each(|ln| {
			if let Ok(num) = ln.parse::<i32>() {
				tmp += num;
			} else {
				if tmp > max {
					max = tmp;
				}
				tmp = 0;
			}
		});

		return max;
	}

*/

/* Part B
	pub fn solution_bad(input: &str) -> i32 {
		let mut all_nums = Vec::new();
		let mut tmp = 0;

		input.lines().map(|ln| ln.trim()).for_each(|ln| {
			if let Ok(num) = ln.parse::<i32>() {
				tmp += num;
			} else {
				all_nums.push(tmp);
				tmp = 0;
			}
		});

		all_nums.sort_unstable();

		return all_nums[all_nums.len() - 3..all_nums.len()].into_iter().sum();
	}

	pub fn solution_better(input: &str) -> i32 {
		let mut top3 = [0; 3];
		let mut tmp = 0;

		input.lines().map(|ln| ln.trim()).for_each(|ln| {
			if let Ok(num) = ln.parse::<i32>() {
				tmp += num;
			} else {
				for num in top3.iter_mut() {
					if tmp > *num {
						*num = tmp;
						break;
					}
				}
				tmp = 0;
			}
		});

		return top3.into_iter().sum();
	}

*/
