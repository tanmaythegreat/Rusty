use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// region Ordered Set
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
// endregion

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

// region Segment Tree
#[derive(Debug, Clone, PartialEq)]
struct SegmentTree<T,B,C,D,V,E> where
	T: Clone,
	V: Clone,
	B : FnMut(&T, &T) -> T,
	C : FnMut(&T,&V,usize,usize)->T,
	D : FnMut(&Option<V>,&V)->Option<V>,
	E : FnMut(&V,usize,usize,usize,usize)->V
{
	///length of the array  
	n:usize,
	/// next power-of-two padding size (cached to avoid recomputing)
	n_padded: usize,
	/// flat array of (val, lazy) pairs — one allocation, adjacent in memory
	// CHANGE: single Vec<(T, Option<T>)> instead of two separate Vecs.
	//         val and lazy for node i now live at nodes[i].0 / nodes[i].1 —
	//         adjacent bytes, one cache-line fetch covers both.
	nodes: Vec<(T, Option<V>)>,
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
	update_combiner: D,
	/// if a node get updated by `update_parameter` `val` then the left child gets 
	/// updated by `update_var_propogate(&val,left,mid,left,right)` and the right 
	/// child get updated by `update_var_propogate(&val,mid+1,right,left,right)`
	update_val_propagate:E,
	t_default:T
}

