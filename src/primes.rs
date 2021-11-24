
pub fn primes(n_primes: i32) -> Vec<i32> {
	let mut primes_: Vec<i32> = vec![1, 2, 3, 5, 7];
	if n_primes < 5 {
		return primes_;
	}
	let mut i = 8;
	while primes_.len() < n_primes.try_into().unwrap() {
		if i % 2 != 0 && i % 3 != 0 && i % 5 != 0 && i % 7 != 0{
			primes_.push(i);
		}
		i += 1;
	}
	return primes_;
}