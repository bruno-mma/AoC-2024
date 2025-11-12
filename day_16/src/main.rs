#![allow(dead_code)]

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::ops::{Add, AddAssign, Sub, SubAssign};

fn test_input_1() -> &'static str {
	concat!(
		"#####\n",
		"#...#\n",
		"#S#E#\n",
		"#####\n"
	)
}

fn test_input_2() -> &'static str {
	concat!(
		"###############\n",
		"#.......#....E#\n",
		"#.#.###.#.###.#\n",
		"#.....#.#...#.#\n",
		"#.###.#####.#.#\n",
		"#.#.#.......#.#\n",
		"#.#.#####.###.#\n",
		"#...........#.#\n",
		"###.#.#####.#.#\n",
		"#...#.....#.#.#\n",
		"#.#.#.###.#.#.#\n",
		"#.....#...#.#.#\n",
		"#.###.#.#.#.#.#\n",
		"#S..#.....#...#\n",
		"###############\n"
	)
}

fn test_input_3() -> &'static str {
	concat!(
		"#################\n",
		"#...#...#...#..E#\n",
		"#.#.#.#.#.#.#.#.#\n",
		"#.#.#.#...#...#.#\n",
		"#.#.#.#.###.#.#.#\n",
		"#...#.#.#.....#.#\n",
		"#.#.#.#.#.#####.#\n",
		"#.#...#.#.#.....#\n",
		"#.#.#####.#.###.#\n",
		"#.#.#.......#...#\n",
		"#.#.###.#####.###\n",
		"#.#.#...#.....#.#\n",
		"#.#.#.#####.###.#\n",
		"#.#.#.........#.#\n",
		"#.#.#.#########.#\n",
		"#S#.............#\n",
		"#################\n",
	)
}

