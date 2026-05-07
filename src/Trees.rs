use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryIndexedTree<T> where T: Default + Copy + PartialEq + PartialOrd + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign{
	tree: Vec<T>,
	len: usize,
	capacity: usize,
}

impl<T> BinaryIndexedTree<T>
where
	T: Default + Copy + PartialEq + PartialOrd + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign,
{
	/// Creates a new BinaryIndexTree in which `max_size` many elements can be stored.
	/// This cannot be expanded later.
	pub fn new(max_size: usize) -> Self {
		Self {
			tree: vec![T::default(); max_size + 1],
			len: 0,
			capacity: max_size,
		}
	}

	/// Returns the number of elements in the tree.
	pub fn len(&self) -> usize {
		self.len
	}

	/// Returns `true` if pushing was successful, `false` if capacity is full.
	pub fn push(&mut self, x: T) -> bool {
		if self.len < self.capacity {
			self.increment(self.len, x);
			self.len += 1;
			true
		} else {
			false
		}
	}

	/// $O(n)$ building from an existing slice.
	pub fn from_slice(iterator: &[T]) -> Self {
		let length = iterator.len();
		let mut tree = vec![T::default(); length + 1];

		// Copy initial values into the 1-based tree array
		for i in 0..length {
			tree[i + 1] = iterator[i];
		}

		// Ripple the values up the tree in O(n)
		for i in 1..=length {
			let j = i + (i & i.wrapping_neg());
			if j <= length {
				let val = tree[i];
				tree[j] += val;
			}
		}

		Self {
			tree,
			len: length,
			capacity: length,
		}
	}

	/// $O(\log n)$ - Zero-based indexing input.
	pub fn increment(&mut self, mut index: usize, delta: T) {
		index += 1;
		while index <= self.capacity {
			self.tree[index] += delta;
			index += index & index.wrapping_neg();
		}
	}

	/// $O(\log n)$ - Zero-based indexing input.
	pub fn decrement(&mut self, mut index: usize, delta: T) {
		index += 1;
		while index <= self.capacity {
			self.tree[index] -= delta;
			index += index & index.wrapping_neg();
		}
	}

	/// $O(\log n)$ - Zero-based indexing. Inclusive of both l and r.
	pub fn get_range_sum(&self, l: usize, r: usize) -> T {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.prefix_sum(r) - if l == 0 { T::default() } else { self.prefix_sum(l - 1) }
	}

	/// Sum of first k+1 elements. If k == 0: return first element.
	/// $O(\log n)$
	pub fn prefix_sum(&self, mut k: usize) -> T {
		let mut sum: T = T::default();
		k += 1;
		while k > 0 {
			sum += self.tree[k];
			k -= k & k.wrapping_neg();
		}
		sum
	}

	/// Finds the smallest index such that prefix sum >= k.
	/// Output is a Zero-based index. $O(\log n)$
	pub fn binary_search(&self, mut k: T) -> usize {
		let mut i = 0;
		let mut step = 1;

		// Find the largest power of 2 that bounds the capacity
		while step <= self.capacity {
			step <<= 1;
		}

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
struct OrderedMultiSet {
	binary_indexed_tree : BinaryIndexedTree<usize>,
	present : Vec<usize>,
	len:usize
}

impl OrderedMultiSet {
	fn len(self:&Self)->usize{self.len}
	fn new(n: usize) -> Self {
		OrderedMultiSet {
			binary_indexed_tree: BinaryIndexedTree::new(n+1),
			present: vec![0; n+1],len:0
		}
	}
	///Can store numbers including 0 and MAX
	fn from(iterator : Vec<usize>, MAX:usize) -> OrderedMultiSet {
		let mut set = OrderedMultiSet {
			binary_indexed_tree:BinaryIndexedTree::new(MAX+1),
			present: vec![0; MAX+1],len:0
		};
		for i in iterator{
			set.insert(i);
		}set
	}
	/// return true if the element exist in the Set
	fn contains(&self,x:usize)->bool{
		self.present[x]>0
	}
	/// return the number of times the element exist in the Set
	fn count(&self,x:usize)->usize{
		self.present[x]
	}
	fn insert(&mut self, x: usize) {
		self.present[x] += 1;
		self.len+=1;
		self.binary_indexed_tree.increment(x, 1); //+1 so that can also store 0
	}
	///remove only one instance
	fn remove(&mut self, x: usize) ->bool{
		if self.present[x ]>0 {
			self.len-=1;
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

	/// kth smallest (0-based)
	fn find_by_order(&self, k: usize) -> usize {
		self.binary_indexed_tree.binary_search(k+1)
	}
}


struct OrderedMultiMap<T>{
	binary_indexed_tree : BinaryIndexedTree<usize>,
	present : Vec<Vec<T>>
}

impl<T> OrderedMultiMap<T> {

	fn new(n: usize) -> Self {
		OrderedMultiMap {
			binary_indexed_tree: BinaryIndexedTree::new(n+1),
			present: vec![Vec::with_capacity(4); n+1],
		}
	}
	///Can store numbers including 0 and MAX
	fn from(iterator : Vec<(usize,T)>, MAX:usize) -> OrderedMultiMap {
		let mut set = OrderedMultiMap {
			binary_indexed_tree:BinaryIndexedTree::new(MAX+1),
			present: vec![Vec::with_capacity(4); MAX+1],
		};
		for i in iterator{
			set.insert(i.0,i.1);
		}set
	}
	/// return true if the element exist in the Set
	fn contains(&self,x:usize)->bool{
		self.present[x].len()>0
	}
	/// return the number of times the element exist in the Set
	fn count(&self,x:usize)->usize{
		self.present[x].len()
	}
	fn insert(&mut self, key: usize,value:T) {
		self.present[key].push(value);
		self.binary_indexed_tree.increment(key, 1);
	}
	///remove only one instance
	fn remove(&mut self, x: usize) ->Option<T>{
		if self.present[x].len()>0 {
			self.binary_indexed_tree.decrement(x, 1);
			self.present[x].pop();
		}else{None}
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


struct FullOrderedMultiSet<T:Eq+Hash+Copy> {
	binary_indexed_tree : BinaryIndexedTree<usize>,
	present : Vec<usize>, // present[i] represents is Data[i] present in set
	Data : Vec<T>, //sorted vector that containing possible items that can be inserted
	mapping: HashMap<T,usize>, // inverse of Data 
	len : usize
}

impl<T:Eq+Hash+Copy> FullOrderedMultiSet<T> {

	/// new() doesn't add the data,it just says that only this data can be added  
	/// use from() if you want to add data as well  
	/// /// Data[i-1]<Data[i] this is assumed  
	fn new(Data : Vec<T>) -> Self {
		let mut set = FullOrderedMultiSet {
			binary_indexed_tree: BinaryIndexedTree::new(Data.len()),
			present: vec![0; Data.len()],
			mapping : HashMap::with_capacity(Data.len()),
			Data,
			len : 0
		};
		for i in set.Data.iter().enumerate(){
			set.mapping.insert(*i.1,i.0);
		}
		set
	}
	/// same as calling new and then inserting first values of Data   
	/// Data[[i-1]]<Data[[i]] this is assumed  
	fn from(Data : Vec<T>,n : usize ) -> FullOrderedMultiSet<T> {
		let mut set = Self::new(Data);
		for i in 0..n{
			set.present[i] += 1;
			set.len+=1;
			set.binary_indexed_tree.increment(i, 1);
		}
		set
	}
	/// return true if the element exist in the Set
	fn contains(&self,x:&T)->bool{
		self.present[self.mapping[x]]>0
	}
	/// return number of times the element exist in the Set
	fn count(&self,x:&T)->usize{
		self.present[self.mapping[x]]
	}
	/// return if the insertion was successful (element could be inserted already)
	/// panics if the element dont exist in data
	fn insert(&mut self, x:&T){
		let index = self.mapping[x];
		self.len+=1;
		self.present[index] += 1;
		self.binary_indexed_tree.increment(index, 1);
	}
	fn remove(&mut self, x: &T) ->bool{
		let index = self.mapping[x];
		if self.present[index]>0 {
			self.present[index] -= 1;
			self.len-=1;
			self.binary_indexed_tree.decrement(index, 1); //+1 so that ,can also store 0
			true
		}else{false}
	}
	fn len(&self)->usize{
		self.len
	}
	/// number of elements < x
	fn order_of_key(&mut self, x: &T) -> usize {
		let x = self.mapping[x];
		if x  == 0 {
			0
		} else {
			self.binary_indexed_tree.prefix_sum(x-1)
		}
	}

	///kth smallest element in the set 
	/// for k = 0 returns the smallest element
	fn find_by_order(&self, k: usize) -> &T {
		&self.Data[self.binary_indexed_tree.binary_search(k+1)]
	}
}



#[derive(Debug, Clone, PartialEq)]
struct SegmentTree<T,B,C,D> where
	T: Default + Clone,
	B : FnMut(&T, &T) -> T,
	C : FnMut(&T,&T,usize,usize)->T,
	D : FnMut(&Option<T>,&T)->Option<T>
{
	///length of the array  
	n:usize,
	tree: Vec<T>,
	lazy_tree: Vec<Option<T>>,
	combiner : B,
	/// `updater(old_val,update_by_parameter,left,right)->new_val`
	///  
	/// if i want to update the segment information (.i.e segment \[left,right\]) then what will be its new value  
	/// for example if segment tree is for adding things then   
	/// ```
	/// updater = |old_val,update_by_parameter,left,right|  
	/// {  
	///     if left==right{  
	///           old_val+update_by_parameter
	///     }  
	///     else{
	///         old_val+(right-left+1)*update_by_val
	///     }
	/// }
	/// ```  
	/// if it is know that only no range updates will happen then you may ignore the case when `left != right`  
	///
	updater : C,
	///`update_combiner(old_update,update_parameter)->new_update_parameter`  
	/// 
	/// if i want to update a number by old_update  
	/// and then i want to update the same number by update_parameter
	/// then it is the same as updating the orignal number by new_update_parameter
	/// 
	/// # For Example
	/// if the segment stores the sum of elements then 
	/// ```
	/// update_combiner = |old_update,update_parameter|{
	///     if let Some(old) = old_update{
	///         Some(old+update_parameter)
	///     }
	///     else{
	///         Some(update_parameter)
	///     }
	/// }
	/// ```
	/// 
	update_combiner: D

}

#[allow(dead_code)]
impl<T: Default+Clone,B:FnMut(&T, &T) -> T,C: FnMut(&T, &T, usize, usize)->T,D: FnMut(&Option<T>, &T)->Option<T>> SegmentTree<T,B,C,D>
{
	///creates a segment tree from a array.
	/// creates a array called tree and a lazy_tree 
	/// `O(4n)`  
	/// # Parameters
	/// * `a`: The initial data source.
	///
	/// * `combiner`: Defines how two child nodes merge to form a parent. 
	///   (e.g., `|a, b| a + b` for range sums, or `std::cmp::min` for range minimums).
	///
	/// * `updater`: Logic for applying a pending update to a segment.
	///   It maps `(current_segment_value, update_value, left, right)` 
	///   to the `new_segment_value`. This must account for the segment length 
	///   if the update affects all elements (like adding $X$ to every element in a range).
	///
	/// * `update_combiner`: Defines how to merge two "lazy" updates when they overlap.
	///   It takes `(existing_update, incoming_update)` and returns the consolidated 
	///   update that represents applying both in sequence.  
	fn new(a: &Vec<T>,combiner:B,updater:C,update_combiner:D) -> Self {
		let n: usize = Self::get_closest_pow2(a.len());
		let mut tree = Self {
			n:a.len(),
			tree: vec![T::default(); 2 * n],
			lazy_tree: vec![None; 2 * n],combiner,updater,update_combiner
		};
		tree.build(1, 0, n - 1, &a);
		tree
	}
	/// for internal use only  
	/// it recursively builds the node by first building the childs  
	fn build(&mut self,node: usize, left: usize, right: usize, a: &Vec<T>) {
		if left == right {
			if left <= a.len() - 1 {
				self.tree[node] = a[left].clone();
			}
		} else {
			let mid: usize = (left + right) >> 1;
			self.build(2 * node, left, mid, a);
			self.build(2 * node + 1, mid + 1, right, a);
			self.tree[node] = (self.combiner)(&self.tree[2 * node] , &self.tree[2 * node + 1]);
		}
	}
	///gets the immutable value at index p (0-based)
	///`O(logn)`   
	fn get(&mut self, p: usize) -> T {
		self.get_range(p, p)
	}

	/// get the combined information of the range (0-based indexing)  
	/// based on the `self.combiner` provided  
	/// `O(logn)`  
	fn get_range(&mut self, l: usize, r: usize) -> T {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.get_val(1, 0, self.tree.len() / 2 - 1, l, r).unwrap()
	}


	/// get the combined information of the range (0-based indexing)  
	/// based on the custom `combiner` provided  
	/// `O(logn)`  
	fn get_range_custom<U,E:FnMut(&T)->U,F:FnMut(U,U)->U>(&mut self, l: usize, r: usize,mut getter:E,mut combiner:F) -> U {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.get_val_custom(1, 0, self.tree.len() / 2 - 1, l, r,&mut getter,&mut combiner).unwrap()
	}

	/// updates a single element bases on the updater provided    
	/// 0-based indexing  
	fn update(&mut self, ind: usize, val: &T) {
		self.update_range(ind, ind, val);
	}
	/// stores what to update in a lazy tree and apply this update when i try to get this value  
	/// it uses the `update_combiner` and `updater`   
	/// `O(logn)`
	fn update_range(&mut self, l: usize, r: usize, val: &T) {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.update_segment(1, 0, self.tree.len() / 2 - 1, l, r, val);
	}

	/// for internal use only   
	/// if you want to update any thing use `update_range`  or `update`  
	fn update_segment(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize, val: &T) {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return;
		}

		if left >= l && right <= r {
			self.tree[node] = (self.updater)(&self.tree[node],val,left,right);
			if left != right {
				self.lazy_tree[2 * node] = (self.update_combiner)(&self.lazy_tree[2*node],val);
				self.lazy_tree[2 * node + 1] = (self.update_combiner)(&self.lazy_tree[2*node+1],val);
			}
		} else {
			let mid: usize = (left + right) >> 1;
			self.update_segment(2 * node, left, mid, l, r, val);
			self.update_segment(2 * node + 1, mid + 1, right, l, r, val);
			self.tree[node] = (self.combiner)(&self.tree[2 * node] , &self.tree[2 * node + 1]);
		}
	}
	/// for internal use only  
	/// [ l,r ] this range is what i am looking for 
	/// [ left,right ] there is where i am looking for l,r 
	fn get_val(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize) -> Option<T> {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return None;
		}

		if left >= l && right <= r {
			return Some(self.tree[node].clone());
		}

		let mid: usize = (left + right) / 2;
		let left_val = self.get_val(2 * node, left, mid, l, r);
		let right_val = self.get_val(2 * node + 1, mid + 1, right, l, r);
		match (left_val, right_val) {
			(Some(l_val), Some(r_val)) => Some((self.combiner)(&l_val, &r_val)),
			(Some(val), None) | (None, Some(val)) => Some(val),
			(None, None) => None,
		}
	}

	/// for internal use only  
	/// [ l,r ] this range is what i am looking for 
	/// [ left,right ] there is where i am looking for l,r 
	fn get_val_custom<U,E:FnMut(&T)->U,F:FnMut(U,U)->U>(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize,mut getter:&mut E,mut combiner:&mut F) -> Option<U> {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return None;
		}

		if left >= l && right <= r {
			return Some(getter(&self.tree[node]));
		}

		let mid: usize = (left + right) / 2;
		let left_val = self.get_val_custom(2 * node, left, mid, l, r,getter,combiner);
		let right_val = self.get_val_custom(2 * node + 1, mid + 1, right, l, r,getter,combiner);
		match (left_val, right_val) {
			(Some(l_val), Some(r_val)) => Some((combiner)(l_val, r_val)),
			(Some(val), None) | (None, Some(val)) => Some(val),
			(None, None) => None,
		}
	}
	/// internal use only  
	/// applies the lazy effect on the node `node`  
	/// and sets lazy to `None`  
	fn update_lazy(&mut self, node: usize, left: usize, right: usize) {
		if let Some(lazy_val) = self.lazy_tree[node].clone(){
			self.tree[node] = (self.updater)(&self.tree[node],&lazy_val,left,right);
			if left != right {
				self.lazy_tree[2 * node] = (self.update_combiner)(&self.lazy_tree[2*node],&lazy_val);
				self.lazy_tree[2 * node + 1] = (self.update_combiner)(&self.lazy_tree[2*node+1],&lazy_val);
			}
			self.lazy_tree[node] = None;
		}
	}
	fn get_closest_pow2(val: usize) -> usize {
		if val & (val - 1) == 0 {
			return val;
		}
		(usize::MAX >> val.leading_zeros()) + 1
	}

	/// Flushes all lazy updates from the root down to the leaves.
	/// After calling this, the leaf nodes in `self.tree` will contain 
	/// the most up-to-date values.
	/// Complexity: O(N)
	fn get_all_elements(&mut self) -> Vec<T>{
		let n = self.tree.len() / 2;
		let mut V  = vec![T::default();self.n];
		self.collect_values(1, 0, n - 1,&mut V);
		V
	}
	/// internal use only   
	/// calls update_lazy for every node  
	/// overall complexity is `O(n)`  
	fn collect_values(&mut self, node: usize, left: usize, right: usize,arr : &mut Vec<T>) {
		// Apply this node's lazy first, then push down
		self.update_lazy(node, left, right);

		if left == right {
			if let Some(v) = arr.get_mut(left){*v = self.tree[node].clone();}
			return;
		}

		let mid = (left + right) / 2;
		self.collect_values(2 * node, left, mid,arr);
		self.collect_values(2 * node + 1, mid + 1, right,arr);
	}

}
