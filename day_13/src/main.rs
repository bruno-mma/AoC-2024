#![allow(dead_code)]

use regex::Regex;
use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"Button A: X+94, Y+34\n",
		"Button B: X+22, Y+67\n",
		"Prize: X=8400, Y=5400\n",
		"\n",
		"Button A: X+26, Y+66\n",
		"Button B: X+67, Y+21\n",
		"Prize: X=12748, Y=12176\n",
		"\n",
		"Button A: X+17, Y+86\n",
		"Button B: X+84, Y+37\n",
		"Prize: X=7870, Y=6450\n",
		"\n",
		"Button A: X+69, Y+23\n",
		"Button B: X+27, Y+71\n",
		"Prize: X=18641, Y=10279\n",
	)
}


fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

type Pos = (i64, i64);

fn parse_input(input: &str) -> Vec<(Pos, Pos, Pos)> { // (A pos delta, B pos delta, prize pos)
	let machine_re = Regex::new(concat!(
		r"Button A: X\+(\d+), Y\+(\d+)\n",
		r"Button B: X\+(\d+), Y\+(\d+)\n",
		r"Prize: X=(\d+), Y=(\d+)",
	)).unwrap();

	machine_re.captures_iter(input)
		.map(|cap| {
			let pos_a = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
			let pos_b = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
			let prize = (cap[5].parse().unwrap(), cap[6].parse().unwrap());
			(pos_a, pos_b, prize)
		})
		.collect()
}

// inputs are just a system of 2 linear equations with 1 (or 0) solution
fn solve_machine(machine: (Pos, Pos, Pos)) -> Option<i64> {
	let (a, b, p) = machine;

	// using Cramer's rule
	let a_b_det = a.0 * b.1 - b.0 * a.1;
	if a_b_det == 0 {
		return None
	}

	let p_b_det = p.0 * b.1 - b.0 * p.1;
	let a_p_det = a.0 * p.1 - p.0 * a.1;

	let a_presses = p_b_det / a_b_det;
	let b_presses = a_p_det / a_b_det;

	if a_presses < 0 || b_presses < 0 {return None}
	if a.0 * a_presses + b.0 * b_presses != p.0 {return None}
	if a.1 * a_presses + b.1 * b_presses != p.1 {return None}

	Some(a_presses * 3 + b_presses)
}

fn sum_machine_cost(machines: &[(Pos, Pos, Pos)], prize_offset: i64) -> i64 {
	machines.iter()
		.map(|machine| {
			let (a, b, prize) = *machine;
			let prize = (prize.0 + prize_offset, prize.1 + prize_offset);
			(a, b, prize)
		})
		.map(|machine| solve_machine(machine).map_or(0, |p| p))
		.sum()
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	let input = &read_input_file("input.txt");

	let machines = parse_input(input);

	println!("Sum of machine costs: {}", sum_machine_cost(&machines, 0));
	println!("Sum of machine costs with prize offset: {}", sum_machine_cost(&machines, 10_000_000_000_000));
}
