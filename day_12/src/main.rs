#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"AAAA\n",
		"BBCD\n",
		"BBCC\n",
		"EEEC\n"
	)
}

fn test_input_2() -> &'static str {
	concat!(
		"OOOOO\n",
		"OXOXO\n",
		"OOOOO\n",
		"OXOXO\n",
		"OOOOO\n"
	)
}

fn test_input_3() -> &'static str {
	concat!(
		"RRRRIICCFF\n",
		"RRRRIICCCF\n",
		"VVRRRCCFFF\n",
		"VVRCCCJFFF\n",
		"VVVVCJJCFE\n",
		"VVIVCCJJEE\n",
		"VVIIICJJEE\n",
		"MIIIIIJJEE\n",
		"MIIISIJEEE\n",
		"MMMISSJEEE\n"
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn parse_input(input: &str) -> ((i32, i32), Vec<char>) {
	let num_columns = input.chars().position(|c| c == '\n').unwrap() as i32;
	let num_rows = input.lines().count() as i32;

	let map = input.lines()
		.flat_map(|line| line.chars())
		.collect();

		((num_rows, num_columns), map)
}

fn is_pos_in_bounds(pos: (i32, i32), map_size: (i32, i32)) -> bool {
	pos.0 >= 0 && pos.0 < map_size.0 && pos.1 >= 0 && pos.1 < map_size.1
}

fn get_pos_plant_type(position: (i32, i32), map_size: (i32, i32), map: &[char]) -> char {
	let index = position.0 * map_size.1 + position.1;
	map[index as usize]
}

fn get_fence_price(map_size: (i32, i32), map: &[char]) -> u32 {
	let mut visited_pos = HashSet::new();

	(0..map_size.0)
		.flat_map(|row| (0.. map_size.1).map(move |column| (row, column)))
		.map(|pos| (pos, get_pos_plant_type(pos, map_size, map)))
		.map(|(pos, plant_type)| get_plot_area_perimeter(pos, plant_type, map_size, map, &mut visited_pos))
		.map(|(area, perimeter)| area * perimeter)
		.sum()
}

fn get_plot_area_perimeter(pos: (i32, i32), plant_type: char, map_size: (i32, i32), map: &[char], visited_pos: &mut HashSet<(i32, i32)>) -> (u32, u32) { // (area, perimeter)
	if !is_pos_in_bounds(pos, map_size) {
		return (0, 1)
	}

	if get_pos_plant_type(pos, map_size, map) != plant_type {
		return (0, 1)
	}

	if visited_pos.contains(&pos) {
		return (0, 0)
	}

	visited_pos.insert(pos);

	[(0,1), (1, 0), (0, -1), (-1, 0)].iter()
		.map(|pos_delta| (pos_delta.0 + pos.0, pos_delta.1 + pos.1))
		.map(|new_pos| get_plot_area_perimeter(new_pos, plant_type, map_size, map, visited_pos))
		.fold((1, 0), |acc, new_pos| (acc.0 + new_pos.0, acc.1 + new_pos.1))
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	// let input = test_input_3();
	let input = &read_input_file("input.txt");

	let (map_size, map) = parse_input(input);
	println!("{:?}", map_size);
	println!("Fence price: {:?}", get_fence_price(map_size, &map));
}
