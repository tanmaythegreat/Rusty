use std::cell::{Cell, RefCell};
use TOVISIT::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::ffi::{c_ushort, c_void};
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{self, BufWriter, Stdout, Write, stdin};
use std::iter::{once, FromIterator};
use std::ops::Add;
use std::str::FromStr;
use std::sync::{Mutex, OnceLock};

//region IO
static OUT: OnceLock<Mutex<BufWriter<Stdout>>> = OnceLock::new();

fn out() -> &'static Mutex<BufWriter<Stdout>> {
	OUT.get_or_init(|| Mutex::new(BufWriter::new(io::stdout())))
}

macro_rules! print {
    ($($arg:tt)*) => {
        write!(out().lock().unwrap(), $($arg)*).unwrap()
    };
}

macro_rules! println {
    () => {
        writeln!(out().lock().unwrap()).unwrap()
    };
    ($($arg:tt)*) => {
        writeln!(out().lock().unwrap(), $($arg)*).unwrap()
    };
}

macro_rules! flush {
    () => {
        out().lock().unwrap().flush().unwrap()
    };
}

fn main() {
	solve();
	flush!()
}
// endregion

// region Input

#[inline(always)]
fn int<T: FromStr>() -> T {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim().parse::<T>().ok().unwrap()
}
#[inline(always)]
fn int2<T, U>() -> (T, U) where
	T: FromStr,
	U: FromStr,
	T::Err: Debug,
	U::Err: Debug,{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<U>().unwrap(),
	)
}
#[inline(always)]
fn int3<T, U,V>() -> (T, U,V) where
	T: FromStr,
	U: FromStr,
	V: FromStr,
	T::Err: Debug,
	U::Err: Debug,
	V::Err: Debug,{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<U>().unwrap(),
		it.next().unwrap().parse::<V>().unwrap(),
	)
}
#[inline(always)]
fn int4<T, U,V,W>() -> (T, U,V,W) where
	T: FromStr,
	U: FromStr,
	V: FromStr,
	W: FromStr,
	T::Err: Debug,
	U::Err: Debug,
	V::Err: Debug,
	W::Err: Debug,
{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<U>().unwrap(),
		it.next().unwrap().parse::<V>().unwrap(),
		it.next().unwrap().parse::<W>().unwrap(),
	)
}
#[inline(always)]

fn int5<T, U,V,W,M>() -> (T, U,V,W,M) where
	T: FromStr,
	U: FromStr,
	V: FromStr,
	W: FromStr,
	M: FromStr,
	T::Err: Debug,
	U::Err: Debug,
	V::Err: Debug,
	W::Err: Debug,
	M::Err: Debug,{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<U>().unwrap(),
		it.next().unwrap().parse::<V>().unwrap(),
		it.next().unwrap().parse::<W>().unwrap(),
		it.next().unwrap().parse::<M>().unwrap()
	)
}

#[inline(always)]

fn int6<T, U,V,W,M,N>() -> (T, U,V,W,M,N) where
	T: FromStr,
	U: FromStr,
	V: FromStr,
	W: FromStr,
	M: FromStr,
	N: FromStr,
	T::Err: Debug,
	U::Err: Debug,
	V::Err: Debug,
	W::Err: Debug,
	N::Err: Debug,
	M::Err: Debug,{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<U>().unwrap(),
		it.next().unwrap().parse::<V>().unwrap(),
		it.next().unwrap().parse::<W>().unwrap(),
		it.next().unwrap().parse::<M>().unwrap(),
		it.next().unwrap().parse::<N>().unwrap()
	)
}
#[inline(always)]

fn array<T: FromStr,B:FromIterator<T>>() -> B {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
}
/// returns ( compressed array , orignal array , maping (index:compressed,value : original),maping (key:original,value:compressed)) 
#[inline(always)]
fn array_compressed<T: FromStr +Copy + Ord + Hash>() -> (Vec<usize>, Vec<T>, Vec<T>, HashMap<T, usize>) {
	let arr : Vec<T> = array();
	let mut maping = arr.clone();
	maping.sort_unstable();
	maping.dedup();
	let mut inv : HashMap<T,usize> = HashMap::with_capacity(2* maping.len());
	for (i,&t) in maping.iter().enumerate(){
		inv.insert(t,i);
	}
	(arr.iter().map(|a|inv[a]).collect(),arr,maping,inv)
}

