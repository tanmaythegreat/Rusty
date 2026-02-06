use std::fmt::Debug;
use std::io::stdin;
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

fn int<T: FromStr>() -> T {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim().parse::<T>().ok().unwrap()
}
fn int2<T, U>() -> (T, U)
                where
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
	)
}
fn int3<T, U,V>() -> (T, U,V)
                  where
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
fn int5<T, U,V,W,M>() -> (T, U,V,W,M)
                      where
	                      T: FromStr,
	                      U: FromStr,
	                      V: FromStr,
	                      W: FromStr,
	                      M: FromStr,
	                      T::Err: Debug,
	                      U::Err: Debug,
	                      V::Err: Debug,
	                      W::Err: Debug,
	                      M::Err: Debug,
{
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
fn array<T: FromStr,B:FromIterator<T>>() -> B {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
}
fn int_array<T: FromStr,B:FromIterator<T>>() -> (T,B) {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	let mut ll = input.trim().split_whitespace();
	(ll.next().unwrap().parse::<T>().unwrap(),ll.map(|x| x.parse::<T>().ok().unwrap()).collect())
}
fn word()->String{
	let mut inp = String::new();
	stdin().read_line(&mut  inp).unwrap();
	inp.trim().to_string()
}fn word2() -> (String,String) {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	let mut a = input.trim().split_whitespace();
	(a.next().unwrap().to_string(),a.next().unwrap().to_string())
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
