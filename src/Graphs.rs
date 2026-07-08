use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque,HashSet};
use std::hash::Hash;

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
	#[inline(always)]fn is_node_in_stack(&self, node: V) ->bool{ self.contains(&V)}
}

impl<V> InProgressStore<V> for () {
	#[inline(always)]fn insert_node(&mut self, _node: V) {}
	#[inline(always)]fn remove_node(&mut self, _node: V) {}
	#[inline(always)]
	fn is_node_in_stack(&self, node: V)->bool { false	}
}

impl InProgressStore<usize> for Vec<bool> {
	#[inline(always)]fn insert_node(&mut self, _node: usize) {self[_node] = true;}
	#[inline(always)]fn remove_node(&mut self, _node: V) {self[_node] = false ;}
	#[inline(always)]fn is_node_in_stack(&self, node: usize) -> bool{self[node]}
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

/// Iterative DFS over a tree with `n` nodes, rooted at `root`.  
/// Returns `(entry_time, exit_time, order)`, `order` being the preorder  
/// SubTree of `u` is given by `order[entry_time[u]..=exit_time[u]]`    
/// - `get_children(node)` returns an iterator over `node`'s children.   
/// - `on_entry(node, entry_time)` fires the first time `node` is visited.  
/// - `on_exit(node, exit_time)`  fires after all of `node`'s descendants have been fully visited.
///
/// Timer semantics the shared `timer`
/// only advances on entry. So `exit_time[node] = timer - 1` at the
/// moment `node` finishes, which means:
///   - leaves get entry_time == exit_time  
///
/// (entry-order) sequence of visited nodes.
pub fn dfs_tree<GetChildren, I, OnEntry, OnExit>(
	n: usize,
	root: usize,
	mut get_children: GetChildren,
	mut on_entry: OnEntry,
	mut on_exit: OnExit,
) -> (Vec<usize>, Vec<usize>, Vec<usize>)
	where
		GetChildren: FnMut(usize) -> I,
		I: Iterator<Item = usize>,
		OnEntry: FnMut(usize, usize),
		OnExit: FnMut(usize, usize),
{
	let mut entry_time = vec![0usize; n];
	let mut exit_time = vec![0usize; n];
	let mut order = Vec::with_capacity(n);
	let mut timer: usize = 0;

	let mut stack: Vec<(usize, I)> = Vec::with_capacity(n);

	entry_time[root] = timer;
	on_entry(root, timer);
	order.push(root);
	timer += 1;
	stack.push((root, get_children(root)));

	loop {
		let next_child = match stack.last_mut() {
			Some((_, iter)) => iter.next(),
			None => break,
		};

		match next_child {
			Some(child) => {
				entry_time[child] = timer;
				on_entry(child, timer);
				order.push(child);
				timer += 1;
				let child_iter = get_children(child);
				stack.push((child, child_iter));
			}
			None => {
				let (node, _) = stack.pop().unwrap();
				exit_time[node] = timer - 1;
				on_exit(node, exit_time[node]);
			}
		}
	}

	(entry_time, exit_time, order)
}

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
pub fn multi_source_dijkstra<V, WEdge,WPath,WPathIter, T, Store, ToVisit, Visit, Adjacent, AdjIter, Add>(
	sources: impl IntoIterator<Item = (V, WPath)>,
	mut parent_store: Store,
	mut to_visit: ToVisit,
	mut visit: Visit,
	mut adjacent: Adjacent,
	mut add: Add,
) -> (Store, Option<(T, V)>)
	where
		V: Copy + Ord,
		WEdge: Copy,
		Store: ParentStore<V>,
		ToVisit: FnMut(V, WPath, Option<V>) -> TOVISIT<T>,
		Visit: FnMut(V, WPath, Option<V>) -> TOVISIT<T>,
		Adjacent: FnMut(V, WPath, Option<V>) -> AdjIter,
		AdjIter: Iterator<Item = (V, WEdge)>,
		Add: FnMut(WPath, WEdge) -> WPathIter,
		WPathIter : Iterator<Item=WPath>,
		WPath: Ord + Copy
{
	use TOVISIT::*;
	let mut heap: BinaryHeap<Reverse<(WPath, V, Option<V>)>> = BinaryHeap::new();
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
			let new_weights = add(weight, edge_weight);
			for new_weight in new_weights {
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
	}
	(parent_store, None)
}




/// it computes shortest path between every 2 nodes   
/// Do not use this function , it is only here so that i dont forget it.  
/// `n` is numberr of nodes  
fn floyd_warshall(Adjecncey:Vec<Vec<usize>>,n:usize){
	for k in 0..n{
		for i in 0..n{
			for j in 0..n {
				if Adjecncey[i][k] < usize::MAX && Adjecncey[k][j] < usize::MAX {
					Adjecncey[i][j] = Adjecncey[i][j].min(Adjecncey[i][k] + Adjecncey[k][j]);
					//next_node[i][j] = next_node[i][k]; if A[i][j] was updated
				}
			}
		}
	}
}

fn Bellman_Ford_Algorithm(edges:Vec<(usize,usize,i64)>){
	let mut dist = vec![i64::MAX;v];
	dist[0] = 0;
	for _ in 0..v-1{
		for &(a,b,x) in edges.iter(){
			if dist[a]!=i64::MAX{
				dist[b] = dist[b].min(dist[a]+x);
				// can also do parent[b] = a if dist[b] updated. 
			}
		}
	}
	// up to here was assuming no -ve cycles
	
	// to check -ve cycles, if the next iteration improves anything
	for &(a,b,x) in edges.iter() {
		if dist[a] != i64::MAX {
			if dist[b] < dist[a] + x {
				dist[b] = dist[a]+x;
				// means there is a -ve cycle , you may need to check can this -ve cycle lead to the target point, do bfs or dfs
				
				// let mut visited = vec![false;v];
				// if dfs(
				// 	(b,()),
				// 	&mut (),
				// 	|u,_,_,_|{if visited[u]{No}else { visited[u] = true;Yes }},
				// 	|u,_,_|{if u==v-1{Some(())} else{None}},
				// 	|u,_,_|{Adj[u].iter().copied()},
				// 	|a,b|(),
				// 	|_|{}
				// ).is_some(){
				// 	print!("-1\n");
				// 	return;
				// }
			}
		}
	}
}