use std::fs;
use regex::Regex;

#[allow(dead_code)]
fn test_input_1() -> &'static str {
	"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
}

#[allow(dead_code)]
fn test_input_2() -> &'static str {
	"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
}

#[allow(dead_code)]
fn test_input_3() -> &'static str {
	"xmul(2,4)&mul[3,7]mul(20,40)!^don't()_mul(5,5)do()mul(69,69)don't()+mul(32,64]do()mul(5,5)mul(5,1)don't()(mul(11,8)undo()?mul(8,5))"
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn sum_of_multiplications(input: &str) -> u32 {
	let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

	re.captures_iter(input)
		.map(|capture| capture.extract::<2>())
		.map(|(_, [a, b])| (a, b))
		.map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
		.map(|(a, b)| a * b)
		.sum()
}

fn sum_of_multiplications_only_allowed_regions(input: &str) -> u32 {
	// regex to get allowed memory regions
	let re_allowed_regions = Regex::new(r"^(?:.|\n)*?don't\(\)|do\(\)(?:.|\n)*?don't\(\)|do\(\)(?:.|\n)?$").unwrap();
	let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

	re_allowed_regions.captures_iter(input)
		.map(|allowed_memory_capture| allowed_memory_capture.get(0).unwrap().as_str())
		.map(|allowed_memory| re_mul.captures_iter(allowed_memory))
		.flat_map(|capture_match| capture_match.into_iter())
		.map(|capture| capture.extract::<2>())
		.map(|(_, [a, b])| (a, b))
		.map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
		.map(|(a, b)| a * b)
		.sum()
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	// let input = test_input_3();
	let input = read_input_file("input.txt");

	println!("Sum of multiplications {}", sum_of_multiplications(&input)); // 188_741_603
	println!("Sum of multiplications only allowed regions {}", sum_of_multiplications_only_allowed_regions(&input)); // 67_269_798
}
