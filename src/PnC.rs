// region Combination
struct Combinations<'a, T> {
	arr: &'a [T],
	indices: Vec<usize>,
	r: usize,
	done: bool,
}

fn combinations_iterative<T: Clone>(arr: &[T], r: usize) -> Combinations<'_, T> {
	let n = arr.len();
	Combinations {
		arr,
		indices: (0..r).collect(),
		r,
		done: r > n,
	}
}

impl<'a, T: Clone> Iterator for Combinations<'a, T> {
	type Item = Vec<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.done {
			return None;
		}

		let result: Vec<T> = self.indices.iter().map(|&i| self.arr[i].clone()).collect();

		if self.r == 0 {
			self.done = true;
			return Some(result);
		}

		let n = self.arr.len();
		let mut i = self.r;
		while i > 0 && self.indices[i - 1] == i - 1 + n - self.r {
			i -= 1;
		}

		if i == 0 {
			self.done = true;
		} else {
			self.indices[i - 1] += 1;
			for j in i..self.r {
				self.indices[j] = self.indices[j - 1] + 1;
			}
		}

		Some(result)
	}
}
// endregion

// region Cartesian Product --> choose one element from each 
struct CartesianProduct<'a, T> {
	vecs: &'a [Vec<T>],
	indices: Vec<usize>,
	done: bool,
}

fn cartesian_product<T: Clone>(vecs: &[Vec<T>]) -> CartesianProduct<'_, T> {
	let done = vecs.iter().any(|v| v.is_empty());
	CartesianProduct {
		vecs,
		indices: vec![0; vecs.len()],
		done,
	}
}

impl<'a, T: Clone> Iterator for CartesianProduct<'a, T> {
	type Item = Vec<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.done {
			return None;
		}

		let result: Vec<T> = self
			.indices
			.iter()
			.zip(self.vecs.iter())
			.map(|(&i, v)| v[i].clone())
			.collect();

		if self.vecs.is_empty() {
			// n == 0: single empty selection, then done
			self.done = true;
			return Some(result);
		}

		// Advance like an odometer, starting from the rightmost slot.
		let mut i = self.vecs.len();
		loop {
			if i == 0 {
				self.done = true;
				break;
			}
			i -= 1;
			self.indices[i] += 1;
			if self.indices[i] < self.vecs[i].len() {
				break;
			}
			self.indices[i] = 0;
			if i == 0 {
				self.done = true;
				break;
			}
		}

		Some(result)
	}
}
// endregion

// region Permutation
struct Permutations<'a, T> {
	arr: &'a [T],
	indices: Vec<usize>,
	done: bool,
}

fn permutations_iterative<T: Clone>(arr: &[T]) -> Permutations<'_, T> {
	let n = arr.len();
	Permutations {
		arr,
		indices: (0..n).collect(),
		done: false,
	}
}

impl<'a, T: Clone> Iterator for Permutations<'a, T> {
	type Item = Vec<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.done {
			return None;
		}

		let result: Vec<T> = self.indices.iter().map(|&i| self.arr[i].clone()).collect();

		let n = self.indices.len();
		if n == 0 {
			self.done = true;
			return Some(result);
		}

		// Find largest i such that indices[i] < indices[i + 1]  (next_permutation)
		let mut i = n - 1;
		while i > 0 && self.indices[i - 1] >= self.indices[i] {
			i -= 1;
		}

		if i == 0 {
			// indices is in fully descending order: this was the last permutation
			self.done = true;
		} else {
			let i = i - 1; // pivot position
			// find rightmost element greater than indices[i]
			let mut j = n - 1;
			while self.indices[j] <= self.indices[i] {
				j -= 1;
			}
			self.indices.swap(i, j);
			self.indices[i + 1..].reverse();
		}

		Some(result)
	}
}
// endregion
