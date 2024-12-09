#![allow(dead_code)]

use std::fs;
use std::ops::RangeInclusive;

fn test_input_1() -> &'static str {
	"12345"
}

fn test_input_2() -> &'static str {
	"2333133121414131402"
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

type DiskMap = Vec<Option<u32>>;

fn disk_map_to_string(disk_map: &DiskMap) -> String {
	disk_map.iter().map(|o| o.map_or(".".to_string(), |v| v.to_string())).collect::<String>()
}

fn parse_input(input: &str) -> DiskMap {
	input.chars().enumerate()
		.filter(|(_i, c)| c.is_ascii_digit())
		.flat_map(|(i, c)| {
			let number = c.to_digit(10).unwrap();
			let value = if i % 2 == 0 { Some((i / 2) as u32) } else { None };
			std::iter::repeat(value).take(number as usize)
		})
		.collect()
}

fn find_last_block_index(map: &DiskMap, search_range: RangeInclusive<usize>) -> Option<usize> {
	map[search_range].iter().rposition(|o| o.is_some())
}

fn compact_disk(input: &DiskMap) -> DiskMap {
	let mut disk_map = input.clone();
	let mut last_block_search_start_index = input.len() - 1;

	for (i, id_o) in input.iter().enumerate() {
		if i >= last_block_search_start_index {break}
		if id_o.is_some() {continue}

		let last_block_index = find_last_block_index(&disk_map, i..=last_block_search_start_index);
		if last_block_index.is_none() {break} // no more empty blocks, we are done
		let last_block_index = last_block_index.unwrap() + i; // we have to add i to the result because we are searching in a slice of the original array

		disk_map.swap(i, last_block_index);
		last_block_search_start_index = last_block_index
	}

	disk_map
}

fn compute_checksum(input: &DiskMap) -> usize {
	input.iter()
		.enumerate()
		.filter(|(_i, id)| id.is_some())
		.map(|(i, id)| i * (id.unwrap() as usize))
		.sum()
}

fn main() {
	// let input = test_input_2();
	let input = &read_input_file("input.txt");
	let disk_map = parse_input(input);

	let compacted_disk_map = compact_disk(&disk_map);
	println!("{:?}", compute_checksum(&compacted_disk_map));
}
