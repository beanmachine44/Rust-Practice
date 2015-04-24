fn main() {
	let limit = 4000000;
	let mut fib: Vec<u32> = vec![1,2];

	while fib[fib.len() - 1] <= limit {
		let next = fib[fib.len() - 1] + fib[fib.len() - 2];
		fib.push(next);
	}

	println!("Sum of even fib numbers up to 4000000: {}", 
		fib.iter()
		.filter(|&x| x % 2 == 0)
		.fold(0, |sum, x| sum + x));
}