fn test_input_4() -> &'static str {
	concat!(
		"######\n",
		"#...##\n",
		"#S#.E#\n",
		"#...##\n",
		"######\n"
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Pos(i32, i32);

impl Add for Pos {
	type Output = Pos;

	fn add(self, rhs: Self) -> Self::Output {
		Pos(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl Sub for Pos {
	type Output = Pos;

	fn sub(self, rhs: Self) -> Self::Output {
		Pos(self.0 - rhs.0, self.1 - rhs.1)
	}
}

impl AddAssign for Pos {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl SubAssign for Pos {
	fn sub_assign(&mut self, rhs: Self) {
		self.0 -= rhs.0;
		self.1 -= rhs.1;
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Dir {
	North,
	East,
	South,
	West
}

const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

struct Map {
	num_rows: i32,
	num_columns: i32,
	start: Pos,
	end: Pos,
	positions: Vec<char>
}

struct Graph {
	start: Pos,
	end: Pos,
	neighbours: HashMap<Pos, HashSet<Pos>>
}

fn array_idx_to_coord(array_pos: i32, num_columns: i32) -> Pos {
	Pos(array_pos / num_columns, array_pos % num_columns)
}

fn coord_to_array_idx(pos: &Pos, num_columns: i32) -> usize {
	(pos.0 * num_columns + pos.1) as usize
}

fn get_pos_delta_from_dir(dir: &Dir) -> Pos {
	match dir {
		Dir::North => Pos(-1, 0),
		Dir::East => Pos(0, 1),
		Dir::South => Pos(1, 0),
		Dir::West => Pos(0, -1)
	}
}

fn are_opposite(dir1: &Dir, dir2: &Dir) -> bool {
	matches!((dir1, dir2),
		(Dir::North, Dir::South) |
		(Dir::South, Dir::North) |
		(Dir::East, Dir::West) |
		(Dir::West, Dir::East)
	)
}

fn get_pos_symbol(map: &Map, pos: &Pos) -> char {
	assert!(
		pos.0 >= 0 && pos.0 < map.num_rows && pos.1 >= 0 && pos.1 < map.num_columns,
		"Invalid map position"
	);
	map.positions[coord_to_array_idx(pos, map.num_columns)]
}

fn parse_input(input: &str) -> Map {
	let num_columns = input.chars().position(|c| c == '\n').unwrap() as i32;
	let num_rows = input.lines().count() as i32;

	let positions: Vec<char> = input.lines()
		.flat_map(|line| line.chars())
		.collect();

	let start_pos = positions.iter()
		.position(|&c| c == 'S')
		.expect("Could not find start position in map");
	let start_pos = array_idx_to_coord(start_pos as i32, num_columns);

	let end_pos = positions.iter()
		.position(|&c| c == 'E')
		.expect("Could not find end position in map");
	let end_pos = array_idx_to_coord(end_pos as i32, num_columns);

	Map {
		num_rows,
		num_columns,
		start: start_pos,
		end: end_pos,
		positions,
	}
}

fn is_pos_node(map: &Map, pos: Pos) -> bool {
	if pos == map.start || pos == map.end {
		return true;
	}

	let walkable_dirs = [Dir::North, Dir::East, Dir::South, Dir::West].into_iter()
		.map(|dir| (dir, get_pos_delta_from_dir(&dir)))
		.map(|(dir, pos_delta)| (dir, pos + pos_delta))
		.map(|(dir, new_pos)| (dir, get_pos_symbol(map, &new_pos)))
		.filter(|(_dir, symbol)| *symbol != '#')
		.map(|(dir, _symbol)| dir)
		.collect::<Vec<Dir>>();

	if walkable_dirs.len() != 2 {
		return true
	}

	// if there are exactly two walkable directions, make sure they are not opposite (e.g. a straight corridor)
	!are_opposite(&walkable_dirs[0], &walkable_dirs[1])
}

fn get_graph_from(map: &Map) -> Graph {
	let mut neighbours: HashMap<Pos, HashSet<Pos>> = HashMap::new();
	let mut visited: HashSet<Pos> = HashSet::new();
	let mut queue: Vec<Pos> = vec![map.start];

	while let Some(curr_pos) = queue.pop() {
		if !visited.insert(curr_pos) {
			continue;
		}

		for dir in DIRS {
			let facing_pos_delta = get_pos_delta_from_dir(&dir);
			let mut path_pos = curr_pos;

			loop {
				path_pos += facing_pos_delta;
				if get_pos_symbol(map, &path_pos) == '#' {
					break;
				}

				if is_pos_node(map, path_pos) {
					neighbours.entry(curr_pos).or_default().insert(path_pos);
					queue.push(path_pos);
					break
				}
			}
		}
	}

	Graph {
		start: map.start,
		end: map.end,
		neighbours,
	}
}

fn get_cost_and_new_facing_to(from: &Pos, facing: &Dir, to: &Pos) -> Option<(i32, Dir)> {
	assert_ne!(from, to, "Cannot get cost to same position");

	let pos_delta = *to - *from;
	assert!(pos_delta.0 == 0 || pos_delta.1 == 0, "Can only move in straight lines");

	let required_facing = if pos_delta.0 > 0 {
		Dir::South
	} else if pos_delta.0 < 0 {
		Dir::North
	} else if pos_delta.1 > 0 {
		Dir::East
	} else {
		Dir::West
	};

	if are_opposite(facing, &required_facing) {
		return None // don't bother with U-turns
	}

	let mut cost = pos_delta.0.abs() + pos_delta.1.abs();
	if *facing != required_facing {
		cost += 1000;
	}

	Some((cost, required_facing))
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct State {
	pos: Pos,
	facing: Dir,
	cost: i32,
}

impl PartialOrd<Self> for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cost.cmp(&other.cost)
	}
}

fn get_best_paths(graph: Graph) -> Option<(i32, HashSet<Vec<Pos>>)> {
	let mut heap: BinaryHeap<Reverse<State>> = BinaryHeap::new();
	heap.push(Reverse(State{
		pos: graph.start,
		facing: Dir::East,
		cost: 0
	}));

	let mut cost_to: HashMap<(Pos, Dir), i32> = HashMap::new();
	cost_to.insert((graph.start, Dir::East), 0);

	// for each (pos, facing), store the list of pos/facings that can lead to it with the best cost
	let mut predecessors: HashMap<(Pos, Dir), Vec<(Pos, Dir)>> = HashMap::new();

	while let Some(Reverse(state)) = heap.pop() {
		// if we found a better way to get here
		#[allow(clippy::collapsible_if)]
		if let Some(saved_cost_to_here) = cost_to.get(&(state.pos, state.facing)) {
			if saved_cost_to_here < &state.cost {
				continue;
			}
		}

		let neighbours = graph.neighbours.get(&state.pos).expect("Node not found in graph");
		for neighbour in neighbours {
			let Some((cost, new_facing)) = get_cost_and_new_facing_to(&state.pos, &state.facing, neighbour) else { continue };
			let total_cost_to_neighbour = cost + state.cost;

			// check if this is better than what we have saved
			let pos_facing = (*neighbour, new_facing);
			match cost_to.get(&pos_facing) {
				None => {
					// no saved cost for this pos + facing
					cost_to.insert(pos_facing, total_cost_to_neighbour);
					predecessors.insert(pos_facing, vec![(state.pos, state.facing)]);
					heap.push(Reverse(State{
						pos: *neighbour,
						facing: new_facing,
						cost: total_cost_to_neighbour
					}))
				}
				Some(&saved_cost) if total_cost_to_neighbour < saved_cost => {
					// we have a saved cost, but found a better way there
					cost_to.insert(pos_facing, total_cost_to_neighbour);
					predecessors.insert(pos_facing, vec![(state.pos, state.facing)]);
					heap.push(Reverse(State{
						pos: *neighbour,
						facing: new_facing,
						cost: total_cost_to_neighbour
					}))
				}
				Some(&existing) if total_cost_to_neighbour == existing => {
					// we found a way that has the same cost
					predecessors.entry(pos_facing).or_default().push((state.pos, state.facing));
				}
				_ => {
					// we found a way that has larger cost, ignore it
				}
			}
		}
	}

	let best_cost = DIRS.iter()
		.filter_map(|dir| cost_to.get(&(graph.end, *dir)))
		.min()
		.copied();

	if let Some(best_cost) = best_cost {
		let mut paths = HashSet::new();
		for dir in DIRS {
			// only compute paths with best cost
			let end_pos_dir = (graph.end, dir);
			if let Some(cost) = cost_to.get(&end_pos_dir) && *cost == best_cost {
				compute_paths(&predecessors, &end_pos_dir, &graph.start, &vec![graph.end], &mut paths);
			}
		}
		Some((best_cost, paths))
	} else {
		None
	}
}

fn compute_paths(
	predecessors: &HashMap<(Pos, Dir), Vec<(Pos, Dir)>>,
	curr_pos_dir: &(Pos, Dir),
	goal: &Pos,
	curr_path: &Vec<Pos>,
	paths: &mut HashSet<Vec<Pos>>
) {
	if curr_pos_dir.0 == *goal {
		// we reached the end
		let path = curr_path.clone();
		paths.insert(path);
		return;
	}

	let pos_predecessors = predecessors.get(curr_pos_dir).unwrap_or_else(|| panic!("Could not find a way to {:?}", curr_pos_dir));
	assert!(!pos_predecessors.is_empty(), "Predecessors should not be empty for {:?}", curr_pos_dir);

	for predecessor in pos_predecessors {
		let mut path_fork = curr_path.clone();
		path_fork.push(predecessor.0);
		compute_paths(predecessors, predecessor, goal, &path_fork, paths);
	}
}

fn get_pos_between(a: Pos, b: Pos) -> Vec<Pos>{
	assert_ne!(a, b, "Positions must be different");
	let delta = b - a;
	assert!(delta.0 == 0 || delta.1 == 0, "Positions must be aligned either horizontally or vertically");

	let axis_delta = if delta.0 != 0 {
		Pos(delta.0 / delta.0.abs(), 0)
	} else {
		Pos(0, delta.1 / delta.1.abs())
	};

	let mut curr_pos = a;
	let mut positions = vec![a];
	while curr_pos != b {
		curr_pos += axis_delta;
		positions.push(curr_pos);
	}

	positions
}

fn get_all_pos_in_path(path: &[Pos]) -> HashSet<Pos> {
	assert!(path.len() >= 2, "Path must have at least 2 positions");
	let mut path_iter = path.iter();

	let mut positions = HashSet::new();
	let mut curr_pos = path_iter.next().unwrap();

	for next_pos in path_iter {
		get_pos_between(*curr_pos, *next_pos).iter()
			.for_each(|pos| {
				positions.insert(*pos);
			});

		curr_pos = next_pos;
	}

	positions
}

fn get_all_pos_in_paths(paths: &HashSet<Vec<Pos>>) -> HashSet<Pos> {
	paths.iter()
		.flat_map(|path| get_all_pos_in_path(path).into_iter())
		.collect()
}

fn print_graph_node_neighbours(map: &Map, neighbours: &HashMap<Pos, HashSet<Pos>>) {
	for graph_entry in neighbours {
		println!("{graph_entry:?}");
		let mut modified_map = map.positions.clone();
		modified_map[coord_to_array_idx(graph_entry.0, map.num_columns)] = 'N';

		for neighbour in graph_entry.1 {
			modified_map[coord_to_array_idx(neighbour, map.num_columns)] = 'n';
		}

		for line in modified_map.chunks(map.num_columns as usize) {
			let line_str: String = line.iter().collect();
			println!("{line_str}");
		}
	}
}

fn print_all_graph_nodes(map: &Map, neighbours: &HashMap<Pos, HashSet<Pos>>) {
	let mut modified_map = map.positions.clone();
	for graph_entry in neighbours {
		modified_map[coord_to_array_idx(graph_entry.0, map.num_columns)] = 'N';
	}

	for line in modified_map.chunks(map.num_columns as usize) {
		let line_str: String = line.iter().collect();
		println!("{line_str}");
	}
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	// let input = test_input_3();
	// let input = test_input_4();
	let input = &read_input_file("input.txt");

	println!("Parsing input...");
	let map = parse_input(input);
	println!("Creating graph...");
	let graph = get_graph_from(&map);
	// println!("{:?}", graph.neighbours);
	// print_all_graph_nodes(&map, &graph.neighbours);
	// print_graph_node_neighbours(&map, &graph.neighbours);

	println!("Computing best paths...");
	if let Some((cost, paths)) = get_best_paths(graph) {
		println!("Best cost {}", cost);
		println!("Best paths position count: {:?}", get_all_pos_in_paths(&paths).len());
	} else {
		println!("No path found");
	}
}
