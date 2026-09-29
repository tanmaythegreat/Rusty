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