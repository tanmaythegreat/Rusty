const NOT_STARTED: usize = 10;
const NUM_STATES: usize = 11;

fn transition(state: usize, d: usize) -> Option<usize> {
	if state == NOT_STARTED {
		Some(if d == 0 { NOT_STARTED } else { d })
	} else if d == state {
		None // two adjacent equal digits -> invalid
	} else {
		Some(d)
	}
}

fn get_ans(n: usize) -> usize {
	let digits: Vec<usize> = convert_to_base(n, 10).unwrap(); // MSB first
	let len = digits.len();

	// g[k][state] = # ways to freely fill k more positions starting from `state`
	let mut g = vec![[0usize; NUM_STATES]; len + 1];
	for s in 0..NUM_STATES { g[0][s] = 1; }
	for k in 1..=len {
		for state in 0..NUM_STATES {
			let mut total = 0;
			for d in 0..10 {
				if let Some(ns) = transition(state, d) {
					total += g[k - 1][ns];
				}
			}
			g[k][state] = total;
		}
	}

	// walk the tight path along the bound's digits
	let mut answer = 0;
	let mut state = NOT_STARTED;
	let mut tight_valid = true;
	for pos in 0..len {
		let bound_digit = digits[pos];
		let remaining = len - pos - 1;
		for d in 0..bound_digit {
			if let Some(ns) = transition(state, d) {
				answer += g[remaining][ns];
			}
		}
		match transition(state, bound_digit) {
			Some(ns) => state = ns,
			None => { tight_valid = false; break; }
		}
	}
	if tight_valid { answer += 1; } // count n itself
	answer
}