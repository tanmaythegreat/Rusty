use std::ops::{Add, Div, Sub};

/// Returns the value of the partition point according to the given predicate (the first element of the second partition).
///
/// O(1) space Complexity
/// O(log(n))
/// n is the Range size
/// First partition maps to true
/// Second partition maps to false
/// start and end both inclusive
/// step = 1 (recommended) , for floats step = tolerable error
fn partition_point<T,P>(mut start:T,mut end:T,step:T,mut pred: P)->Result<T,u8>
                        where
	                        P : FnMut(T) -> bool,
	                        T : PartialOrd + Copy
	                        + Add<Output = T>
	                        + Sub<Output = T>
	                        + Div<Output = T>
	                        + From<u8>
{
	if step==T::from(0){return Err(0)}
	let mut ans = end+step;
	while start<=end{
		let mid = start+(end-start)/T::from(2);
		if pred(mid)
		{
			start = mid+step;
		}
		else
		{
			ans = mid;
			if ans==start{break;}
			end = mid-step;
		}
	}
	Ok(ans)
}

///For a function f(x), if there is a single minima in the domain [[start,end]] then
///find_some_minima(start,end,step,f) return the value x such that f(x) is minimum
///step should be greater than 0
/// space complexity O(1)
/// Time Complexity O(log(size))
/// where size is (end-start)/step
fn find_some_minima<T,P,U>(mut start:T, mut end:T, step:T, mut f:P) -> Result<(T,U), u8>
                           where
	                           P : FnMut(T)->U,
	                           U : Sub<Output=U> + PartialOrd + From<u8> + Copy,
	                           T : Copy + Add<Output = T> + Sub<Output = T> + From<u8> + Div<Output=T> + PartialOrd
{
	if step<=T::from(0){return Err(0);}

	let mut initial_point=start+(end-start)/T::from(2);
	let mut F = f(initial_point);
	while start<=end{
		initial_point=start+(end-start)/T::from(2);
		F = f(initial_point);
		let derivative = f(initial_point+step)-F;
		if derivative>=U::from(0){
			end = initial_point-step;
		}
		else{
			start = initial_point+step;
		}
	}
	Ok((initial_point,F))
}

///For a function f(x), if there is a single maxima in the domain [[start,end]] then
///find_some_maxima(start,end,step,f) return the value x such that f(x) is maximum
///step should be greater than 0
/// space complexity O(1)
/// Time Complexity O(log(size))
/// where size is (end-start)/step
fn find_some_maxima<T,P,U>(mut start:T, mut end:T, step:T, mut f:P) -> Result<(T,U), u8>
                           where
	                           P : FnMut(T)->U,
	                           U : Sub<Output=U> + PartialOrd + From<u8> + Copy,
	                           T : Copy + Add<Output = T> + Sub<Output = T> + From<u8> + Div<Output=T> + PartialOrd
{
	if step<=T::from(0){return Err(0);}
	let mut initial_point=start+(end-start)/T::from(2);
	let mut F = f(initial_point);
	while start<=end{
		initial_point=start+(end-start)/T::from(2);
		F = f(initial_point);
		let derivative = f(initial_point+step)-F;
		if derivative>=U::from(0){
			start = initial_point+step;
		}
		else{
			end = initial_point-step;
		}
	}
	Ok((initial_point,F))
}

/// For a function f(x), if there is a single minima in the domain [start, end] then  
/// find_some_minima_discrete(start, end, step, f) returns the value x such that f(x) is minimum  
/// step should be greater than 0  
/// Space complexity O(1)  
/// Time Complexity O(log(size))  
/// where size is (end-start)/step    
/// ternary search each step one-thirds the search space   
fn find_some_minima_discrete<T, P, U>(mut start: T, mut end: T, step: T, mut f: P) -> Result<(T, U), u8>
                                      where
	                                      P: FnMut(T) -> U,
	                                      U: PartialOrd + Copy,
	                                      T: Copy + Add<Output=T> + Sub<Output=T> + From<u8> + Div<Output=T> + PartialOrd,
{
	if step <= T::from(0) { return Err(0); }

	while start < end {
		let diff = (end - start) / T::from(3);

		if diff <= T::from(1) {
			let mut best_x = start;
			let mut best_y = f(start);
			let mut cur = start + step;
			while cur <= end {
				let val = f(cur);
				if val < best_y {
					best_y = val;
					best_x = cur;
				}
				cur = cur + step;
			}
			return Ok((best_x, best_y));
		}

		let m1 = start + diff;
		let m2 = end - diff;

		if f(m1) > f(m2) {
			start = m1 + step;
		} else {
			end = m2 - step;
		}
	}
	let ans = start;
	let val = f(ans);
	Ok((ans, val))
}


/// For a function f(x), if there is a single maxima in the domain \[start, end\] then  
/// find_some_maxima_discrete(start, end, step, f) returns the value x such that f(x) is maximum    
/// step should be greater than 0  
/// Space complexity O(1)  
/// Time Complexity O(log(size))  
/// where size is (end-start)/step    
/// ternary search each step one-thirds the search space   
fn find_some_maxima_discrete<T, P, U>(mut start: T, mut end: T, step: T, mut f: P) -> Result<(T, U), u8>
                                      where
	                                      P: FnMut(T) -> U,
	                                      U: PartialOrd + Copy,
	                                      T: Copy + Add<Output=T> + Sub<Output=T> + From<u8> + Div<Output=T> + PartialOrd,
{
	if step <= T::from(0) { return Err(0); }

	while start < end {
		let diff = (end - start) / T::from(3);

		if diff <= T::from(1) {
			let mut best_x = start;
			let mut best_y = f(start);
			let mut cur = start + step;
			while cur <= end {
				let val = f(cur);
				if val > best_y {  
					best_y = val;
					best_x = cur;
				}
				cur = cur + step;
			}
			return Ok((best_x, best_y));
		}

		let m1 = start + diff;
		let m2 = end - diff;

		if f(m1) < f(m2) {
			start = m1 + step;
		} else {
			end = m2 - step;
		}
	}

	let ans = start;
	let val = f(ans);
	Ok((ans, val))
}

/// choose r elements form `arr`  
fn combinations_iterative<T: Clone>(arr: &[T], r: usize) -> Vec<Vec<T>> {
	let n = arr.len();
	if r > n { return vec![]; }
	if r == 0 { return vec![vec![]]; }

	let mut indices: Vec<usize> = (0..r).collect();
	let mut result = Vec::new();

	loop {
		result.push(indices.iter().map(|&i| arr[i].clone()).collect());
		let mut i = r;

		while i > 0 && indices[i - 1] == i - 1 + n - r {
			i -= 1;
		}

		if i == 0 {
			break;
		}
		indices[i - 1] += 1;

		for j in i..r {
			indices[j] = indices[j - 1] + 1;
		}
	}

	result
}