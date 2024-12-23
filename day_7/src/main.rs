#![allow(dead_code)]

use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"190: 10 19\n",
		"3267: 81 40 27\n",
		"83: 17 5\n",
		"156: 15 6\n",
		"7290: 6 8 6 15\n",
		"161011: 16 10 13\n",
		"192: 17 8 14\n",
		"21037: 9 7 18 13\n",
		"292: 11 6 16 20\n",
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

// operands will be in reverse order for efficient pop
fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
	input.lines().map(|line| {
		let mut parts = line.split(": ");
		let result = parts.next().unwrap().parse().unwrap();
		let operands = parts.next().unwrap().split(' ').map(|x| x.parse().unwrap()).rev().collect();
		(result, operands)
	}).collect()
}

fn get_equation_result_if_correct(result: u64, operands: &[u64], current_value: u64) -> u64 {
	if current_value > result {
		return 0
	}

	if let Some((next_operand, other_operands)) = operands.split_last() {
		let result_mul = get_equation_result_if_correct(result, other_operands, current_value * next_operand);
		if result_mul > 0 {
			result_mul
		} else {
			get_equation_result_if_correct(result, other_operands, current_value + next_operand)
		}
	} else if current_value == result { result } else { 0 }
}

fn sum_of_correct_equations(equations: &[(u64, Vec<u64>)]) -> u64 {
	equations.iter()
		.map(|(result, operands)| get_equation_result_if_correct(*result, operands, 0))
		.sum()
}

fn u64_concatenation(a: u64, b: u64) -> u64 {
	let mut b_copy = b;
	let mut a_copy = a;
	while b_copy > 0 {
		a_copy *= 10;
		b_copy /= 10;
	}
	a_copy + b
}

fn get_equation_result_if_correct_concatenation(result: u64, operands: &[u64], current_value: u64) -> u64 {
	if current_value > result {
		return 0
	}

	if let Some((next_operand, other_operands)) = operands.split_last() {
		let result_mul = get_equation_result_if_correct_concatenation(result, other_operands, current_value * next_operand);
		if result_mul > 0 {
			return result_mul
		}

		let result_concat = get_equation_result_if_correct_concatenation(result, other_operands, u64_concatenation(current_value, *next_operand));
		if result_concat > 0 {
			return result_concat
		}

		get_equation_result_if_correct_concatenation(result, other_operands, current_value + next_operand)
	} else if current_value == result { result } else { 0 }
}

fn sum_of_correct_equations_concatenation_allowed(equations: &[(u64, Vec<u64>)]) -> u64 {
	equations.iter()
		.map(|(result, operands)| get_equation_result_if_correct_concatenation(*result, operands, 0))
		.sum()
}

fn main() {
	// let input = test_input_1();
	let input = &read_input_file("input.txt");

	let equations = parse_input(input);
	println!("Sum of correct equations: {}", sum_of_correct_equations(&equations));
	println!("Sum of correct equations (concatenation allowed): {}", sum_of_correct_equations_concatenation_allowed(&equations));
}
