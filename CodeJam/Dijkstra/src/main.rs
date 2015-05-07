use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Sign {
    Positive,
    Negative
}

#[derive(Debug, PartialEq)]
enum Component {
    R,
    I,
    J,
    K
}

type Quaternion = (Sign, Component);

fn multiply_quaternion(q1: Quaternion, q2: Quaternion) -> Quaternion {
	use Sign::*;
	use Component::*;

	let mut q = match (q1.1, q2.1) {
			(R, q) => (Positive, q),
			(q, R) => (Positive, q),
			(I, I) => (Negative, R),
			(I, J) => (Positive, K),
			(I, K) => (Negative, J),
			(J, I) => (Negative, K),
			(J, J) => (Negative, R),
			(J, K) => (Positive, I),
			(K, I) => (Positive, J),
			(K, J) => (Negative, I),
			(K, K) => (Negative, R),
	};

	q.0 = match (q1.0, q2.0, q.0) {
		(Positive, Positive, Positive) => Positive,
		(Positive, Negative, Positive) => Negative,
		(Negative, Positive, Positive) => Negative,
		(Negative, Negative, Positive) => Positive,
		(Positive, Positive, Negative) => Negative,
		(Positive, Negative, Negative) => Positive,
		(Negative, Positive, Negative) => Positive,
		(Negative, Negative, Negative) => Negative,
	};

	q
}

fn reduces(string: &str, repeat: usize) -> &'static str {
	let mut i: Quaternion = (Sign::Positive, Component::R);
	let mut j: Quaternion = (Sign::Positive, Component::R);
	for c in string.chars().cycle().take(string.len() * repeat) {
		if i != (Sign::Positive, Component::I) {
			i = match c {
				'i' => multiply_quaternion(i, (Sign::Positive, Component::I)),
				'j' => multiply_quaternion(i, (Sign::Positive, Component::J)),
				'k' => multiply_quaternion(i, (Sign::Positive, Component::K)),
				_ => multiply_quaternion(i, (Sign::Positive, Component::R)),
			}
		} else if j != (Sign::Positive, Component::J) {
			j = match c {
				'i' => multiply_quaternion(j, (Sign::Positive, Component::I)),
				'j' => multiply_quaternion(j, (Sign::Positive, Component::J)),
				'k' => multiply_quaternion(j, (Sign::Positive, Component::K)),
				_ => multiply_quaternion(j, (Sign::Positive, Component::R)),
			}
		} else {
			return "YES";
		}
	}

	"NO"
}


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

	let mut reader 	= BufReader::new(&fin);
	let mut writer 	= BufWriter::new(&fout);
	
	let mut buffer 	= String::new();
	reader.read_line(&mut buffer);
	let num_cases = u64::from_str(buffer.trim())
						.ok().expect("Failed to parse cases.");

	for case in (1..num_cases+1) {
		buffer.clear();
		reader.read_line(&mut buffer);
		let repeat: usize = buffer.trim().split(' ').skip(1).map(|x| usize::from_str(&x)
					.ok().expect("Failed to parse item.")).next().unwrap();
		

		buffer.clear();
		reader.read_line(&mut buffer);
		let string = buffer.trim();
		let mut q: Quaternion = (Sign::Positive, Component::R);
		for c in string.chars() {
			q = match c {
				'i' => multiply_quaternion(q, (Sign::Positive, Component::I)),
				'j' => multiply_quaternion(q, (Sign::Positive, Component::J)),
				'k' => multiply_quaternion(q, (Sign::Positive, Component::K)),
				_ => multiply_quaternion(q, (Sign::Positive, Component::R)),
			}
		}

		print!("Case#{}: {:?} {} = ", case, q, repeat);

		let total = match (q, repeat) {
				((Sign::Negative, Component::R), x) if x % 2 == 1 => -1,
				((_, Component::R), _) => 0,
				((_, _), x) if x % 2 == 0 && x % 4 != 0 => -1,
				_ => 0
			};

		let result: &'static str = if total == -1 {
			reduces(string, repeat)
		} else {
			"NO"
		};

		println!("{:?}", result);
		writer.write(format!("Case #{}: {}\n", case, result).as_bytes());
	}

	writer.flush();
}
