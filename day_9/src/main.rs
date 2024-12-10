#![allow(dead_code)]

use std::fs;
use std::ops::RangeInclusive;

type DiskMap = Vec<Option<usize>>;
type DiskZone = (Option<usize>, usize); // (Option(value), len)

fn test_input_1() -> &'static str {
	"12345"
}

fn test_input_2() -> &'static str {
	"2333133121414131402"
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

fn disk_map_to_string(disk_map: &DiskMap) -> String {
	disk_map.iter().map(|o| o.map_or(".".to_string(), |v| v.to_string())).collect::<String>()
}

fn disk_zones_to_string(zones: &Vec<DiskZone>) -> String {
	zones.iter()
		.flat_map(|(v_o, len)| std::iter::repeat_n(v_o.map_or(".".to_string(), |v| v.to_string()), *len))
		.collect()
}

fn parse_input(input: &str) -> DiskMap {
	input.chars().enumerate()
		.filter(|(_i, c)| c.is_ascii_digit())
		.flat_map(|(i, c)| {
			let number = c.to_digit(10).unwrap();
			let value = if i % 2 == 0 { Some(i / 2) } else { None };
			std::iter::repeat(value).take(number as usize)
		})
		.collect()
}

fn get_disk_map_zones(map: &DiskMap) -> Vec<DiskZone> {

	let mut zones = Vec::new();
	let mut current_zone_value = map[0];
	let mut current_zone_start = 0;

	for (i, id_o) in map.iter().enumerate().skip(1) {
		if *id_o != current_zone_value {
			zones.push((current_zone_value, i - current_zone_start));

			current_zone_value = *id_o;
			current_zone_start = i;
		}
	}

	zones.push((current_zone_value, map.len() - current_zone_start));
	zones
}

fn find_last_block_index(map: &DiskMap, search_range: RangeInclusive<usize>) -> Option<usize> {
	map[search_range].iter().rposition(|o| o.is_some())
}

fn compact_disk_blocks(input: &DiskMap) -> DiskMap {
	let mut disk_map = input.clone();
	let mut last_block_search_start_index = input.len() - 1;

	for (i, id_o) in input.iter().enumerate() {
		if i >= last_block_search_start_index {break}
		if id_o.is_some() {continue}

		let last_block_index = find_last_block_index(&disk_map, i..=last_block_search_start_index);
		if last_block_index.is_none() {break}
		let last_block_index = last_block_index.unwrap() + i; // we have to add i to the result because we are searching in a slice of the original array

		disk_map.swap(i, last_block_index);
		last_block_search_start_index = last_block_index
	}

	disk_map
}

fn compact_disk_files(input: &Vec<DiskZone>) -> Vec<DiskZone> {
	let mut disk_zones = input.clone();

	for zone in input.iter().rev().filter(|(o, _)| o.is_some()) {
		let available_zone_o = disk_zones.iter().enumerate()
			.find(|(_, (v_o, len))| v_o.is_none() && len >= &zone.1);

		if let Some((i_available, available_zone)) = available_zone_o {
			let old_zone_pos = disk_zones.iter().position(|(v_o, _)| *v_o == zone.0).unwrap(); // this is pretty terrible
			if old_zone_pos < i_available {continue} // I'm not sure why this works
			disk_zones[i_available] = (None, available_zone.1 - zone.1);

			disk_zones[old_zone_pos] = (None, zone.1);
			disk_zones.insert(i_available, *zone);
		}
	}

	disk_zones
}


fn compute_disk_map_checksum(input: &DiskMap) -> usize {
	input.iter()
		.enumerate()
		.map(|(i, id)| id.map_or(0, |v| v * i))
		.sum()
}

fn compute_disc_zones_checksum(input: &Vec<DiskZone>) -> usize {
	input.iter()
		.flat_map(|(v_o, len)| std::iter::repeat_n(v_o, *len))
		.enumerate()
		.map(|(i, id)| id.map_or(0, |v| v * i))
		.sum()
}

fn main() {
	// let input = test_input_1();
	// let input = test_input_2();
	let input = &read_input_file("input.txt");
	let disk_map = parse_input(input);
	let disk_zones = get_disk_map_zones(&disk_map);

	let compacted_disk_map = compact_disk_blocks(&disk_map);
	println!("Compacted disk blocks checksum: {:?}", compute_disk_map_checksum(&compacted_disk_map));
	let compacted_disk_files = compact_disk_files(&disk_zones);
	println!("Compacted disk files checksum: {:?}", compute_disc_zones_checksum(&compacted_disk_files));
}
