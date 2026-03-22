use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq)]
struct BinaryIndexedTree<T> where
	T: Default + Copy + PartialEq + PartialOrd +
	Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign  {
	tree: Vec<T>,
	len : usize,
	capacity : usize

}

impl<T> BinaryIndexedTree<T> where
	T: Default + Copy + PartialEq + PartialOrd +
	Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign
{
	///creates a new BinaryIndexTree in which `max_size` many elements can be stored  
	///this cannot be expanded later   
	///this datatype helps in computing prefix sum and sum of elements from index a to b in log time
	pub fn new(max_size: usize) -> Self {
		Self { tree: vec![T::default(); max_size + 1] ,len:0,capacity:max_size}
	}
	///returns the number of elements in the tree
	pub fn len(&self)->usize{
		self.len
	}

	/// return if pushing was successful or not 
	/// it will be unsuccessful if capacity is full
	pub fn push(&mut self,x:T) -> bool{
		if self.len<self.capacity
		{
			self.increment(self.len,x);
			self.len+=1;
			true
		} else {
			false
		}
	}

	///`O(n)' building
	pub fn from(iterator: &Vec<T>,length:usize) -> Self {
		let mut tree = Self { tree: vec![T::default(); length + 1], len: length, capacity: length };
		let mut index:usize = 0;
		for (value) in iterator
		{
			tree.tree[index]+=*value;
			let j :usize=index+index&index.wrapping_neg();
			if j<=tree.len{
				tree.tree[j]+=*value;
			}
			index+=1;
		}
		tree
	}

	///`O(log(n))`  
	/// Zero indexing input
	pub fn increment(&mut self, mut index: usize, delta: T) {
		index += 1;
		while index <= self.capacity {
			self.tree[index] += delta;
			index += index & (index.wrapping_neg());
		}
	}
	///`O(log(n))`
	///Zero indexing input  
	pub fn decrement(&mut self, mut index: usize, delta: T) {
		index += 1;
		while index <= self.capacity {
			self.tree[index] -= delta;
			index += index & (index.wrapping_neg());
		}
	}

	///`O(log(n))`  
	///Zero based indexing  
	/// inclusive both l and r  
	pub fn get_range_sum(&mut self, l: usize, r: usize) -> T {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.prefix_sum(r) - if l==0{T::default()} else{self.prefix_sum(l - 1)}
	}

	///Sum of first k+1 elements
	/// if k == 0 : return first element
	///`O(log(n))`
	pub fn prefix_sum(&mut self, mut k: usize) -> T {
		let mut sum: T = T::default();
		k+=1;
		while k > 0 {
			sum += self.tree[k];
			k -= k & (k.wrapping_neg());
		}
		sum
	}

	/// find smallest index such that prefix sum >= k  
	/// output is Zero based index  
	/// `O(log(n))`
	fn binary_search(&self, mut k: T) -> usize {
		let mut i = 0;
		let mut bit_mask = 1;

		while bit_mask <= self.capacity {
			bit_mask <<= 1;
		}

		let mut step = bit_mask;
		while step > 0 {
			if i + step <= self.capacity && self.tree[i + step] < k {
				k -= self.tree[i + step];
				i += step;
			}
			step >>= 1;
		}
		i // 0-based index
	}

}

struct OrderedSet{
	binary_indexed_tree : BinaryIndexedTree<usize>,
	present : Vec<usize>
}

impl OrderedSet {

	fn new(n: usize) -> Self {
		OrderedSet {
			binary_indexed_tree: BinaryIndexedTree::new(n+1),
			present: vec![0; n+1],
		}
	}
	///Can store numbers including 0 and MAX
	fn from(iterator : Vec<usize>, MAX:usize) -> OrderedSet {
		let mut set = OrderedSet{
			binary_indexed_tree:BinaryIndexedTree::new(MAX+1),
			present: vec![0; MAX+1],
		};
		for i in iterator{
			set.insert(i);
		}set
	}


