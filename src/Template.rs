use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::iter::FromIterator;
use std::str::{FromStr, SplitAsciiWhitespace};

struct InputReader<'a> {
	stream: SplitAsciiWhitespace<'a>,
}
impl<'a> InputReader<'a> {
	fn new(s: &'a str) -> Self {
		Self {
			stream: s.split_ascii_whitespace(),
		}
	}
	fn next<T:FromStr>(&mut self)->T where <T as FromStr>::Err: Debug{
		self.stream.next().unwrap().parse::<T>().unwrap()
	}
	fn next_word(&mut self) ->&str {
		self.stream.next().unwrap()
	}
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

fn main() {
	/*
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let mut input = InputReader::new(&input);

	let t:i32=input.next();
	*/
	let t:usize = int();
	for _ in 0..t {

	}
}