#[inline(always)]

fn int_array<T: FromStr,B:FromIterator<T>>() -> (T,B)  where <T as FromStr>::Err: Debug{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	let mut ll = input.trim().split_whitespace();
	(ll.next().unwrap().parse::<T>().unwrap(),ll.map(|x| x.parse::<T>().ok().unwrap()).collect())
}
#[inline(always)]

fn word()->String{
	let mut inp = String::new();
	stdin().read_line(&mut  inp).unwrap();
	inp.trim().to_string()
}
#[inline(always)]

fn word2() -> (String,String) {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	let mut a = input.trim().split_whitespace();
	(a.next().unwrap().to_string(),a.next().unwrap().to_string())
}

#[inline(always)]

fn int_word<T>() -> (T, String) where
	T: FromStr,
	T::Err: Debug,{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().to_owned()
	)
}
#[inline(always)]

fn int2_word<T,U>() -> (T,U, String) where
	T: FromStr,
	U: FromStr,
	T::Err: Debug,
	U::Err: Debug,
{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<U>().unwrap(),
		it.next().unwrap().to_owned()
	)
}

#[inline(always)]

fn word_int2<T>() -> (String,T,T) where
	T: FromStr,
	T::Err: Debug,{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().to_owned(),
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<T>().unwrap(),
	)
}
#[inline(always)]

fn word_int<T>() -> (String,T) where
	T: FromStr,
	T::Err: Debug,{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().to_owned(),
		it.next().unwrap().parse::<T>().unwrap(),
	)
}
// endregion

/// computes divisors upto n (inclusive)  
/// O(nlog(n))
pub fn precompute_divisors(n:usize) -> Vec<Vec<usize>> {
	let mut divisors : Vec<Vec<usize>> = vec![Vec::with_capacity((n.ilog2() + 1) as usize); n+1];
	for i in 1..=n{
		for j in (i..=n).step_by(i){
			divisors[j].push(i);
		}
	}
	divisors
}

use std::ops::{Div, Sub};

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

pub enum TOVISIT<T>{Yes,No,	Return(T)}

pub trait ParentStore<V> {fn record_parent(&mut self, child: V, parent: V);}
impl<V: Eq + Hash> ParentStore<V> for HashMap<V, V> {fn record_parent(&mut self, child: V, parent: V) {	self.insert(child, parent);	}}
impl ParentStore<usize> for Vec<usize> {fn record_parent(&mut self, child: usize, parent: usize) {	self[child] = parent;}}
impl<V> ParentStore<V> for () {	fn record_parent(&mut self, _child: V, _parent: V) {}}

