#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}, fs};

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

	let rules = rules_pages_split.next().unwrap()
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

fn valid_manuals_middle_page_sum(rules: Vec<(u32, u32)>, manuals: Vec<Vec<u32>>) -> u32 {
	let mut page_to_pages_after: HashMap<u32, HashSet<u32>> = HashMap::new();

	for (bef, aft) in rules {
		page_to_pages_after.entry(bef).or_default().insert(aft);
	}

	manuals.iter()
		.map(|manual| {
			let not_valid = manual.iter().enumerate().rev()
				.any(|(i, page)| {
					if let Some(pages_after) = page_to_pages_after.get(page) {
						manual[..i].iter().any(|page| pages_after.contains(page))
					} else {
						false
					}
				});
			(manual, !not_valid)
		})
		.filter(|(_manual, valid)| *valid)
		.map(|(manual, _valid)| manual.get(manual.len() / 2).unwrap())
		.sum()
}

fn main() {
	// let input = test_input_1();
	let input = &read_input_file("input.txt");

	let (rules, manuals) = parse_input(input);
	println!("Sum of middle page of valid manuals: {}", valid_manuals_middle_page_sum(rules, manuals));
}
