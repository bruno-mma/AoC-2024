#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"....#.....\n",
		".........#\n",
		"..........\n",
		"..#.......\n",
		".......#..\n",
		"..........\n",
		".#..^.....\n",
		"........#.\n",
		"#.........\n",
		"......#...\n",
    )
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn parse_input(input: &str) -> ((i32, i32), (i32, i32), HashSet<(i32, i32)>) {
	let num_columns = input.chars().position(|c| c == '\n').unwrap() as i32;
	let num_rows = input.lines().count() as i32;

	let guard_pos = input.chars().position(|c| c == '^').unwrap() as i32;
	let guard_pos = (guard_pos / num_rows, guard_pos % (num_columns + 1));

	let obstacles: HashSet<(i32, i32)> = input.lines()
		.enumerate()
		.flat_map(|(row, line)| {
			line.chars().enumerate()
				.map(move |(column, char)| ((row, column), char))
				.filter(|((_row, _column), char)| *char == '#')
				.map(|((row, column), _char)| (row as i32, column as i32))
		})
		.collect();

	((num_rows, num_columns), guard_pos, obstacles)
}

fn is_guard_pos_in_bounds(map_size: (i32, i32), guard_pos: (i32, i32)) -> bool {
	guard_pos.0 >= 0 && guard_pos.0 < map_size.0 && guard_pos.1 >= 0 && guard_pos.1 < map_size.1
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
enum GuardDirection {
	Up,
	Right,
	Down,
	Left
}

fn get_next_pos(pos: (i32, i32), dir: &GuardDirection) -> (i32, i32) {
	match dir {
		GuardDirection::Up => {(pos.0 - 1, pos.1)}
		GuardDirection::Right => {(pos.0, pos.1 + 1)}
		GuardDirection::Down => {(pos.0 + 1, pos.1)}
		GuardDirection::Left => {(pos.0, pos.1 - 1)}
	}
}

fn get_next_dir(dir: GuardDirection) -> GuardDirection {
	match dir {
		GuardDirection::Up => {GuardDirection::Right}
		GuardDirection::Right => {GuardDirection::Down}
		GuardDirection::Down => {GuardDirection::Left}
		GuardDirection::Left => {GuardDirection::Up}
	}
}

fn get_visited_positions(map_size: (i32, i32), mut guard_pos: (i32, i32), obstacles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
	let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
	let mut guard_dir = GuardDirection::Up;

	// Let's assume that the guard will not get stuck in a loop
	loop {
		visited_positions.insert(guard_pos);

		let next_pos = get_next_pos(guard_pos, &guard_dir);
		if !is_guard_pos_in_bounds(map_size, next_pos) {break}
		if !obstacles.contains(&next_pos) {
			guard_pos = next_pos
		} else {
			guard_dir = get_next_dir(guard_dir);
		}
	}

	visited_positions
}

fn test_loop(map_size: (i32, i32), mut guard_pos: (i32, i32), obstacles: &HashSet<(i32, i32)>) -> bool {
	let mut visited_positions: HashSet<(i32, i32, GuardDirection)> = HashSet::new();
	let mut guard_dir = GuardDirection::Up;

	loop {
		if !visited_positions.insert((guard_pos.0, guard_pos.1, guard_dir)) {
			// the guard was in this state already, loop detected
			return true
		}

		let next_pos = get_next_pos(guard_pos, &guard_dir);
		if !is_guard_pos_in_bounds(map_size, next_pos) {return false}
		if !obstacles.contains(&next_pos) {
			guard_pos = next_pos
		} else {
			guard_dir = get_next_dir(guard_dir);
		}
	}
}

// this is really slow, probably don't need to check every position...
fn get_num_loop_obstacles(map_size: (i32, i32), guard_pos: (i32, i32), mut obstacles: HashSet<(i32, i32)>) -> usize {
	let possible_loop_obstacles = get_visited_positions(map_size, guard_pos, &obstacles);
	let mut loop_obstacles_count = 0;

	for possible_loop_obstacle in possible_loop_obstacles {
		obstacles.insert(possible_loop_obstacle);

		if test_loop(map_size, guard_pos, &obstacles) {
			loop_obstacles_count += 1
		}

		obstacles.remove(&possible_loop_obstacle);
	}

	loop_obstacles_count
}

fn main() {
	// let input = test_input_1();
	let input = &read_input_file("input.txt");

	let (map_size, guard_pos, obstacles) = parse_input(input);

	println!("Number of visited positions: {}", get_visited_positions(map_size, guard_pos, &obstacles).len());
	println!("Number of possible loop obstacles : {}", get_num_loop_obstacles(map_size, guard_pos, obstacles));
}