#[allow(dead_code)]
impl<V:Clone,T: Clone,B:FnMut(&T, &T) -> T,C: FnMut(&T, &V, usize, usize)->T,D: FnMut(&Option<V>, &V)->Option<V>,E:FnMut(&V,usize,usize,usize,usize)->V> SegmentTree<T,B,C,D,V,E>
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
	/// 
	/// * `update_val_propagate`: Defines how the child node should update if the parent 
	///   node updates by `val`.   
	///   `update_val_propagate(&val,child_left,child_right,parent_left,parent_right)->child_val`  
	///   
	fn new(a: &Vec<T>, combiner:B, updater:C, update_combiner:D,update_val_propagate:E, t_default:T) -> Self {
		// CHANGE: a.len().next_power_of_two() — single lzcnt/bsr instruction,
		//         replaces the custom 8-line bit-twiddling helper.
		//         Also safe for a.len() == 0 (returns 1).
		let n_padded = a.len().next_power_of_two();

		let mut nodes: Vec<(T, Option<V>)> = vec![(t_default.clone(), None); 2 * n_padded];

		// Fill leaves directly (iterative, no recursion).
		for (i, v) in a.iter().enumerate() {
			nodes[n_padded + i].0 = v.clone();
		}

		let mut tree = Self {
			n:a.len(),
			n_padded,
			nodes,
			combiner,
			updater,
			update_combiner,
			update_val_propagate,
			t_default
		};

		// Build internal nodes bottom-up.
		// CHANGE: iterative — avoids O(n) stack frames of the old recursive build().
		for i in (1..n_padded).rev() {
			let left_val  = tree.nodes[2 * i].0.clone();
			let right_val = tree.nodes[2 * i + 1].0.clone();
			tree.nodes[i].0 = (tree.combiner)(&left_val, &right_val);
		}

		tree
	}

	///gets the immutable value at index p (0-based)
	///`O(logn)`   
	#[inline]
	fn get(&mut self, p: usize) -> T {
		self.get_range(p, p)
	}

	/// get the combined information of the range (0-based indexing)  
	/// based on the `self.combiner` provided  
	/// `O(logn)`  
	#[inline]
	fn get_range(&mut self, l: usize, r: usize) -> T {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		// CHANGE: cache n_padded — no recomputation of tree.len() / 2 - 1.
		self.get_val(1, 0, self.n_padded - 1, l, r).unwrap()
	}


	/// get the combined information of the range (0-based indexing)  
	/// based on the custom `combiner` provided  
	/// `O(logn)`  
	///  using the `getter` and the existing `nodes` it creates another data,
	///  and combines it using the `combiner`
	///  is it used when the data of node is large and creating copies of it is not
	///  possible. Maybe the entire information of the node is not required and only
	///  some part of it is needed for this query. 
	#[inline]
	fn get_range_custom<U,G:FnMut(&T)->U,F:FnMut(U,U)->U>(&mut self, l: usize, r: usize,mut getter:G,mut combiner:F) -> U {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.get_val_custom(1, 0, self.n_padded - 1, l, r,&mut getter,&mut combiner).unwrap()
	}

	/// updates a single element bases on the updater provided    
	/// 0-based indexing  
	#[inline]
	fn update(&mut self, ind: usize, val: &V) {
		self.update_range(ind, ind, val);
	}

	/// stores what to update in a lazy tree and apply this update when i try to get this value  
	/// it uses the `update_combiner` and `updater`   
	/// `O(logn)`
	#[inline]
	fn update_range(&mut self, l: usize, r: usize, val: &V) {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		self.update_segment(1, 0, self.n_padded - 1, l, r, val);
	}

	/// for internal use only   
	/// if you want to update any thing use `update_range`  or `update`  
	#[inline(always)]
	fn update_segment(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize, val: &V) {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return;
		}

		if left >= l && right <= r {
			self.nodes[node].0 = (self.updater)(&self.nodes[node].0, &(self.update_val_propagate)(val,left,right,l,r), left, right);
			if left != right {
				let mid = left + (right - left) / 2;
				self.nodes[2 * node].1 = (self.update_combiner)(&self.nodes[2 * node].1, &(self.update_val_propagate)(val,left,mid,l,r));
				self.nodes[2 * node + 1].1 = (self.update_combiner)(&self.nodes[2 * node + 1].1, &(self.update_val_propagate)(val,mid+1,right,l,r));
			}
		} else {
			// CHANGE: left + (right - left) / 2 — no overflow risk on large indices,
			//         consistent throughout (old code mixed >> 1 and / 2).
			let mid: usize = left + (right - left) / 2;
			self.update_segment(2 * node, left, mid, l, r, val);
			self.update_segment(2 * node + 1, mid + 1, right, l, r, val);
			let lv = self.nodes[2 * node].0.clone();
			let rv = self.nodes[2 * node + 1].0.clone();
			self.nodes[node].0 = (self.combiner)(&lv, &rv);
		}
	}

	/// for internal use only  
	/// [ l,r ] this range is what i am looking for 
	/// [ left,right ] there is where i am looking for l,r 
	#[inline(always)]
	fn get_val(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize) -> Option<T> {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return None;
		}

		if left >= l && right <= r {
			return Some(self.nodes[node].0.clone());
		}

		// CHANGE: left + (right - left) / 2 — no overflow risk on large indices,
		//         consistent throughout (old code mixed >> 1 and / 2).
		let mid: usize = left + (right - left) / 2;
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
	#[inline(always)]
	fn get_val_custom<U,G:FnMut(&T)->U,F:FnMut(U,U)->U>(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize,mut getter:&mut G,mut combiner:&mut F) -> Option<U> {
		self.update_lazy(node, left, right);

		if left > r || right < l {
			return None;
		}

		if left >= l && right <= r {
			return Some(getter(&self.nodes[node].0));
		}

		// CHANGE: left + (right - left) / 2 — no overflow risk on large indices,
		//         consistent throughout (old code mixed >> 1 and / 2).
		let mid: usize = left + (right - left) / 2;
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
	// CHANGE: .take() replaces .clone() + manual `= None` — moves the value out
	//         atomically, no clone allocation, no separate `= None` write.
	#[inline(always)]
	fn update_lazy(&mut self, node: usize, left: usize, right: usize) {
		if let Some(lazy_val) = self.nodes[node].1.take() {
			self.nodes[node].0 = (self.updater)(&self.nodes[node].0, &lazy_val, left, right);
			if left != right {
				let mid = left + (right - left) / 2;
				self.nodes[2 * node].1 = (self.update_combiner)(&self.nodes[2 * node].1, &(self.update_val_propagate)(&lazy_val,left,mid,left,right));
				self.nodes[2 * node + 1].1 = (self.update_combiner)(&self.nodes[2 * node + 1].1, &(self.update_val_propagate)(&lazy_val,mid+1,right,left,right));
			}
		}
	}

	/// Flushes all lazy updates from the root down to the leaves.
	/// After calling this, the leaf nodes in `self.tree` will contain 
	/// the most up-to-date values.
	/// Complexity: O(N)
	#[inline]
	fn get_all_elements(&mut self) -> Vec<T>{
		let mut v = vec![self.t_default.clone();self.n];
		self.collect_values(1, 0, self.n_padded - 1, &mut v);
		v
	}

	/// Flushes all lazy values and writes into a caller-supplied buffer.
	/// Prefer this over `get_all_elements` in hot loops to avoid allocating.
	/// `O(n)`
	#[inline]
	fn get_all_elements_into(&mut self, out: &mut Vec<T>) {
		out.clear();
		out.resize(self.n, self.t_default.clone());
		self.collect_values(1, 0, self.n_padded - 1, out);
	}

	/// internal use only   
	/// calls update_lazy for every node  
	/// overall complexity is `O(n)`  
	#[inline(always)]
	fn collect_values(&mut self, node: usize, left: usize, right: usize,arr : &mut Vec<T>) {
		// Apply this node's lazy first, then push down
		self.update_lazy(node, left, right);

		if left == right {
			if let Some(v) = arr.get_mut(left){*v = self.nodes[node].0.clone();}
			return;
		}

		// CHANGE: left + (right - left) / 2 — no overflow risk on large indices,
		//         consistent throughout (old code mixed >> 1 and / 2).
		let mid = left + (right - left) / 2;
		self.collect_values(2 * node, left, mid,arr);
		self.collect_values(2 * node + 1, mid + 1, right,arr);
	}

	/// `pred` takes 2 inputs prefix_combined (based on combiner) and i that is the length of the prefix  
	///
	/// walks the tree descending greedily into the left child whenever the left
	/// subtree alone already satisfies `pred`, otherwise folds the left child's
	/// aggregate into an accumulator and descends right  
	/// returns the length of the prefix found, or `None` if no prefix satisfies `pred`  
	/// `O(logn)`  
	///
	/// # How it works
	/// at each internal node we have already accumulated everything to the left
	/// of the current subtree in `acc`  
	/// - if `pred(acc combined with left_child)` is true → the answer lies in
	///   the left subtree, recurse left without touching `acc`  
	/// - otherwise → the left subtree contributes to `acc` but the answer is
	///   to the right, recurse right with the updated `acc`  
	///
	/// # Requirements
	/// `pred` must be monotone: once `pred(prefix,i)` becomes true it must stay
	/// true for all longer prefixes  
	/// the combiner must be the same aggregation used for the property being
	/// searched (e.g. sum, min, max)  
	///
	/// # Example
	/// ```
	/// // first index where prefix-sum >= k
	/// st.find_first(|prefix_sum| *prefix_sum >= k)
	/// ```
	#[inline]
	fn find_first<P: FnMut(&T,usize) -> bool>(&mut self, mut pred: P) -> Option<usize> {
		// if the whole tree does not satisfy pred, no answer exists
		self.update_lazy(1, 0, self.n_padded - 1);
		if !pred(&self.nodes[1].0,self.n) {
			return None;
		}
		self.find_first_in(1, 0, self.n_padded - 1, &self.t_default.clone(), &mut pred)
	}

	/// for internal use only  
	/// `acc` is the aggregate of everything to the left of this subtree  
	#[inline(always)]
	fn find_first_in<P: FnMut(&T,usize) -> bool>(&mut self,node: usize,left: usize,right: usize,acc: &T,pred: &mut P) -> Option<usize>
	{
		self.update_lazy(node, left, right);

		if left == right {
			// leaf: only report it if it is within the original array bounds
			return if left < self.n { Some(left+1) } else { None };
		}

		let mid = left + (right - left) / 2;

		// combine acc with the left child's aggregate
		let left_combined = (self.combiner)(acc, &self.nodes[2 * node].0);

		if pred(&left_combined,mid+1) {
			// answer is in the left subtree
			self.find_first_in(2 * node, left, mid, acc, pred)
		} else {
			// left subtree does not contain the answer; carry its aggregate
			// forward and search the right subtree
			self.find_first_in(2 * node + 1, mid + 1, right, &left_combined, pred)
		}
	}

}
// endregion 

