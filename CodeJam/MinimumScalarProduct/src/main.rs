use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;

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

	let mut lines = BufReader::new(&fin).lines();
	let mut writer = BufWriter::new(&fout);

	let num_cases = i64::from_str(&lines.next().unwrap().ok().unwrap())
						.ok().expect("Failed to parse cases.");

	for case in (1..num_cases+1) {
		let num_of_ints = i64::from_str(&lines.next().unwrap().ok().unwrap())
						.ok().expect(&format!("Failed to parse number of ints for case #{}.", case));

		let mut x: Vec<i64> = lines.next().unwrap().ok().unwrap()
								.split(' ').map(|x| i64::from_str(&x)
									.ok().expect("Failed to parse item.")
								).collect();

		let mut y: Vec<i64> = lines.next().unwrap().ok().unwrap()
								.split(' ').map(|x| i64::from_str(&x)
									.ok().expect("Failed to parse item.")
								).collect();

		// Sort x in ascending order and y in descending order
		x.sort_by(|a, b| a.cmp(b));
		y.sort_by(|a, b| b.cmp(a));

		writer.write(format!("Case #{}: {}\n", case,
					x.iter().zip(y.iter())
					.fold(0, |sum, item| sum + item.0*item.1)
				).as_bytes());
	}

	writer.flush();
}
