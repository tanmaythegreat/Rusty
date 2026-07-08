use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::io::{self, stdin, BufWriter, Stdout, Write};
use std::str::FromStr;
use std::sync::{Mutex, OnceLock};

pub enum TOVISIT<T>{Yes,No,	Return(T)}
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
fn int3<T, U,V>() -> (T, U,V) where
	T: FromStr,
	U: FromStr,
	V: FromStr,
	T::Err: Debug,
	U::Err: Debug,
	V::Err: Debug,
{
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

const MOD :u64= 998244353;
fn solve() {
	'outer: loop {
		let n : usize = int();
		let ans = Vec::with_capacity(10*n*n);
		
	}
}