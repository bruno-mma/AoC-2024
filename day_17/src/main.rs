use std::fs;
use regex::Regex;

const INPUT_PATTERN: &str = r"^Register A: (?P<a>\d+)\nRegister B: (?P<b>\d+)\nRegister C: (?P<c>\d+)\n\nProgram: (?P<p>(?:\d,?)+)";

fn test_input_1() -> &'static str {
	concat!(
		"Register A: 729\n",
		"Register B: 0\n",
		"Register C: 0\n",
		"\n",
		"Program: 0,1,5,4,3,0\n"
	)
}

fn read_input_file(file_name: &str) -> String {
	fs::read_to_string(file_name).unwrap()
}

#[derive(Debug)]
struct Computer {
	reg_a: u32,
	reg_b: u32,
	reg_c: u32,
	instructions: Vec<u8>,
	inst_pointer: u32,
	output: Vec<u8>,
}

impl Computer {
	fn get_instruction(&self, offset: u32) -> Result<u8, &'static str> {
		self.instructions
			.get((self.inst_pointer + offset) as usize)
			.ok_or("instruction pointer out of bounds")
			.copied()
	}

	fn get_combo_operand_value(&self, operand: u8) -> u32 {
		if operand <= 3 {
			operand as u32
		} else if operand == 4 {
			self.reg_a
		} else if operand == 5 {
			self.reg_b
		} else if operand == 6 {
			self.reg_c
		} else {
			panic!("Invalid operand value: {}", operand);
		}
	}

	fn run_adv_instruction(&mut self) -> Result<(), &'static str> {
		let combo_operand = self.get_combo_operand_value(
			self.get_instruction(1)?
		);

		self.reg_a = self.reg_a / 2_u32.pow(combo_operand);

		self.inst_pointer += 2;
		Ok(())
	}

	fn run_bxl_instruction(&mut self) -> Result<(), &'static str> {
		self.reg_b = self.reg_b ^ self.get_instruction(1)? as u32;

		self.inst_pointer += 2;
		Ok(())
	}

	fn run_bst_instruction(&mut self) -> Result<(), &'static str> {
		let combo_operand = self.get_combo_operand_value(
			self.get_instruction(1)?
		);

		self.reg_b = combo_operand % 8;

		self.inst_pointer += 2;
		Ok(())
	}

	fn run_jnz_instruction(&mut self) -> Result<(), &'static str> {
		if self.reg_a != 0 {
			self.inst_pointer = self.get_instruction(1)? as u32;
		} else {
			self.inst_pointer += 2;
		}

		Ok(())
	}

		fn run_bxc_instruction(&mut self) -> Result<(), &'static str> {
		// The operand for BXC is intentionally ignored as per the instruction specification.
		self.get_instruction(1)?;

		self.reg_b = self.reg_b ^ self.reg_c;

		self.inst_pointer += 2;
		Ok(())
	}

	fn run_out_instruction(&mut self) -> Result<(), &'static str> {
		let combo_operand = self.get_combo_operand_value(
			self.get_instruction(1)?
		);

		self.output.push((combo_operand % 8) as u8);

		self.inst_pointer += 2;
		Ok(())
	}

	fn run_bdv_instruction(&mut self) -> Result<(), &'static str> {
		let combo_operand = self.get_combo_operand_value(
			self.get_instruction(1)?
		);

		self.reg_b = self.reg_a / 2_u32.pow(combo_operand);

		self.inst_pointer += 2;
		Ok(())
	}

	fn run_cdv_instruction(&mut self) -> Result<(), &'static str> {
		let combo_operand = self.get_combo_operand_value(
			self.get_instruction(1)?
		);

		self.reg_c = self.reg_a / 2_u32.pow(combo_operand);

		self.inst_pointer += 2;
		Ok(())
	}

	fn run_instruction(&mut self) -> Result<(), &'static str> {
		let opcode = self.get_instruction(0)?;
		match opcode {
		    0 => self.run_adv_instruction()?,
			1 => self.run_bxl_instruction()?,
			2 => self.run_bst_instruction()?,
			3 => self.run_jnz_instruction()?,
			4 => self.run_bxc_instruction()?,
			5 => self.run_out_instruction()?,
			6 => self.run_bdv_instruction()?,
			7 => self.run_cdv_instruction()?,
			_ => panic!("Unknown opcode: {}", opcode),
		};

		Ok(())
	}

	fn run_program(&mut self) {
		loop {
			if self.run_instruction().is_err() {
				return;
			}
		}
	}

	fn get_output_string(&self) -> String {
		self.output.iter()
			.map(|n| n.to_string())
			.collect::<Vec<_>>()
			.join(",")
	}
}

fn parse_input(input: &str) -> Computer {
	let re = Regex::new(INPUT_PATTERN).unwrap();
	let cap = re.captures(input).expect("Input did not match expected format");

	let instructions = cap.name("p").unwrap().as_str().split(",")
		.map(|c| c.parse().expect("Could not parse number in program instructions"))
		.collect::<Vec<u8>>();

	assert!(!instructions.is_empty(), "Program instructions cannot be empty");

	Computer{
		reg_a: cap.name("a").unwrap().as_str().parse().expect("Could not parse number for register A"),
		reg_b: cap.name("b").unwrap().as_str().parse().expect("Could not parse number for register B"),
		reg_c: cap.name("c").unwrap().as_str().parse().expect("Could not parse number for register C"),
		instructions,
		inst_pointer: 0,
		output: Vec::new(),
	}
}

fn main() {
	// let input = test_input_1();
	let input = &read_input_file("input.txt");

	println!("Parsing program...");
	let mut computer = parse_input(input);
	// println!("{:?}", computer);
	println!("Running program...");
	computer.run_program();
	println!("Done, output: {}", computer.get_output_string())
}