/// Performs BFS from multiple sources parallelly  
/// start from source,v is the vertex/node and w of source will be 0 usually,or +1,-1 if you want 
/// to distinguish between sources (bidirectional search).  
/// `parent_store` : `()` if dont want to store parent,vec![usize::MAX;n] or HashMap::<V,V>::with_capacity(N).
/// `to_visit` : tells weather to visit this node or not. caller is supposed to keep track of visited nodes.  
///  `visit` : `visit(node,weight,parent)` this node. mark this mode visited in here. decide weather to stop 
/// the BFS here and now, return none to continue.  
///  `adjacent` : returns the adjacent nodes along with the edge weight. it is no problem if it contains the 
/// parent as well, make sure things are handled in `to_visit`.   
/// Returns:
/// - `parent` map built during traversal (source nodes are NOT keys in this map)
/// - `Some((value, node))` if `visit` returned `Some(value)` for some node, else `None`
#[inline]
pub fn multi_source_bfs<V, WEdge,WPath,WPathIter, T, Store, ToVisit, Visit, Adjacent, AdjIter, Add>(
	sources: impl IntoIterator<Item = (V, WPath)>,
	mut parent_store: Store,
	mut to_visit: ToVisit,
	mut visit: Visit,
	mut adjacent: Adjacent,
	mut add: Add,
) -> (Store, Option<(T, V)>)
	where
		V: Copy,
		WEdge : Copy,
		WPath: Copy,
		Store: ParentStore<V>,
		ToVisit: FnMut(V, WPath, Option<V>) -> TOVISIT<T>,
		Visit: FnMut(V, WPath, Option<V>) -> Option<T>,
		Adjacent: FnMut(V, WPath, Option<V>) -> AdjIter,
		AdjIter: Iterator<Item = (V, WEdge)>,
		Add: FnMut(WPath, WEdge) -> WPathIter,
		WPathIter:Iterator<Item=WPath>,
{
	let mut queue: VecDeque<(V, WPath, Option<V>)> = VecDeque::new();

	for (s, w) in sources {
		match to_visit(s, w, None) {
			Yes => {
				if let Some(value) = visit(s, w, None) {
					return (parent_store, Some((value, s)));
				}
				queue.push_back((s, w, None));
			}
			No => {}
			Return(r) => return (parent_store, Some((r, s))),
		}
	}

	while let Some((current, weight, parent_of_current)) = queue.pop_front() {
		for (neighbor, edge_weight) in adjacent(current, weight, parent_of_current) {
			let new_weights = add(weight, edge_weight);
			for new_weight in new_weights {
				match to_visit(neighbor, new_weight, Some(current)) {
					Yes => {
						parent_store.record_parent(neighbor, current);

						if let Some(value) = visit(neighbor, new_weight, Some(current)) {
							return (parent_store, Some((value, neighbor)));
						}
						queue.push_back((neighbor, new_weight, Some(current)));
					}
					No => {}
					Return(r) => {
						return (parent_store, Some((r, neighbor)));
					}
				}
			}
		}
	}

	(parent_store, None)
}

pub trait InProgressStore<V> {
	fn insert_node(&mut self, node: V);
	fn remove_node(&mut self, node: V);
	fn is_node_in_stack(&self,node:V)->bool;
}
impl<V: Eq + Hash> InProgressStore<V> for HashSet<V> {
	#[inline(always)]fn insert_node(&mut self, node: V) {self.insert(node);	}
	#[inline(always)]fn remove_node(&mut self, node: V) {self.remove(&node);	}
	#[inline(always)]fn is_node_in_stack(&self, node: V) ->bool{ self.contains(&node)}
}
impl<V> InProgressStore<V> for () {
	#[inline(always)]fn insert_node(&mut self, _node: V) {}
	#[inline(always)]fn remove_node(&mut self, _node: V) {}
	#[inline(always)]
	fn is_node_in_stack(&self, node: V)->bool { false	}
}
/// Performs DFS from a single source.
/// `source`: starting vertex and its initial weight (usually 0).
/// `in_progress`: Pass `&mut HashSet::new()` to track cycle/back-edges, or `&mut ()` to disable tracking.
/// `to_visit(node, weight, parent, &in_progress)`: whether to visit this node.
/// `visit(node, weight, parent)`: mark visited. return `Some` to stop, `None` to continue.
/// `adjacent(node, weight, parent)`: neighbors + edge weights.
/// `add`: combines accumulated weight with edge weight.
/// `on_exit(node,weight,parent)`: called when backtracking past `node` (all neighbors exhausted).
///
/// Returns `Some((path, value, node))` where `path` is the chain of ancestors
/// from the source down to (but not including) `node` — read directly off the
/// stack, no parent map needed. `None` if nothing was found.
#[inline]
pub fn dfs<V, WEdge,WPath,WPathIter, T, Store, ToVisit, Visit, Adjacent, AdjIter, Add, OnExit>(
	source: (V, WPath),
	in_progress: &mut Store,
	mut to_visit: ToVisit,
	mut visit: Visit,
	mut adjacent: Adjacent,
	mut add: Add,
	mut on_exit: OnExit,
) -> Option<(Vec<V>, T, V)>
	where
		V: Copy,
		WPath: Copy,
		WEdge: Copy,
		Store: InProgressStore<V>,
		ToVisit: FnMut(V, WPath, Option<V>, &Store) -> TOVISIT<T>,
		Visit: FnMut(V, WPath, Option<V>) -> Option<T>,
		Adjacent: FnMut(V, WPath, Option<V>) -> AdjIter,
		AdjIter: Iterator<Item = (V, WEdge)>,
		Add: FnMut(WPath, WEdge) -> WPathIter,
		WPathIter : Iterator<Item=WPath>,
		OnExit: FnMut(V,WPath,Option<V>),

