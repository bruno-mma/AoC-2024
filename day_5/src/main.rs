#![allow(dead_code)]

use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"47|53\n",
		"97|13\n",
		"97|61\n",
		"97|47\n",
		"75|29\n",
		"61|13\n",
		"75|53\n",
		"29|13\n",
		"97|29\n",
		"53|29\n",
		"61|53\n",
		"97|53\n",
		"61|29\n",
		"47|13\n",
		"75|47\n",
		"97|75\n",
		"47|61\n",
		"75|61\n",
		"47|29\n",
		"75|13\n",
		"53|13\n",
		"\n",
		"75,47,61,53,29\n",
		"97,61,53,29,13\n",
		"75,29,13\n",
		"75,97,47,61,53\n",
		"61,13,29\n",
		"97,13,75,29,47\n",
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
	let mut rules_pages_split = input.split("\n\n");

	let rules: Vec<(u32, u32)> = rules_pages_split.next().unwrap()
		.lines()
		.map(|rule| {
			let mut rule_split = rule.split('|');
			(rule_split.next().unwrap(), rule_split.next().unwrap())
		})
		.map(|(before, after)| (before.parse::<u32>().unwrap(), after.parse::<u32>().unwrap()))
		.collect();

	let manuals = rules_pages_split.next().unwrap()
		.lines()
		.map(|manual_line|
			manual_line
				.split(",")
				.map(|manual_file| manual_file.parse::<u32>().unwrap())
				.collect()
		).collect();

	(rules, manuals)
}

fn main() {
	println!("{:?}", parse_input(test_input_1()));
}
