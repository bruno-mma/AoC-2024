#![allow(dead_code)]

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

fn parse_input(input: &str) -> Vec<u64> {
	input.split(" ")
		.map(|str| str.trim_end())
		.map(|num| num.parse().unwrap())
		.collect()
}

fn split_if_digit_count_even(num: u64) -> Option<(u64, u64)> {
	let digit_count = num.ilog10() + 1;
	if digit_count % 2 == 0 {
		let mask = 10_u64.pow(digit_count / 2);
		let right_split = num % mask;
		let left_split = num / mask;
		Some((left_split, right_split))
	} else {
		None
	}
}

fn blink_times(mut stones: Vec<u64>, blink_count: u32) -> Vec<u64> {
	for blink in 0..blink_count {
		// println!("Blink {}", blink);
		let mut i = 0;
		while i < stones.len() {
			if stones[i] == 0 {
				stones[i] = 1;
			}
			else if let Some(stone_split) = split_if_digit_count_even(stones[i]) {
				stones[i] = stone_split.0;
				stones.insert(i + 1, stone_split.1);
				i += 1;
			} else {
				stones[i] *= 2024;
			}
			i += 1;
		}
	}
	stones
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	let input = &read_input_file("input.txt");
	
	let stones = parse_input(input);
	println!("Number of stones after 25 blinks: {:?}", blink_times(stones.clone(), 25).len());
	// println!("Number of stones after 75 blinks: {:?}", blink_times(stones, 75).len());
}
