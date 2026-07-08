use std::fmt::Debug;
use std::io::{self, Write, BufWriter, Stdout, stdin};
use std::str::FromStr;
use std::sync::{OnceLock, Mutex};

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
fn array<T: FromStr,B:FromIterator<T>>() -> B {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
}
// endregion

fn solve(){
	let x = 4534;
	print!("{}\n",x);
}