#![allow(dead_code)]
use std::fs;
use regex::Regex;

fn test_input_1() -> &'static str {
	concat!(
		"XMAS--\n",
		"M---X-\n",
		"A---M-\n",
		"S---A-\n",
		"----S-\n"
	)
}

fn test_input_2() -> &'static str {
	concat!(
		"MMMSXXMASM\n",
		"MSAMXMSMSA\n",
		"AMXSXMAAMM\n",
		"MSAMASMSMX\n",
		"XMASAMXAMM\n",
		"XXAMMXXAMA\n",
		"SMSMSASXSS\n",
		"SAXAMASAAA\n",
		"MAMMMXMMMM\n",
		"MXMXAXMASX\n"
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn count_xmas(input: &str) -> usize {
	let line_len = input.chars().position(|c| c == '\n').unwrap();

	// Since this regex engine does not support lookahead to find overlapping matches, I am forced to do this fuckery
	let regexes: Vec<Regex> = [
		r"^XMAS".to_string(), // right
		format!(r"^X(?:.|\n){{{}}}M(?:.|\n){{{}}}A(?:.|\n){{{}}}S", line_len + 1, line_len + 1, line_len + 1), // down right
		format!(r"^X(?:.|\n){{{}}}M(?:.|\n){{{}}}A(?:.|\n){{{}}}S", line_len, line_len, line_len), // down
		format!(r"^X(?:.|\n){{{}}}M(?:.|\n){{{}}}A(?:.|\n){{{}}}S", line_len - 1, line_len - 1, line_len - 1), // down left
		r"^SAMX".to_string(), // left
		format!(r"^S(?:.|\n){{{}}}A(?:.|\n){{{}}}M(?:.|\n){{{}}}X", line_len + 1, line_len + 1, line_len + 1), // up left
		format!(r"^S(?:.|\n){{{}}}A(?:.|\n){{{}}}M(?:.|\n){{{}}}X", line_len, line_len, line_len), // up
		format!(r"^S(?:.|\n){{{}}}A(?:.|\n){{{}}}M(?:.|\n){{{}}}X", line_len - 1, line_len - 1, line_len - 1) // up right
	].iter()
		.map(|re_pattern| Regex::new(re_pattern).unwrap())
		.collect();

	input.char_indices()
		.map(|(i,_)| i)
		.flat_map(|i| regexes.iter()
			.filter_map(move |re| re.captures(&input[i..]))
		)
		.count()
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	let input = read_input_file("input.txt");

	println!("XMAS count: {}", count_xmas(&input));
}