	fn insert(&mut self, x: usize) {
		self.present[x] += 1;
		self.binary_indexed_tree.increment(x, 1); //+1 so that can also store 0
	}
	///remove only one instance
	fn remove(&mut self, x: usize) ->bool{
		if self.present[x ]>0 {
			self.present[x ] -= 1;
			self.binary_indexed_tree.decrement(x, 1); //+1 so that ,can also store 0
			true
		}else{false}
	}

	/// number of elements < x
	fn order_of_key(&mut self, x: usize) -> usize {
		if x  == 0 {
			0
		} else {
			self.binary_indexed_tree.prefix_sum(x-1)
		}
	}

	// kth smallest (0-based)
	fn find_by_order(&self, k: usize) -> usize {
		self.binary_indexed_tree.binary_search(k+1)
	}
}

#[derive(Debug, Clone, PartialEq)]
struct SegmentTree<T> where
	T: Default + Copy + PartialEq +
	Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign +
	Mul<Output = T> + MulAssign {
	tree: Vec<T>,
	lazy_tree: Vec<T>
}

#[allow(dead_code)]
impl<T> SegmentTree<T> where
	T: Default + Copy + PartialEq +
	Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign +
	Mul<Output = T> + MulAssign + From<i32> 
{
	fn new(a: &Vec<T>) -> Self {
		let n: usize = Self::get_closest_pow2(a.len());
		let mut tree = Self {
			tree: vec![T::default(); 2 * n],
			lazy_tree: vec![T::default(); 2 * n],
		};
		Self::build(1, 0, n - 1, &mut tree.tree, &a);
		tree
	}

	fn get(&mut self, p: usize) -> T {
		self.get_for(p, p)
	}

	fn get_for(&mut self, l: usize, r: usize) -> T {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.get_val(1, 0, self.tree.len() / 2 - 1, l, r)
	}

	fn inc(&mut self, ind: usize, val: T) {
		self.inc_for(ind, ind, val);
	}

	fn inc_for(&mut self, l: usize, r: usize, val: T) {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.inc_seg(1, 0, self.tree.len() / 2 - 1, l, r, val);
	}

	fn inc_seg(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize, val: T) {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return;
		}

		if left >= l && right <= r {
			self.tree[node] += val * ((right - left + 1) as i32).into();
			if left != right {
				self.lazy_tree[2 * node] += val;
				self.lazy_tree[2 * node + 1] += val;
			}
		} else {
			let mid: usize = (left + right) >> 1;
			self.inc_seg(2 * node, left, mid, l, r, val);
			self.inc_seg(2 * node + 1, mid + 1, right, l, r, val);
			self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
		}
	}

	fn get_val(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize) -> T {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return T::default();
		}

		if left >= l && right <= r {
			return self.tree[node];
		}

		let mid: usize = (left + right) >> 1;
		let left_val: T = self.get_val(2 * node, left, mid, l, r);
		let right_val: T = self.get_val(2 * node + 1, mid + 1, right, l, r);
		left_val + right_val
	}

	fn update_lazy(&mut self, node: usize, left: usize, right: usize) {
		if self.lazy_tree[node] != T::default() {
			let lazy_val: T = self.lazy_tree[node];
			self.tree[node] += lazy_val * ((right - left + 1) as i32).into();
			if left != right {
				self.lazy_tree[2 * node] += lazy_val;
				self.lazy_tree[2 * node + 1] += lazy_val;
			}
			self.lazy_tree[node] = T::default();
		}
	}

	fn build(node: usize, left: usize, right: usize, tree: &mut Vec<T>, a: &Vec<T>) {
		if left == right {
			if left <= a.len() - 1 {
				tree[node] = a[left];
			}
		} else {
			let mid: usize = (left + right) >> 1;
			Self::build(2 * node, left, mid, tree, a);
			Self::build(2 * node + 1, mid + 1, right, tree, a);
			tree[node] = tree[2 * node] + tree[2 * node + 1];
		}
	}

	fn get_closest_pow2(val: usize) -> usize {
		if val & (val - 1) == 0 {
			return val;
		}
		(usize::MAX >> val.leading_zeros()) + 1
	}
}