// region BinaryIndexTree
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryIndexedTreeGeneral<T, A>
where
	T: Copy,
	A: Fn(T, T) -> T,
{
	tree: Vec<T>,
	len: usize,
	capacity: usize,
	addition: A,
	subtract: A,
	identity: T,
}

impl<T, A> BinaryIndexedTreeGeneral<T, A>
where
	T: Copy,
	A: Fn(T, T) -> T,
{
	/// Creates a new, empty Binary Indexed Tree with a fixed capacity.
	///
	/// # Parameters
	/// * `max_size`: The maximum number of elements this tree can hold. It cannot be resized later.
	/// * `adder`: A function or closure defining the associative addition operation.
	/// * `subtraction`: A function or closure defining the inverse operation of addition.
	/// * `identity`: The neutral identity element for the addition operation (e.g., 0 for integers).
	///
	/// # Returns
	/// Returns a new instance of `BinaryIndexedTreeGeneral`.
	pub fn new(max_size: usize, adder: A, subtraction: A, identity: T) -> Self {
		Self {
			tree: vec![identity; max_size + 1],
			len: 0,
			capacity: max_size,
			identity,
			addition: adder,
			subtract: subtraction,
		}
	}

	/// Returns the current number of elements stored in the tree.
	///
	/// # Parameters
	/// * `&self`: Reference to the tree instance.
	///
	/// # Returns
	/// Returns a `usize` representing the current element count.
	pub fn len(&self) -> usize {
		self.len
	}

	/// Appends a new element to the end of the tree.
	/// Complexity: O(log n)
	///
	/// # Parameters
	/// * `&mut self`: Mutable reference to the tree.
	/// * `x`: The value of type `T` to be appended.
	///
	/// # Returns
	/// Returns `true` if the element was successfully added, or `false` if the tree has reached its maximum capacity.
	pub fn push(&mut self, x: T) -> bool {
		if self.len < self.capacity {
			self.increment(self.len, x);
			self.len += 1;
			true
		} else {
			false
		}
	}

	/// Instantly builds a fully populated Binary Indexed Tree from an existing slice.
	/// Complexity: O(n)
	///
	/// # Parameters
	/// * `iterator`: A slice of initial elements to populate the tree.
	/// * `adder`: A function or closure defining the addition operation.
	/// * `subtraction`: A function or closure defining the subtraction operation.
	/// * `identity`: The neutral identity element for the addition operation.
	///
	/// # Returns
	/// Returns a fully constructed and initialized `BinaryIndexedTreeGeneral`.
	pub fn from_slice(iterator: &[T], adder: A, subtraction: A, identity: T) -> Self {
		let length = iterator.len();
		let mut tree = vec![identity; length + 1];

		// Copy initial values into the 1-based tree array
		for i in 0..length {
			tree[i + 1] = iterator[i];
		}

		// Ripple the values up the tree in O(n)
		for i in 1..=length {
			let j = i + (i & i.wrapping_neg());
			if j <= length {
				let val = tree[i];
				tree[j] = (adder)(tree[j], val);
			}
		}

		Self {
			tree,
			len: length,
			capacity: length,
			identity,
			addition: adder,
			subtract: subtraction,
		}
	}

	/// Mutates the tree by adding a delta value to the element at a specific index.
	/// Complexity: O(log n)
	///
	/// # Parameters
	/// * `&mut self`: Mutable reference to the tree.
	/// * `index`: The 0-based index of the element to modify.
	/// * `delta`: The value to combine/add to the existing element.
	pub fn increment(&mut self, mut index: usize, delta: T) {
		index += 1;
		while index <= self.capacity {
			self.tree[index] = (self.addition)(self.tree[index], delta);
			index += index & index.wrapping_neg();
		}
	}

	/// Mutates the tree by subtracting a delta value from the element at a specific index.
	/// Complexity: O(log n)
	///
	/// # Parameters
	/// * `&mut self`: Mutable reference to the tree.
	/// * `index`: The 0-based index of the element to modify.
	/// * `delta`: The value to subtract from the existing element.
	pub fn decrement(&mut self, mut index: usize, delta: T) {
		index += 1;
		while index <= self.capacity {
			self.tree[index] = (self.subtract)(self.tree[index], delta);
			index += index & index.wrapping_neg();
		}
	}

	/// Computes the accumulated sum within a specific range, inclusive of both bounds.
	/// Complexity: O(log n)
	///
	/// # Parameters
	/// * `&self`: Reference to the tree.
	/// * `l`: The 0-based inclusive lower bound index of the range.
	/// * `r`: The 0-based inclusive upper bound index of the range.
	///
	/// # Panics
	/// Panics if the lower bound `l` is strictly greater than the upper bound `r`.
	///
	/// # Returns
	/// Returns the total combined range sum of type `T`.
	pub fn get_range_sum(&self, l: usize, r: usize) -> T {
		if l > r {
			panic!("Invalid Range! (l <= r)");
		}
		(self.subtract)(
			self.prefix_sum(r),
			if l == 0 {
				self.identity
			} else {
				self.prefix_sum(l - 1)
			},
		)
	}

	/// Computes the prefix sum of all elements from index 0 up to and including index `k`.
	/// Complexity: O(log n)
	///
	/// # Parameters
	/// * `&self`: Reference to the tree.
	/// * `k`: The 0-based inclusive upper bound index of the prefix.
	///
	/// # Returns
	/// Returns the accumulated prefix sum of type `T`.
	pub fn prefix_sum(&self, mut k: usize) -> T {
		let mut sum: T = self.identity;
		k += 1;
		while k > 0 {
			sum = (self.addition)(sum, self.tree[k]);
			k -= k & k.wrapping_neg();
		}
		sum
	}

	/// Performs binary lifting to find the smallest index where a condition becomes false.
	/// Complexity: O(log n)
	///
	/// # Assumptions
	/// This function assumes that the predicate function `f` is monotonic. It must return 
	/// `true` for early prefix values and transition to `false` permanently at some point.
	///
	/// # Parameters
	/// * `&self`: Reference to the tree.
	/// * `f`: A mutable closure or function that takes an accumulated prefix sum of type `T` 
	///        and returns a `bool`.
	///
	/// # Returns
	/// Returns the 0-based index of the first element that causes the predicate `f` to return `false`.
	pub fn binary_search<S>(&self, mut f: S) -> usize
	                        where
		                        S: FnMut(T) -> bool,
	{
		let mut i = 0;
		let mut current_prefix = self.identity;
		let mut step = 1;

		// Find the largest power of 2 less than or equal to capacity
		while step <= self.capacity {
			step <<= 1;
		}
		step >>= 1;

		while step > 0 {
			if i + step <= self.capacity {
				// Calculate what the prefix sum would be if we jump to (i + step)
				let next_prefix = (self.addition)(current_prefix, self.tree[i + step]);

				// If it still satisfies the true condition, take the jump
				if f(next_prefix) {
					current_prefix = next_prefix;
					i += step;
				}
			}
			step >>= 1;
		}
		i
	}
}
// endregion