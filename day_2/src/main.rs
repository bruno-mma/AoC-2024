use std::fs;

#[allow(dead_code)]
fn get_test_reports() -> Vec<Vec<i32>> {
	vec![
		vec![7, 6, 4, 2, 1],
		vec![1, 2, 7, 8, 9],
		vec![9, 7, 6, 2, 1],
		vec![1, 3, 2, 4, 5],
		vec![8, 6, 4, 4, 1],
		vec![1, 3, 6, 7, 9],
	]
}

#[allow(dead_code)]
fn read_reports_from_file(file_name: &str)  -> Vec<Vec<i32>> {
	fs::read_to_string(file_name).unwrap().lines()
		.map(|line| line.split(" "))
		.map(|line_values| line_values.map(|value| value.parse::<i32>().unwrap()))
		.map(|line_values| line_values.collect::<Vec<i32>>())
		.collect()
}

fn check_report(report: &[i32]) -> bool {
	let first_diff = report[0] - report[1];
	if first_diff == 0 || first_diff.abs() > 3 {return false}

	let decreasing = first_diff < 0;

	for i in 1..(report.len() - 1) {
		let diff = report[i] - report[i + 1];
		if diff == 0 || diff.abs() > 3 {return false}
		if decreasing {
			if diff > 0 {return false}
		} else if diff < 0 {return false}
	}

	true
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> usize {
	reports.iter().filter(|report| check_report(report)).count()
}

fn count_safe_reports_tolerant(reports: Vec<Vec<i32>>) -> usize {

	let mut safe_reports = 0;

	for report in reports {
		if check_report(&report) {
			safe_reports += 1
		} else {
			let found_solution = (0..report.len())
				.any(|ignore|
					// This is kinda terrible
					check_report(&report.iter()
						.enumerate()
						.filter(|&(i, _)| i != ignore)
						.map(|(_, &v)| v)
						.collect::<Vec<i32>>()
					)
				);
			safe_reports += if found_solution {1} else {0};
		}
	}

	safe_reports
}

fn main() {
	// let reports = get_test_reports();
	let reports = read_reports_from_file("input.txt");
	println!("Safe reports count: {}", count_safe_reports(reports.clone())); // 402
	println!("Safe reports count tolerant: {}", count_safe_reports_tolerant(reports)); // 455
}
