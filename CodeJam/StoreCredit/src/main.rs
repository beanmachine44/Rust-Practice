use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;
use std::collections::hash_set::HashSet;

fn price_match(credit: i32, items: Vec<i32>) -> Option<(usize, usize)> {
	let mut item_set: HashSet<i32> = HashSet::new();

	for item in items.iter().cloned() {
		if item_set.contains(&(credit - item)) {
			items.iter().cloned().enumerate().rev()
				.filter(|&x| x.1 ==  item)
				.map(|x| x.0).next();
			return Some(
					(items.iter().cloned().enumerate()
					.filter(|&x| x.1 == credit - item)
					.map(|x| x.0).next().unwrap(),
					items.iter().cloned().enumerate().rev()
					.filter(|&x| x.1 ==  item)
					.map(|x| x.0).next().unwrap())
				);
		}

		item_set.insert(item);
	}

	None
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

	let mut lines = BufReader::new(&fin).lines();
	let mut writer = BufWriter::new(&fout);

	let num_cases = i32::from_str(&lines.next().unwrap().ok().unwrap())
						.ok().expect("Failed to parse cases.");

	for case in (1..num_cases+1) {
		let credit = i32::from_str(&lines.next().unwrap().ok().unwrap())
						.ok().expect(&format!("Failed to parse credit for case #{}.", case));

		let num_of_items = i32::from_str(&lines.next().unwrap().ok().unwrap())
						.ok().expect(&format!("Failed to parse the number of items for case #{}.", case));

		let items: Vec<i32> = lines.next().unwrap().ok().unwrap()
								.split(' ').map(|x| i32::from_str(&x)
									.ok().expect("Failed to parse item.")
								).collect();

		println!("Given ${} credit.", credit);
		println!("Buying 2 items out of {}.", num_of_items);

		match price_match(credit, items) {
			Some((i, j)) =>  {
				println!("Case #{}: {} {}\n", case, i, j);
				writer.write(format!("Case #{}: {} {}\n", case, i+1, j+1).as_bytes());
			},
			None => println!("Case #{}: No solution\n", case)
		};
	}
}
