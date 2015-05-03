extern crate regex;

use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;
use regex::Regex;

fn calculate_path(cmd: &str) -> Vec<(isize, isize)> {
	let mut path = Vec::new();
	let mut dir = 0;

	let re = Regex::new(r"[:digit:]+|[:alpha:]").unwrap();
	for cap in re.captures_iter(cmd) {
		match cap.at(0).unwrap_or("") {
			"l" => dir = (dir + 3) % 4,
			"r" => dir = (dir + 1) % 4,
			""  => {},
			num => path.push((dir, isize::from_str(num).ok().unwrap()))
		};
	}

	path
}

fn valid_path(x: isize, y: isize, dir: isize, path: &[(isize, isize)],
	map: &[Vec<char>]) -> Option<((isize, isize), (isize, isize))> {
	let mut x_end: usize = x as usize;
	let mut y_end: usize = y as usize;

	for &(d, len) in path {
		let direction = (dir + d) % 4;

		for _ in (0..len) {
			match direction {
				0 => y_end -= 1,
				1 => x_end += 1,
				2 => y_end += 1,
				3 => x_end -= 1,
				_ => {}
			};

			if map[y_end][x_end] != ' ' {
				return None;
			}
		}
	}

	Some(((x, y), (x_end as isize, y_end as isize)))
}

fn main() {
	let path_in = Path::new("src/maze.in");
	let path_out = Path::new("src/paths.out");

	println!("Opening file: {}\n", path_in.to_str().unwrap());

	let fin = match File::open(path_in) {
	    Ok(file) => file,
	    Err(error)  => panic!(format!("Error: {}", error)),
	};

	let fout = match OpenOptions::new().create(true).truncate(true).write(true).open(path_out) {
	    Ok(file) => file,
	    Err(error)  => panic!(format!("Error: {}", error)),
	};

	let mut reader 	= BufReader::new(&fin);
	let mut writer 	= BufWriter::new(&fout);
	
	let mut buffer 	= String::new();
	reader.read_line(&mut buffer);
	let num_of_lines = u64::from_str(buffer.trim())
						.ok().expect("Failed to parse cases.");

	let mut walls: Vec<Vec<char>> = Vec::new();
	let mut holes: Vec<(isize, isize)> = Vec::new();
	let mut good_paths: Vec<((isize, isize), (isize, isize))> = Vec::new();
	for y in (0..num_of_lines) {
		buffer.clear();
		reader.read_line(&mut buffer);
		walls.push(buffer.trim().chars().collect());
		for (x, _) in (0..).zip(buffer.trim().chars()).filter(|c| c.1 == ' ') {
			holes.push((x as isize, y as isize));
		}
	}

	buffer.clear();
	reader.read_line(&mut buffer);
	let path = calculate_path(buffer.trim());

	for (x, y) in holes {
		for start_dir in (0..4) {
			match valid_path(x, y, start_dir, &path, &walls) {
				Some((start, end)) => {
					good_paths.push((start, end))
				},
				None => {}
			};
		}
	}

	for (start, end) in good_paths {
		writer.write(format!("From {:?} to {:?}\n",
			start, end).as_bytes());
	}

	writer.flush();
}
