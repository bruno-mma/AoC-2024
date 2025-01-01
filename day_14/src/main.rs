#![allow(dead_code)]

use std::fs;

fn test_input_1() -> &'static str {
	concat!(
		"p=0,4 v=3,-3\n",
		"p=6,3 v=-1,-3\n",
		"p=10,3 v=-1,2\n",
		"p=2,0 v=2,-1\n",
		"p=0,0 v=1,3\n",
		"p=3,0 v=-2,-2\n",
		"p=7,6 v=-1,-3\n",
		"p=3,0 v=-1,-2\n",
		"p=9,3 v=2,3\n",
		"p=7,3 v=-1,2\n",
		"p=2,4 v=2,-3\n",
		"p=9,5 v=-3,-3\n",
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

type Robot = ((i32, i32), (i32, i32));

fn parse_input(input: &str) -> Vec<Robot> {
	input.lines()
		.map(|line| {
			let mut parts = line.split(' ');
			let mut pos_parts = parts.next().unwrap().split('=').nth(1).unwrap().split(',');
			let mut vel_parts = parts.next().unwrap().split('=').nth(1).unwrap().split(',');
			let pos = (pos_parts.next().unwrap().parse().unwrap(), pos_parts.next().unwrap().parse().unwrap());
			let vel = (vel_parts.next().unwrap().parse().unwrap(), vel_parts.next().unwrap().parse().unwrap());
			(pos, vel)
		})
		.collect()
}

// assuming robot speed in axis is less than map size in same axis
fn wrap_robot_pos_axis(coord: i32, max_coord: i32) -> i32 {
	if coord < 0 {
		coord + max_coord
	} else if coord >= max_coord {
		coord - max_coord
	} else {
		coord
	}
}

fn next_robot_state(robot: Robot, map_size: (i32, i32)) -> Robot {
	let (p, v) = robot;
	let p = (p.0 + v.0, p.1 + v.1);
	let p = (wrap_robot_pos_axis(p.0, map_size.0), wrap_robot_pos_axis(p.1, map_size.1));
	(p, v)
}

fn wait_seconds(robots: Vec<Robot>, seconds: u32, map_size: (i32, i32)) -> Vec<Robot> {
	robots.into_iter()
		.map(|mut robot| {
			for _ in 0..seconds {robot = next_robot_state(robot, map_size) }
			robot
		})
		.collect()
}

fn compute_safety_factor(robots: Vec<Robot>, map_size: (i32, i32)) -> i32 {
	let m_column = map_size.0 / 2;
	let m_row = map_size.1 / 2;
	let mut quadrant_count = [0; 4];

	for robot in robots {
		let (p, _) = robot;

		if p.0 < m_column {
			if p.1 < m_row {
				quadrant_count[0] += 1;
			} else if p.1 > m_row {
				quadrant_count[1] += 1;
			}
		} else if p.0 > m_column {
			if p.1 < m_row {
				quadrant_count[2] += 1;
			} else if p.1 > m_row {
				quadrant_count[3] += 1;
			}
		}
	}

	quadrant_count.into_iter().reduce(|acc, e|{
		if e == 0 {acc}
		else {acc * e}
	}).unwrap()
}

fn main() {
	// let input = test_input_1();
	// let map_size = (11, 7);

	let input = &read_input_file("input.txt");
	let map_size = (101, 103);

	let robots = parse_input(input);
	let robots = wait_seconds(robots, 100, map_size);
	println!("Safety score: {}", compute_safety_factor(robots, map_size))
}
