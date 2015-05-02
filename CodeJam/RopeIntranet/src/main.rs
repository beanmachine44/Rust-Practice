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
	
}
