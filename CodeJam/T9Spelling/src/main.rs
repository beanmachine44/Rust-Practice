use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;

fn main() {
    let path_in = Path::new("src/C-large-practice.in");
	let path_out = Path::new("src/C-large-practice.out");

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
			.chars().map(|x| match x {
				'a' => "2",
				'b' => "22",
				'c' => "222",
				'd' => "3",
				'e' => "33",
				'f' => "333",
				'g' => "4",
				'h' => "44",
				'i' => "444",
				'j' => "5",
				'k' => "55",
				'l' => "555",
				'm' => "6",
				'n' => "66",
				'o' => "666",
				'p' => "7",
				'q' => "77",
				'r' => "777",
				's' => "7777",
				't' => "8",
				'u' => "88",
				'v' => "888",
				'w' => "9",
				'x' => "99",
				'y' => "999",
				'z' => "9999",
				' ' => "0",
				_ => ""
			}).fold(String::new(), |mut fold, part| {
				match (fold.chars().rev().next(), part.chars().next()) {
					(Some(c1), Some(c2)) if c1 == c2 => {
						fold.push(' ');
						fold.push_str(part);
					},
					_ => fold.push_str(part)
				};
				fold
			})).as_bytes());
	}

	writer.flush();
}
