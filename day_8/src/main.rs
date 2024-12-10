#![allow(dead_code)]

use itertools::Itertools; // Just for unique() ;)
use std::collections::HashMap;
use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"..........\n",
		"..........\n",
		"..........\n",
		"....a.....\n",
		"........a.\n",
		".....a....\n",
		"..........\n",
		"..........\n",
		"..........\n",
		"..........\n"
	)
}

fn test_input_2() -> &'static str {
	concat!(
		"............\n",
		"........0...\n",
		".....0......\n",
		".......0....\n",
		"....0.......\n",
		"......A.....\n",
		"............\n",
		"............\n",
		"........A...\n",
		".........A..\n",
		"............\n",
		"............"
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

type Position = (i32, i32);

fn parse_input(input: &str) -> ((i32, i32), HashMap<char, Vec<Position>>) {
	let num_columns = input.chars().position(|c| c == '\n').unwrap() as i32;
	let num_rows = input.lines().count() as i32;
	let mut freq_to_antennas: HashMap<char, Vec<Position>> = HashMap::new();

	input.lines().enumerate()
		.flat_map(|(row, line)|
			line.chars().enumerate()
				.map(move |(column, c)| (c, (row as i32, column as i32)))
		)
		.filter(|(char, _)| *char != '.')
		.for_each(|(char, (r, c))|
			freq_to_antennas.entry(char).or_default().push((r, c))
		);

	((num_rows, num_columns), freq_to_antennas)
}

fn pos_in_bounds(pos: Position, map_size: (i32, i32)) -> bool {
	pos.0 >= 0 && pos.0 < map_size.0 && pos.1 >= 0 && pos.1 < map_size.1
}

fn count_anti_nodes(map_size: (i32, i32), freq_to_antennas: &HashMap<char, Vec<Position>>) -> usize {
	freq_to_antennas.iter()
		.flat_map(|(_, antennas)| {
			antennas.iter().enumerate().flat_map(move |(i, &a1)| {
				antennas.iter().skip(i + 1).map(move |&a2| (a1, a2))
			})
		})
		.flat_map(|(a1, a2)| {
			let diff = (a2.0 - a1.0, a2.1 - a1.1);
			[(a1.0 - diff.0, a1.1 - diff.1), (a2.0 + diff.0, a2.1 + diff.1)]
		})
		.filter(|pos| pos_in_bounds(*pos, map_size))
		.unique()
		.count()
}

fn count_line_anti_nodes(map_size: (i32, i32), freq_to_antennas: &HashMap<char, Vec<Position>>) -> usize {
	freq_to_antennas.iter()
		.flat_map(|(_, antennas)| {
			antennas.iter().enumerate().flat_map(move |(i, &a1)| {
				antennas.iter().skip(i + 1).map(move |&a2| (a1, a2))
			})
		})
		.flat_map(|(a1, a2)| {
			let diff = (a2.0 - a1.0, a2.1 - a1.1);
			std::iter::successors(Some(a1), move |&(r, c)| {
				let next = (r + diff.0, c + diff.1);
				if pos_in_bounds(next, map_size) { Some(next) } else { None }
			})
			.chain(std::iter::successors(Some(a2), move |&(r, c)| {
				let next = (r - diff.0, c - diff.1);
				if pos_in_bounds(next, map_size) { Some(next) } else { None }
			}))
		})
		.unique()
		.count()
}


fn main() {
	// let input = test_input_2();
	let input = &read_input_file("input.txt");
	let (map_size, antennas) = parse_input(input);

	println!("Number of anti nodes: {}", count_anti_nodes(map_size, &antennas));
	println!("Number of line anti nodes: {}", count_line_anti_nodes(map_size, &antennas));
}
