

// 
pub fn fib(end: i32) -> Vec<i32> {
	let mut a = 1;
	let mut b = 0;
	let mut c;
	let mut result: Vec<i32> = vec![a];

	loop {
		c = b+a;
		b = a;
		a = c;
		if a > end {
			break;
		}
		result.push(a);
	}

	return result;
}
