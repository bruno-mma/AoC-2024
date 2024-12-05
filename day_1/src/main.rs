use std::collections::HashMap;
use std::fs;
use std::iter::zip;

fn read_lists_file(file_name: &str) -> (Vec<u32>, Vec<u32>) {
	let file_path = format!("../{}", file_name);

	fs::read_to_string(file_path).unwrap().lines()
		.map(|line| line.split("   ").collect::<Vec<&str>>())
		.map(|vec_ids| (vec_ids[0], vec_ids[1]))
		.map(|(id_a, id_b)| (id_a.parse::<u32>().unwrap(), id_b.parse::<u32>().unwrap()))
		.unzip()
}

#[allow(dead_code)]
fn test_lists()  -> (Vec<u32>, Vec<u32>) {
	(vec![3, 4, 2, 1, 3, 3],
	 vec![4, 3, 5, 3, 9, 3])
}

fn total_distance(first: &Vec<u32>, second: &Vec<u32>) -> u32 {
	// get sorted copies of lists without modifying original lists
	// let first = BinaryHeap::from(first).into_sorted_vec();
	// let second = BinaryHeap::from(second).into_sorted_vec();

	let mut first = first.clone();
	first.sort_unstable();
	let mut second = second.clone();
	second.sort_unstable();

	zip(first, second)
		.map(|(first_elem, second_elem)| first_elem.abs_diff(second_elem))
		.sum()
}

fn similarity_score(first: &Vec<u32>, second: &Vec<u32>) -> u32 {
	let mut second_elem_count: HashMap<u32, u32> = HashMap::new();
	second.iter().for_each(|elem| *second_elem_count.entry(*elem).or_insert(0) += 1);
	
	first.iter()
		.map(|&id| id * *second_elem_count.entry(id).or_insert(0))
		.sum()
}

fn main() {
	let (first, second) = read_lists_file("input.txt");
	println!("Total distance: {}", total_distance(&first, &second)); // 1506483
	println!("Similarity score: {}", similarity_score(&first, &second)) // 23126924
}
