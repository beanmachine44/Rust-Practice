extern crate itertools;
use itertools::Itertools;

use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;

fn main() {
    let path_in = Path::new("src/B-large-practice.in");
	let path_out = Path::new("src/B-large-practice.out");

	println!("Opening file: {}\n", path_in.to_str().unwrap());

	let fin = match File::open(path_in) {
	    Ok(file) => file,
	    Err(error)  => panic!(format!("Error: {}", error)),
	};

	let fout = match OpenOptions::new().create(true).truncate(true).write(true).open(path_out) {
	    Ok(file) => file,
	    Err(error)  => panic!(format!("Error: {}", error)),
	};

	let mut lines = BufReader::new(&fin).lines();
	let mut writer = BufWriter::new(&fout);

	let num_cases = i32::from_str(&lines.next().unwrap().ok().unwrap())
						.ok().expect("Failed to parse cases.");

	for case in (1..num_cases+1) {
		writer.write(format!("Case #{}: {}\n", case, 
			lines.next().unwrap().ok().unwrap()
			.split(' ').rev().join(" ")).as_bytes());
	}

	writer.flush();
}
