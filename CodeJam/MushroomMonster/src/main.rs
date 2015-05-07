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

	println!("\nOpening file: {}\n", path_in.to_str().unwrap());

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
	let num_cases = usize::from_str(buffer.trim())
						.ok().expect("Failed to parse cases.");

	for case in (1..num_cases+1) {
		buffer.clear();
		reader.read_line(&mut buffer);
		let num: usize = usize::from_str(buffer.trim())
						.ok().expect("Failed to parse cases.");

		buffer.clear();
		reader.read_line(&mut buffer);
		let counts: Vec<isize> = buffer.trim().split(' ').map(|x| 
			isize::from_str(x).ok().expect("Failed to parse count.")).collect();

		let diffs = counts.iter().zip(counts.iter().skip(1))
						.map(|(x, y)| x - y).filter(|&x| x > 0);

		let mut min_consumed = 0;
		let mut max_diff = 0;
		for x in diffs {
			min_consumed += x;

			if x > max_diff {
				max_diff = x;
			}
		}

		let const_consumed = counts.iter().take(num - 1).map(|&x| if x > max_diff {
			max_diff
		} else {
			x
		}).fold(0, |sum, x| sum + x);

		writer.write(format!("Case #{}: {} {}\n", 
			case, 
			min_consumed, 
			const_consumed).as_bytes());
	}

	writer.flush();
}