{
	let (s, w) = source;
	let mut stack: Vec<(V, WPath, Option<V>, AdjIter)> = Vec::new();

	match to_visit(s, w, None, in_progress) {
		TOVISIT::Yes => {
			if let Some(value) = visit(s, w, None) {
				return Some((Vec::new(), value, s));
			}
			in_progress.insert_node(s);
			let adj = adjacent(s, w, None);
			stack.push((s, w, None, adj));
		}
		TOVISIT::No => return None,
		TOVISIT::Return(r) => return Some((Vec::new(), r, s)),
	}

	while let Some((current, weight, parent_of_current, mut iter)) = stack.pop() {
		if let Some((neighbor, edge_weight)) = iter.next() {
			stack.push((current, weight, parent_of_current, iter));

			let new_weights = add(weight, edge_weight);
			for new_weight in new_weights {
				match to_visit(neighbor, new_weight, Some(current), in_progress) {
					TOVISIT::Yes => {
						if let Some(value) = visit(neighbor, new_weight, Some(current)) {
							let path = stack.iter().map(|&(v, _, _, _)| v).collect();
							return Some((path, value, neighbor));
						}
						in_progress.insert_node(neighbor);
						let adj = adjacent(neighbor, new_weight, Some(current));
						stack.push((neighbor, new_weight, Some(current), adj));
					}
					TOVISIT::No => {}
					TOVISIT::Return(r) => {
						let path = stack.iter().map(|&(v, _, _, _)| v).collect();
						return Some((path, r, neighbor));
					}
				}
			}
		} else {
			in_progress.remove_node(current);
			on_exit(current,weight,parent_of_current);
		}
	}

	None
}

/// Greatest Common Divisor  
/// Time: O(log(min(a, b)))  
#[inline]
pub fn gcd(mut a: usize, mut b: usize) -> usize {
	while b != 0 {
		let r = a % b;
		a = b;
		b = r;
	}
	a
}

///binary exponenciation with a custom multiplication rule
#[inline]
pub fn binary_exponentiation<T:Clone,U:FnMut(&mut T,&T)>(base:T,exponent:usize,mut multiply:U)->Option<T>
{
	if exponent==0{
		return None;
	}
	else if exponent==1{
		return Some(base);
	}
	let mut Ans = base.clone();
	let mut bit = 1<<(usize::BITS-exponent.leading_zeros()-1);
	while bit>1{
		let temp = Ans.clone();
		multiply(&mut Ans,&temp);
		bit = bit>>1;
		if exponent&bit!=0{
			multiply(&mut Ans,&base);
		}
	}
	Some(Ans)
}


/// Safe Modular Multiplication (a * b % m)  
/// Uses u128 to strictly prevent overflow.  
#[inline]
pub fn mul_mod(a:usize, b:usize, m: usize) -> usize {
	(a*b)%m
}

/// Modular Exponentiation (base^exp % m)  
/// Time: O(log exp)  
#[inline]
pub fn mod_pow(mut base: usize, mut exp: usize, m: usize) -> usize {
	let mut res = 1;
	base %= m;
	while exp > 0 {
		if exp % 2 == 1 { res = mul_mod(res, base, m); }
		base = mul_mod(base, base, m);
		exp /= 2;
	}
	res
}

