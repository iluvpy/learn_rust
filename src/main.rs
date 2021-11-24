mod fib;
mod primes;

fn main() {
	println!("fibonacci!");
	let vec = fib::fib(100);
	for i in vec.iter() {
		println!("{}", i);
	}
	println!("primes!");
	let primes = primes::primes(100000);
	for i in primes.iter() {
		println!("{}", i);
	}
}
