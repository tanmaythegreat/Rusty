pub fn radix_sort<T, F, const BASE: usize, const MAX_VALUE: usize>(
	arr: &mut [T],
	key_fn: F,
) where
		T: Copy,
		F: Fn(&T) -> usize,
{
	assert!(BASE >= 2, "BASE must be at least 2");

	let n = arr.len();
	if n <= 1 {
		return;
	}

	// ilog(base) gives floor(log_base(MAX_VALUE)); +1 converts "index of
	// highest digit" into "count of digits". ilog panics on 0, so handle
	// that case separately (0 always needs exactly 1 pass).
	let passes = if MAX_VALUE == 0 {
		1
	} else {
		MAX_VALUE.ilog(BASE) as usize + 1
	};

	let mut buf_a: Vec<T> = arr.to_vec();
	let mut buf_b: Vec<T> = arr.to_vec();
	let mut src: &mut [T] = &mut buf_a;
	let mut dst: &mut [T] = &mut buf_b;

	let mut place_value: usize = 1;
	let mut counts:[usize;BASE];

	for _ in 0..passes {
		counts = [0usize; BASE];

		for item in src.iter() {
			let digit = (key_fn(item) / place_value) % BASE;
			counts[digit] += 1;
		}

		let mut total = 0;
		for c in counts.iter_mut() {
			let cnt = *c;
			*c = total;
			total += cnt;
		}

		for item in src.iter() {
			let digit = (key_fn(item) / place_value) % BASE;
			dst[counts[digit]] = *item;
			counts[digit] += 1;
		}

		std::mem::swap(&mut src, &mut dst);
		place_value *= BASE;
	}

	arr.copy_from_slice(src);
}