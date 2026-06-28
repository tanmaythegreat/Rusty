use std::cell::Cell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::str::FromStr;
use TOVISIT::{No, Return, Yes};

pub enum TOVISIT<T>{
	Yes,
	No,
	Return(T)
}

#[inline(always)]
fn word()->String{
	let mut inp = String::new();
	stdin().read_line(&mut  inp).unwrap();
	inp.trim().to_string()
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


pub trait ParentStore<V> {fn record_parent(&mut self, child: V, parent: V);}
impl<V: Eq + Hash> ParentStore<V> for HashMap<V, V> {fn record_parent(&mut self, child: V, parent: V) {	self.insert(child, parent);	}}
impl ParentStore<usize> for Vec<usize> {fn record_parent(&mut self, child: usize, parent: usize) {	self[child] = parent;}}
impl<V> ParentStore<V> for () {	fn record_parent(&mut self, _child: V, _parent: V) {}}

/// Performs Dijkstra's algorithm from multiple sources.
///
/// - `to_visit(node, weight, parent)` the caller should keep its own `dist` table and return `Yes` 
/// (recording `dist[node] = weight`) only if `weight` improves on the best known
///   distance, `No` otherwise. A node may legitimately get `Yes` more than
///   once (once per improving relaxation), each pushing a fresh heap entry.  
///  A node might be present with multiple weight values in the heap. when the minimum is taken out 
/// I would want the others to just get canceled   
/// - `visit(node, weight, parent)` : It should return `Yes` `if dist[node]==weight`
///  (means not stale) and `No` if stale. in case of `No` the adjacent nodes won't be visited.   
/// `parent_store`, `adjacent`, and `add` behave exactly as in
/// `multi_source_bfs`.
///
/// Returns:
/// - `parent` map built during traversal (source nodes are NOT keys in this map)
/// - `Some((value, node))` if `visit` returned `Some(value)` for some node, else `None`
#[inline]
pub fn multi_source_dijkstra<V, W, T, Store, ToVisit, Visit, Adjacent, AdjIter, Add>(
	sources: impl IntoIterator<Item = (V, W)>,
	mut parent_store: Store,
	mut to_visit: ToVisit,
	mut visit: Visit,
	mut adjacent: Adjacent,
	mut add: Add,
) -> (Store, Option<(T, V)>)
	where
		V: Copy + Ord,
		W: Copy + Ord,
		Store: ParentStore<V>,
		ToVisit: FnMut(V, W, Option<V>) -> TOVISIT<T>,
		Visit: FnMut(V, W, Option<V>) -> TOVISIT<T>,
		Adjacent: FnMut(V, W, Option<V>) -> AdjIter,
		AdjIter: Iterator<Item = (V, W)>,
		Add: FnMut(W, W) -> W,
{
	use TOVISIT::*;
	let mut heap: BinaryHeap<Reverse<(W, V, Option<V>)>> = BinaryHeap::new();
	for (s, w) in sources {
		match to_visit(s, w, None) {
			Yes => heap.push(Reverse((w, s, None))),
			No => {}
			Return(r) => return (parent_store, Some((r, s))),
		}
	}
	while let Some(Reverse((weight, current, parent_of_current))) = heap.pop() {
		
		match visit(current, weight, parent_of_current) {
			Yes=>{}
			No=>{continue;}
			Return(value)=>	return (parent_store, Some((value, current)))
		}

		for (neighbor, edge_weight) in adjacent(current, weight, parent_of_current) {
			let new_weight = add(weight, edge_weight);
			match to_visit(neighbor, new_weight, Some(current)) {
				Yes => {
					parent_store.record_parent(neighbor, current);
					heap.push(Reverse((new_weight, neighbor, Some(current))));
				}
				No => {}
				Return(r) => return (parent_store, Some((r, neighbor))),
			}
		}
	}
	(parent_store, None)
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
fn main() {
	let (v,e) : (usize,usize) = int2();
	let mut Adjecncey :Vec<Vec<(usize,(usize,usize,usize,usize))>> = vec![Vec::new();v];
	for _ in 0..e{
		let (a,b,w):(usize,usize,usize) = int3();
		Adjecncey[a-1].push((b-1,(w,w,w,w)));
		Adjecncey[b-1].push((a-1,(w,w,w,w)));
	}
	let mut dist = vec![Cell::new((usize::MAX,usize::MAX,0,usize::MAX));v];
	multi_source_dijkstra(
		[(0,(0,0,0,0))],
		(),
		|u,w,_|{
			if w.0<dist[u].get().0 { dist[u].set(w);Yes} else { TOVISIT::<()>::No }
		},
		|u,w,_|{if w==dist[u].get(){Yes}else{No}},
		|u,_,_|{Adjecncey[u].iter().copied()},
		|(w1,s1,max1,min1),(w2,s2,max2,min2)|{
			let (sum3,max3,min3) = (s1+s2,max1.max(max2),min1.min(min2));
			(sum3-max3+min3,sum3,max3,min3)
		},		
	);
	let mut itr = dist.into_iter();
	itr.next();
	for i in itr{
		print!("{} ",i.take().0);
	}
}