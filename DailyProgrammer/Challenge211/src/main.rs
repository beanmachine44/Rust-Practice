#![feature(advanced_slice_patterns, slice_patterns)]

extern crate rand;
use rand::Rng;
use rand::SeedableRng;
use rand::XorShiftRng;

fn fitness(chromosome: &[u8]) -> i8 {
	if chromosome.len() < 2 {
		return 0;
	}

	let mut score = match chromosome {
		[x, y, ..] if x > y => 1,
		[x, y, ..] if x == y => 0,
		_ => -1,
	};

	for i in (1..chromosome.len()-1) {
		score += if chromosome[i-1] < chromosome[i] && chromosome[i] > chromosome[i+1] {
			1
		} else if chromosome[i-1] < chromosome[i] && chromosome[i] > chromosome[i+1] {
			-1
		} else {
			0
		}

		// score += match chromosome[i-1 .. i+1] {
		// 	[x, y, z] if x < y && y > z => 1,
		// 	[x, y, z] if x > y && y < z => -1,
		// 	_ => 0
		// }
	}

	score + match chromosome {
		[.., x, y] if x < y => 1,
		[.., x, y] if x == y => 0,
		_ => -1,
	}
}

fn genetic_algorithm(input: Vec<u8>) -> (Vec<u8>, i8) {
	let rng = XorShiftRng::from_seed([2423412,423423,764563,348356]);
	let population: Vec<(&mut [u8], i8)> = Vec::with_capacity(64);

	for i in (0..population.capacity()/8) {
		population.push((&mut input, 0));
		rng.shuffle(population[i].0);
		population[i].1 = fitness(population[i].0);
	}


	(input, 0)
}


fn main() {
	let v = vec![1,2,3,4,5];

	let rng = XorShiftRng::from_seed([2423412,423423,764563,348356]);
}
