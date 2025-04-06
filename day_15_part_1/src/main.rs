#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;
use std::iter::repeat_n;

fn test_input_1() -> &'static str {
	concat!(
		"########\n",
		"#..O.O.#\n",
		"##@.O..#\n",
		"#...O..#\n",
		"#.#.O..#\n",
		"#...O..#\n",
		"#......#\n",
		"########\n",
		"\n",
		"<^^>>>vv<v>>v<<\n",
	)
}

fn test_input_2() -> &'static str {
	concat!(
		"##########\n",
		"#..O..O.O#\n",
		"#......O.#\n",
		"#.OO..O.O#\n",
		"#..O@..O.#\n",
		"#O#..O...#\n",
		"#O..O..O.#\n",
		"#.OO.O.OO#\n",
		"#....O...#\n",
		"##########\n",
		"\n",
		"<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n",
		"vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n",
		"><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n",
		"<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n",
		"^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n",
		"^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n",
		">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n",
		"<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n",
		"^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n",
		"v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n",
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Dir {
	Up,
	Right,
	Down,
	Left
}

fn get_dir_from_char(c: char) -> Dir {
	match c {
		'^' => Dir::Up,
		'>' => Dir::Right,
		'v' => Dir::Down,
		'<' => Dir::Left,
		_ => panic!("Invalid direction character: {}", c)
	}
}

fn get_pos_delta_from_dir(dir: &Dir) -> (i32, i32) {
	match dir {
		Dir::Up => (-1, 0),
		Dir::Right => (0, 1),
		Dir::Down => (1, 0),
		Dir::Left => (0, -1)
	}
}

#[derive(Debug)]
struct Map {
	size: (i32, i32),
	robot: Pos,
	boxes: HashSet<Pos>,
	obstacles: HashSet<Pos>
}

impl std::fmt::Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let border = repeat_n('#', (self.size.1 + 2) as usize).collect::<String>();
		writeln!(f, "{}", border)?;

		for row in 0..self.size.0 {
			write!(f, "#")?;
			for column in 0..self.size.1 {
				let pos = (row, column);
				let symbol = if pos == self.robot {
					"@"
				} else if self.boxes.contains(&pos) {
					"O"
				} else if self.obstacles.contains(&pos) {
					"#"
				} else {
					"."
				};

				write!(f, "{}", symbol)?;
			}
			writeln!(f, "#")?;
		}

		writeln!(f, "{}", border)
	}
}

fn parse_input(input: &str) -> (Map, Vec<Dir>) {
	let (map, directions) = input.split_once("\n\n").unwrap();

	let num_rows = map.lines().count() - 2;
	let num_columns = map.chars().position(|c| c == '\n').unwrap() - 2;

	let mut robot_pos = (-1, -1);
	let mut boxes = HashSet::new();
	let mut obstacles = HashSet::new();

	map.lines()
		.skip(1)
		.enumerate()
		.filter(|(row, _line)| *row != num_rows)
		.flat_map(|(row, line)|
			line.chars()
				.skip(1)
				.enumerate()
				.filter(|(column, _c)| *column != num_columns)
				.filter(|(_column, c)| *c != '.')
				.map(move |(column, c)| ((row, column), c))
		)
		.map(|((row, column), c)| ((row as i32, column as i32), c))
		.for_each(|((row, column), c)| {
			match c {
				'@' => robot_pos = (row, column),
				'O' => { boxes.insert((row, column)); },
				'#' => { obstacles.insert((row, column)); },
				_ => panic!("Invalid character in map: {}", c)
			};
		});

	if robot_pos == (-1, -1) {
		panic!("Robot not found in map");
	}

	let commands: Vec<Dir> = directions.chars()
		.filter(|c| *c != '\n')
		.map(get_dir_from_char).collect();

	(
		Map {
			size: (num_rows as i32, num_columns as i32),
			robot: robot_pos,
			boxes,
			obstacles,
		},
		commands
	)
}

fn pos_is_wall(pos: &Pos, map: &Map) -> bool {
	let map_size = &map.size;
	let obstacles = &map.obstacles;

	pos.0 < 0 ||
	pos.0 >= map_size.0 ||
	pos.1 < 0 ||
	pos.1 >= map_size.1 ||
	obstacles.contains(pos)
}

fn get_next_empty_pos_in_dir(dir: &Dir, mut curr_pos: Pos, map: &Map) -> (Option<Pos>, Option<Pos>) { // first empty pos, first box
	let pos_delta = get_pos_delta_from_dir(dir);
	let mut first_box = None;

	loop {
		curr_pos = (curr_pos.0 + pos_delta.0, curr_pos.1 + pos_delta.1);
		if pos_is_wall(&curr_pos, map) {
			return (None, first_box);
		}

		if map.boxes.contains(&curr_pos) {
			if first_box.is_none() {
				first_box = Some(curr_pos);
			} else {
				continue;
			}
		} else {
			// found an empty position
			return (Some(curr_pos), first_box);
		}
	}
}

fn execute_command(dir: Dir, map: &mut Map) {
	let (empty_pos, first_box) = get_next_empty_pos_in_dir(&dir, map.robot, map);

	if let Some(empty_pos) = empty_pos {
		if let Some(first_box) = first_box {
			// remove box
			map.boxes.remove(&first_box);
			// add box to new position
			map.boxes.insert(empty_pos);
			// move robot
			map.robot = first_box;
		} else {
			// move robot
			map.robot = empty_pos;
		}
	}
}

fn get_gps_sum(map: Map) -> i32 {
	map.boxes.iter()
		.map(|(row, column)| (row + 1, column + 1))
		.map(|(row, column)| 100 * row + column)
		.sum()
}

fn main() {
	// let input = test_input_2();
	let input = &read_input_file("input.txt");
	let (mut map, commands) = parse_input(input);

	for dir in commands {
		execute_command(dir, &mut map);
	}

	// println!("{}", map);
	println!("GPS: {}", get_gps_sum(map));
}
