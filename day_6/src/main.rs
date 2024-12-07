use std::collections::HashSet;

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

fn parse_input(input: &str) -> ((u32, u32), (u32, u32), HashSet<(u32, u32)>) {
	let num_columns = input.chars().position(|c| c == '\n').unwrap() as u32;
	let num_rows = input.lines().count() as u32;

	let guard_pos = input.chars().position(|c| c == '^').unwrap() as u32;
	let guard_pos = (guard_pos / num_rows, guard_pos % (num_columns + 1));

	let obstacles: HashSet<_> = input.lines()
		.enumerate()
		.flat_map(|(row, line)| {
			line.chars().enumerate()
				.map(move |(column, char)| ((row, column), char))
				.filter(|((_row, _column), char)| *char == '#')
				.map(|((row, column), _char)| (row as u32, column as u32))
		})
		.collect();

	((num_rows, num_columns), guard_pos, obstacles)
}

fn main() {
	let input = test_input_1();
	let (size, guard_pos, obstacles) = parse_input(input);

	println!("{:?} {:?} {:?}", size, guard_pos, obstacles)
}
