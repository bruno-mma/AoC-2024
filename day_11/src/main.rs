#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;

fn test_input_1() -> &'static str {
	"0 1 10 99 999"
}

fn test_input_2() -> &'static str {
	"125 17"
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
	input
		.split_whitespace()
		.map(|num| num.parse().unwrap())
		.fold(HashMap::new(), |mut stone_counts, stone| {
			*stone_counts.entry(stone).or_insert(0) += 1;
			stone_counts
		})
}


fn digit_count(mut num: u64) -> u32 {
	let mut digit_count = 0;
	while num != 0 {
		num /= 10;
		digit_count += 1;
	}
	digit_count
}

fn split_if_digit_count_even(num: u64) -> Option<(u64, u64)> {
	let digit_count = digit_count(num);
	if digit_count % 2 == 0 {
		let mask = 10_u64.pow(digit_count / 2);
		let right_split = num % mask;
		let left_split = num / mask;
		Some((left_split, right_split))
	} else {
		None
	}
}

fn get_stone_blink_result(stone: u64) -> (u64, Option<u64>) {
	if stone == 0 {
		return (1, None);
	}

	if let Some(stone_split) = split_if_digit_count_even(stone) {
		(stone_split.0, Some(stone_split.1))
	} else {
		(stone * 2024, None)
	}
}

fn blink_times(mut stones: HashMap<u64, u64>, blink_count: u32) -> HashMap<u64, u64> {
	for _ in 0..blink_count {
		let mut blink_result = HashMap::new();

		for (stone, stone_count) in stones.iter() {
			let new_stones = get_stone_blink_result(*stone);

			*blink_result.entry(new_stones.0).or_insert(0) += stone_count;
			if let Some(new_stone_1) = new_stones.1 {
				*blink_result.entry(new_stone_1).or_insert(0) += stone_count;
			}
		}

		stones = blink_result.clone();
	}

	stones
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	let input = &read_input_file("input.txt");

	let stone_to_stone_count = parse_input(input);

	println!("Number of stones after 25 blinks: {}", blink_times(stone_to_stone_count.clone(), 25).values().sum::<u64>());
	println!("Number of stones after 75 blinks: {}", blink_times(stone_to_stone_count, 75).values().sum::<u64>());
}
