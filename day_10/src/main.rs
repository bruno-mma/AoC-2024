#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"0123\n",
		"1234\n",
		"8765\n",
		"9876\n"
	)
}

fn test_input_2() -> &'static str {
	concat!(
		"89010123\n",
		"78121874\n",
		"87430965\n",
		"96549874\n",
		"45678903\n",
		"32019012\n",
		"01329801\n",
		"10456732\n"
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn parse_input(input: &str) -> ((i32, i32), Vec<u8>) {
	let num_columns = input.chars().position(|c| c == '\n').unwrap() as i32;
	let num_rows = input.lines().count() as i32;

	let map = input.lines()
		.flat_map(|line| line.chars())
		.map(|c| c.to_digit(10).unwrap() as u8)
		.collect();

	((num_rows, num_columns), map)
}

fn get_map_height(position: (i32, i32), map_size: (i32, i32), map: &[u8]) -> u8 {
	let index = position.0 * map_size.1 + position.1;
	map[index as usize]
}

fn sum_trailhead_scores(map_size: (i32, i32), map: &[u8]) -> usize {
	(0..map_size.0)
		.flat_map(|row| (0.. map_size.1).map(move |column| (row, column)))
		.filter(|position| get_map_height(*position, map_size, map) == 0)
		.map(|trailhead_start| get_trailhead_end_positions(trailhead_start, 0, map_size, map))
		.map(|trailhead_ends| trailhead_ends.len())
		.sum()
}

fn get_trailhead_end_positions(curr_pos: (i32, i32), current_height: u8, map_size: (i32, i32), map: &[u8]) -> HashSet<(i32, i32)> {
	let mut result = HashSet::new();
	if curr_pos.0 < 0 || curr_pos.0 >= map_size.0 || curr_pos.1 < 0 || curr_pos.1 >= map_size.1 {
		return result;
	}
	if get_map_height(curr_pos, map_size, map) != current_height {return result}
	if current_height == 9 {
		result.insert(curr_pos);
		return result;
	}

	let new_height = current_height + 1;
	[(0,1), (1, 0), (0, -1), (-1, 0)].iter()
		.map(|pos_delta| (pos_delta.0 + curr_pos.0, pos_delta.1 + curr_pos.1))
		.flat_map(|new_pos| get_trailhead_end_positions(new_pos, new_height, map_size, map))
		.collect()
}

fn sum_trailhead_ratings(map_size: (i32, i32), map: &[u8]) -> usize {
	(0..map_size.0)
		.flat_map(|row| (0.. map_size.1).map(move |column| (row, column)))
		.filter(|position| get_map_height(*position, map_size, map) == 0)
		.map(|trailhead_start| get_trailhead_rating(trailhead_start, 0, map_size, map))
		.sum()
}

fn get_trailhead_rating(curr_pos: (i32, i32), current_height: u8, map_size: (i32, i32), map: &[u8]) -> usize {
	if curr_pos.0 < 0 || curr_pos.0 >= map_size.0 || curr_pos.1 < 0 || curr_pos.1 >= map_size.1 {
		return 0;
	}
	if get_map_height(curr_pos, map_size, map) != current_height {return 0}
	if current_height == 9 {
		return 1;
	}

	let new_height = current_height + 1;
	[(0,1), (1, 0), (0, -1), (-1, 0)].iter()
		.map(|pos_delta| (pos_delta.0 + curr_pos.0, pos_delta.1 + curr_pos.1))
		.map(|new_pos| get_trailhead_rating(new_pos, new_height, map_size, map))
		.sum()
}

fn main() {
	// let input = test_input_3();
	let input = &read_input_file("input.txt");
	let (map_size, map) = parse_input(input);

	println!("Trailhead scores: {}", sum_trailhead_scores(map_size, &map));
	println!("Trailhead ratings: {}", sum_trailhead_ratings(map_size, &map));
}
