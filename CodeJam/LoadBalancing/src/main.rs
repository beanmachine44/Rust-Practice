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

	let mut reader 	= BufReader::new(&fin);
	let mut writer 	= BufWriter::new(&fout);
	
	let mut buffer 	= String::new();
	reader.read_line(&mut buffer);
	let num_cases = u64::from_str(buffer.trim())
						.ok().expect("Failed to parse cases.");

	for case in (1..num_cases+1) {
		buffer.clear();
		reader.read_line(&mut buffer);
		let data: Vec<u64> = buffer.trim().split(' ').map(|x| u64::from_str(&x)
					.ok().expect("Failed to parse item.")).collect();

		let l = data[0] as f32;
		let p = data[1] as f32;
		let c = data[2] as f32;
		let count = match (p/l).log(c).log2().ceil() {
			x if x > 0f32 => x,
			_ => 0f32
		};


		writer.write(format!("Case #{}: {}\n", case, 
			count).as_bytes());
	}

	writer.flush();
}
