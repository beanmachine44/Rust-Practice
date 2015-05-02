use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;

fn num_of_intersections(lines: Vec<(u64, u64)>) -> u64 {
	let mut count = 0;

	for (i, line_a) in (1..).zip(lines.iter()) {
		for line_b in lines.iter().skip(i) {
			match (line_a.0 > line_b.0) ^ (line_a.1 > line_b.1) {
				true => count += 1,
				false => {}
			};
		}
	}

	count
}

fn main() {
	let path_in = Path::new("src/A-large-practice.in");
	let path_out = Path::new("src/A-large-practice.out");

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
	
	let mut line 	= String::new();
	reader.read_line(&mut line);
	let num_cases = u64::from_str(line.trim())
						.ok().expect("Failed to parse cases.");

	for case in (1..num_cases+1) {
		let mut number = String::new();
		reader.read_line(&mut number);
		let num_of_items = u64::from_str(number.trim())
						.ok().expect(&format!("Failed to parse the number of items for case #{}.", case));

		let mut wires: Vec<(u64, u64)> = Vec::with_capacity(num_of_items as usize);

		for _ in (0..num_of_items) {
			let mut wire = String::new();
			reader.read_line(&mut wire);
			let mut points = wire.trim().split(' ').map(|x| u64::from_str(&x)
								.ok().expect("Failed to parse item."));

			wires.push((points.next().unwrap(), points.next().unwrap()));
		}

		writer.write(format!("Case #{}: {}\n", case, 
			num_of_intersections(wires)).as_bytes());
	}

	writer.flush();
}
