#![allow(dead_code)]

use std::cmp::PartialEq;
use std::collections::HashSet;
use std::iter::repeat_n;
use std::ops::Add;
use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"#######\n",
		"#...#.#\n",
		"#.....#\n",
		"#..OO@#\n",
		"#..O..#\n",
		"#.....#\n",
		"#######\n",
		"\n",
		"<vv<<^^<<^^\n",
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
	r: i32,
	c: i32,
}

impl Pos {
	fn new(r: i32, c: i32) -> Self {
		Self { r, c }
	}
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Self {
        Self {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

fn get_pos_delta_from_dir(dir: &Dir) -> Pos {
	match dir {
		Dir::Up => Pos::new(-1, 0),
		Dir::Right => Pos::new(0, 1),
		Dir::Down => Pos::new(1, 0),
		Dir::Left => Pos::new(0, -1)
	}
}

enum CollisionCheckType {
	RobotToBox,
	BoxToBox,
	BoxToWall
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
		let border = repeat_n('#', (self.size.1 + 4) as usize).collect::<String>();
		writeln!(f, "{}", border)?;

		for row in 0..self.size.0 {
			write!(f, "##")?;
			let mut last_symbol_was_box = false;
			for column in 0..self.size.1 {
				let pos = Pos::new(row, column);

				if last_symbol_was_box {
					last_symbol_was_box = false;
					assert!(
						pos != self.robot && !self.boxes.contains(&pos) && !self.obstacles.contains(&pos),
						"Invalid map: box next to box, robot or obstacle"
					);
					continue;
				}

				let symbol;
				if pos == self.robot {
					symbol = "@";
				} else if self.boxes.contains(&pos) {
					symbol = "[]";
					last_symbol_was_box = true;
				} else if self.obstacles.contains(&pos) {
					symbol = "#";
				} else {
					symbol = ".";
				};

				write!(f, "{}", symbol)?;
			}
			writeln!(f, "##")?;
		}

		write!(f, "{}", border)
	}
}

fn parse_input(input: &str) -> (Map, Vec<Dir>) {
	let (map, directions) = input.split_once("\n\n").unwrap();

	let num_rows = map.lines().count() - 2;
	let num_columns = map.chars().position(|c| c == '\n').unwrap() - 2;

	let mut robot_pos = None;
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
		.map(|((row, column), c)| ((row as i32, (column * 2) as i32), c))
		.for_each(|((row, column), c)| {
			match c {
				'@' => robot_pos = Some(Pos::new(row, column)),
				'O' => { boxes.insert( Pos::new(row, column)); },
				'#' => {
					obstacles.insert( Pos::new(row, column));
					obstacles.insert( Pos::new(row, column + 1));
				},
				_ => panic!("Invalid character in map: {}", c)
			};
		});

	assert!(robot_pos.is_some(), "Robot not found in map");

	let commands: Vec<Dir> = directions.chars()
		.filter(|c| *c != '\n')
		.map(get_dir_from_char)
		.collect();

	(
		Map {
			size: (num_rows as i32, (num_columns * 2) as i32),
			robot: robot_pos.unwrap(),
			boxes,
			obstacles,
		},
		commands
	)
}

fn pos_is_wall(pos: &Pos, map: &Map) -> bool {
	let map_size = &map.size;

	pos.r < 0 || pos.r >= map_size.0 ||
	pos.c < 0 || pos.c >= map_size.1 ||
	map.obstacles.contains(pos)
}

fn get_possible_blocking_pos(entity_pos: &Pos, dir: &Dir, collision_type: CollisionCheckType) -> Vec<Pos> {
	let pos_delta = get_pos_delta_from_dir(dir);

	// position deltas to check, relative to new entity position
	let pos_tuple_to_check = if *dir == Dir::Left {
		match collision_type {
			CollisionCheckType::RobotToBox => vec![(0, -1)],
			CollisionCheckType::BoxToBox => vec![(0, -1)],
			CollisionCheckType::BoxToWall => vec![(0, 0)],
		}
	} else if *dir == Dir::Right {
		match collision_type {
			CollisionCheckType::RobotToBox => vec![(0, 0)],
			CollisionCheckType::BoxToBox => vec![(0, 1)],
			CollisionCheckType::BoxToWall => vec![(0, 1)],
		}
	} else {
		match collision_type {
			CollisionCheckType::RobotToBox => vec![(0, -1), (0, 0)],
			CollisionCheckType::BoxToBox => vec![(0, -1), (0, 0), (0, 1)],
			CollisionCheckType::BoxToWall => vec![(0, 0), (0, 1)],
		}
	};

	pos_tuple_to_check.into_iter()
		.map(|(r, c)| Pos::new(r, c))
		.map(|pos| pos + *entity_pos)
		.map(|pos| pos + pos_delta)
		.collect()
}

fn execute_command(dir: &Dir, map: &mut Map) {
	let pos_delta = get_pos_delta_from_dir(dir);
	let new_robot = map.robot + pos_delta;

	// check if there's a wall in the way
	if pos_is_wall(&new_robot, map) {
		return;
	}

	// check if there are boxes in the way
	let blocking_box_pos: Vec<Pos> = get_possible_blocking_pos(&map.robot, dir, CollisionCheckType::RobotToBox).into_iter()
		.filter(|blocking_box| map.boxes.contains(blocking_box))
		.collect();

	if blocking_box_pos.is_empty() {
		// there are no boxes in the way, move the robot
		map.robot = new_robot;
		return;
	}

	assert_eq!(blocking_box_pos.len(), 1, "Invalid map: more than one box in the robot's way");

	// there's a box in the way
	let mut boxes_to_check = Vec::new();
	boxes_to_check.push(blocking_box_pos[0]);
	let mut boxes_to_move = HashSet::new();

	// get all boxes that we might need to move
	while let Some(box_to_move) = boxes_to_check.pop() {
		// check if there is a wall in the way
		if get_possible_blocking_pos(&box_to_move, dir, CollisionCheckType::BoxToWall).into_iter()
			.any(|pos| pos_is_wall(&pos, map)) {
				// there's a wall in the way, can't move
				return;
			}

		// we might be able to move this box
		boxes_to_move.insert(box_to_move);

		// check if there are other boxes in the way, and add them to the list of boxes to check
		get_possible_blocking_pos(&box_to_move, dir, CollisionCheckType::BoxToBox).into_iter()
			.filter(|blocking_box| map.boxes.contains(blocking_box))
			.filter(|blocking_box| !boxes_to_move.contains(blocking_box))
			.for_each(|blocking_box| boxes_to_check.push(blocking_box))
	}

	// move boxes and robot
	let mut boxes_to_add = Vec::with_capacity(boxes_to_move.len());
	for box_to_move in boxes_to_move {
		assert!(map.boxes.remove(&box_to_move), "Box to move does not exist");
		let box_to_add = box_to_move + pos_delta;
		boxes_to_add.push(box_to_add);
	}
	boxes_to_add.iter().for_each(|box_to_add| { map.boxes.insert(*box_to_add); });
	map.robot = new_robot;
}

fn get_gps_sum(map: Map) -> i32 {
	map.boxes.iter()
		.map(|box_pos| *box_pos + Pos::new(1, 1))
		.map(|box_pos| (100 * box_pos.r) + box_pos.c + 1)
		.sum()
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	let input = &read_input_file("input.txt");
	let (mut map, commands) = parse_input(input);
	println!("{}", map);
	let num_boxes_start = map.boxes.len();

	for command in commands {
		execute_command(&command, &mut map);
	}

	assert_eq!(num_boxes_start, map.boxes.len(), "Number of boxes has changed");

	println!("\n{}", map);
	println!("GPS: {}", get_gps_sum(map));
}
