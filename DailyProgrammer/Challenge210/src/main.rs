use std::io;
use std::io::Write;
use std::string::String;

fn loopy_robot(cmd: &String) -> i8 {
	// Position state of robot
	let mut x: i8 = 0;
	let mut y: i8 = 0;
	let mut d: i8 = 0;

	// Parse the command passed in for a single cycle
	for c in cmd.chars() {
		if c == 'S' {
			if (d % 2) == 0 {
				y += 1 - d;
			} else {
				x += 2 - d;
			}
		} else if c == 'L' {
			d = (d + 3) % 4;
		} else if c == 'R' {
			d = (d + 1) % 4;
		} else if c != '\n' {
			println!("Invalid Character {}.\n", c);
			return -1;
		}
	}

	// Match the end of one cycle to the amount of cycles
	// needed to loop to the initial (0, 0, 0) position.
	match (x, y, d) {
		(0, 0, 0) => 1,
		(_, _, 2) => 2,
		(_, _, dir) if dir == 1 || dir == 3 => 4,
		(_, _, _) => -1
	}
}

fn main() {
	// Access STD In
	let mut reader = io::stdin();

	// Read in a Command Sequence for the robot
	print!("Enter Command Sequence: ");
	io::stdout().flush().ok().expect("Failed to flush STD Out.");
	let mut input = String::new();
	let _ = reader.read_line(&mut input).ok().expect("Failed to read line.");

	// Clean the input to get rid of the new lines
	let cleaned_len = input.len() - 2;
	input.truncate(cleaned_len);

	match loopy_robot(&input) {
		-1 => println!("No loop found for command {}.\n", input),
		x => println!("Loop found in {} cycles for command {}.\n", x, input)
	}
}