/// Modular Inverse (Fermat's Little Theorem)  
/// REQUIRES: m is Prime.  
/// Time: O(log m)  
#[inline]
pub fn mod_inv(n: usize, m:usize) -> usize {
	mod_pow(n, m - 2, m)
}
/// Converts a `u64` number to a vector of digits in the given base.  
/// The most significant digit will be at index 0.
#[inline]
pub fn convert_to_base(mut num: u64, to_base: u64) -> Result<Vec<u64>,u64> {
	if to_base < 2 {
		return Err(to_base);
	}
	if num == 0 {
		return Ok(vec![0]);
	}

	let mut digits = Vec::with_capacity((num.ilog(to_base) + 3) as usize);

	while num > 0 {
		let remainder = num % to_base;
		digits.push(remainder);
		num /= to_base;
	}
	digits.reverse();
	Ok(digits)
}
#[inline(always)]
fn compress<T:Copy+PartialEq>(a: Vec<T>,n:usize) ->Vec<(T, usize)>{
	let mut compressed = Vec::with_capacity(n);
	let mut current = a[0];
	let mut count = 0;
	for i in a {
		if i==current{
			count+=1;
		}
		else {
			compressed.push((current,count));
			current = i;
			count = 1;
		}
	}
	compressed.push((current,count));
	compressed
}


/// Get All Divisors  
/// Returns a sorted list of all divisors of n.  
/// Time: O(sqrt(n))  
#[inline]
fn divisors(n: u64) -> Vec<u64> {
	let mut res = Vec::new();
	let mut i = 1;
	while i * i <= n {
		if n % i == 0 {
			res.push(i);
			if i * i != n { res.push(n / i); }
		}
		i += 1;
	}
	res.sort_unstable();
	res
}

#[inline(always)]

fn int7<T, U,V,W,M,N,O>() -> (T, U,V,W,M,N,O) where
	T: FromStr,
	U: FromStr,
	V: FromStr,
	W: FromStr,
	M: FromStr,
	N: FromStr,
	O: FromStr,
	T::Err: Debug,
	U::Err: Debug,
	V::Err: Debug,
	W::Err: Debug,
	N::Err: Debug,
	M::Err: Debug,
	O::Err: Debug,
{
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	let mut it = input.split_whitespace();
	(
		it.next().unwrap().parse::<T>().unwrap(),
		it.next().unwrap().parse::<U>().unwrap(),
		it.next().unwrap().parse::<V>().unwrap(),
		it.next().unwrap().parse::<W>().unwrap(),
		it.next().unwrap().parse::<M>().unwrap(),
		it.next().unwrap().parse::<N>().unwrap(),
		it.next().unwrap().parse::<O>().unwrap()
	)
}


const MOD : usize = 1_000_000_007;
fn solve() {
	let q : usize = int();
	for _ in 0..q{
		let (ax,ay,bx,by,cx,cy,r) :(f64,f64,f64,f64,f64,f64,f64)= int7();
		
		let c = (by-ay)*(cx-ax)/(bx-ax) -cy+ay;
		let m= (by-ay)/(bx-ax);
		
		let d = c.abs()/((1f64+m*m).sqrt());
		let theta = (d/r).acos();
		print!("{theta}\n");
		
		let x = cx + (m/(1f64+m*m))*(cy-m*cx-c);
		let y = cy -(cy-m*cx-c)/(1f64+m*m);
		
		assert_eq!(y,m*x+c);
		let d_ab = ( (ax-bx).powi(2) + (by-ay).powi(2)).sqrt();
		
		if d<r && x>=ax.min(bx) && x<=ax.max(bx) && y<=by.max(ay) && y>= ay.min(by){
			
			let rt = 2f64*r*theta - 2f64 * (r*r-d*d).sqrt();
			print!("{}\n",rt+d_ab);
		} else {
			print!("{d_ab}\n");
		}
	};
}