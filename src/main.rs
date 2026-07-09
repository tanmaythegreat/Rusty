use std::cell::Cell;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt::Debug;
use std::io::{self, stdin, BufWriter, Stdout, Write};
use std::str::FromStr;
use std::sync::{Mutex, OnceLock};
use TOVISIT::*;

pub enum TOVISIT<T>{Yes,No,	Return(T)}

pub trait ParentStore<V> {fn record_parent(&mut self, child: V, parent: V);}
impl ParentStore<usize> for Vec<usize> {fn record_parent(&mut self, child: usize, parent: usize) {	self[child] = parent;}}
impl ParentStore<(usize,usize)> for Vec<Vec<(usize,usize)>> {fn record_parent(&mut self, child: (usize,usize), parent: (usize,usize)) {	self[child.0][child.1] = parent;}}
impl<V> ParentStore<V> for () {	fn record_parent(&mut self, _child: V, _parent: V) {}}

//region IO
static OUT: OnceLock<Mutex<BufWriter<Stdout>>> = OnceLock::new();

fn out() -> &'static Mutex<BufWriter<Stdout>> {
	OUT.get_or_init(|| Mutex::new(BufWriter::new(io::stdout())))
}

// macro_rules! print {
//     ($($arg:tt)*) => {
//         write!(out().lock().unwrap(), $($arg)*).unwrap()
//     };
// }

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
fn array<T: FromStr,B:FromIterator<T>>() -> B {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
}
#[inline(always)]

fn word()->String{
	let mut inp = String::new();
	stdin().read_line(&mut  inp).unwrap();
	inp.trim().to_string()
}
// endregion

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
/// `on_exit(node)`: called when backtracking past `node` (all neighbors exhausted).
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
		OnExit: FnMut(V),

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
			on_exit(current);
		}
	}

	None
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
/// Safe Modular Multiplication (a * b % m)  
/// Uses u128 to strictly prevent overflow.  
#[inline]
pub fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
	((a as u128 * b as u128) % m as u128) as u64
}

/// Modular Exponentiation (base^exp % m)  
/// Time: O(log exp)  
#[inline]
pub fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
	let mut res = 1;
	base %= m;
	while exp > 0 {
		if exp % 2 == 1 { res = mul_mod(res, base, m); }
		base = mul_mod(base, base, m);
		exp /= 2;
	}
	res
}

// region Disjoint Set Union
struct DisjointSetUnion {
	/// parent[i] = parent of node i; if parent[i] == i, then i is a root.
	parent: Vec<usize>,
	/// size[i] = number of elements in the set rooted at i (only valid when i is a root).
	size: Vec<usize>,
	/// Current number of disjoint sets remaining.
	num_sets: usize,
}

impl DisjointSetUnion {
	/// Creates n singleton sets, each element its own parent with size 1.
	#[inline]
	fn new(n: usize) -> Self {
		DisjointSetUnion {
			parent: (0..n).collect(),
			size: vec![1; n],
			num_sets: n,
		}
	}

	/// Finds the root of x, applying path halving to flatten the tree as it walks up.
	#[inline]
	fn find(&mut self, mut x: usize) -> usize {
		while self.parent[x] != x {
			self.parent[x] = self.parent[self.parent[x]];
			x = self.parent[x];
		}
		x
	}

	/// Merges the sets containing x and y; returns false if already merged.  
	/// `on_union(new_parent, old_parent)` lets the caller migrate custom per-set
	/// data (max, min, sum, etc.) at the moment of merge — passed per-call,
	#[inline]
	fn union(&mut self, x: usize, y: usize, mut on_union: impl FnMut(usize, usize)) -> bool {
		let mut rx = self.find(x);
		let mut ry = self.find(y);
		if rx == ry {
			return false;
		}
		if self.size[rx] < self.size[ry] {
			std::mem::swap(&mut rx, &mut ry);
		}
		on_union(rx, ry); // old root's data still intact here
		self.parent[ry] = rx;
		self.size[rx] += self.size[ry];
		self.num_sets -= 1;
		true
	}

	#[inline]
	fn same_set(&mut self, x: usize, y: usize) -> bool {
		self.find(x) == self.find(y)
	}

	#[inline]
	fn get_set_size(&mut self, x: usize) -> usize {
		let root = self.find(x);
		self.size[root]
	}

	#[inline]
	fn count_sets(&self) -> usize {
		self.num_sets
	}
}
// endregion

const MOD : u64 = 998244353;
fn solve(){
	let t : usize = int();
	'outer:for _ in 0..t {
		let (n, m): (usize, usize) = int2();
		let mut Adj = vec![Vec::new();n];
		let mut edges = Vec::with_capacity(m);
		let mut dsu = DisjointSetUnion::new(n);
		for _ in 0..m{
			let (a,b):(usize,usize) = int2();
			Adj[a-1].push((b-1,()));
			Adj[b-1].push((a-1,()));
			edges.push((a-1,b-1));
			dsu.union(a-1, b-1, |_,_|{});
		}
		let mut groups : HashMap<usize,(Vec<usize>,Vec<(usize,usize)>)> = HashMap::with_capacity(n*2);
		for i in 0..n{
			groups.entry(dsu.find(i)).and_modify(|a|a.0.push(i)).or_insert((vec![i],vec![]));
		}
		for &e in &edges{
			groups.get_mut(&dsu.find(e.0)).unwrap().1.push(e);
		}
		let mut visited = vec![false; n];
		let mut value = vec![0u8;n];
		let mut ans = 1;
		for (leader,(group,edges_group)) in groups {
			dfs(
				(leader, ()),
				&mut (),
				|i, _, par, _| if !visited[i] {
					visited[i] = true;
					if let Some(par) = par { value[i] = value[par] ^ 1; }
					Yes
				} else { No::<()> },
				|_, _, _| { None },
				|i, _, _| {
					Adj[i].iter().copied()
				},
				|_, _| [()].into_iter(),
				|_| {}
			);
			for (a, b) in edges_group {
				if value[a] + value[b] != 1 {
					print!("0\n");
					continue 'outer;
				}
			}
			let mut z_count = 0;
			for &i in &group {
				if value[i] == 0 {
					z_count += 1;
				}
			}
			let one_count= group.len() as u64-z_count;
			ans = mul_mod(ans,mod_pow(2,z_count,MOD)+mod_pow(2,one_count,MOD),MOD);
		}
		print!("{}\n",ans);
	}
}