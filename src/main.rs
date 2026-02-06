use std::cmp::{max, min_by_key};
use std::fmt::Debug;
use std::io::stdin;
use std::str::FromStr;


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

fn array<T: FromStr,B:FromIterator<T>>() -> B {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
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


/// Linear Sieve to precompute euler totient for numbers upto n
/// Time: O(nlog(log(n)))
pub fn linear_sieve_totient(n: usize) -> Vec<usize> {
    let mut totient = vec![0;n+1];
    for i in 0..=n{totient[i] = i;}

    for i in 2..=n{
        let mut j = 1;
        if totient[i]==i {
            loop {
                if j * i > n { break; }
                totient[i * j] -= totient[i * j] / i;
                j += 1;
            }
        }
    }
    totient
}
/// Get All Divisors
/// Returns a sorted list of all divisors of n.
/// Time: O(sqrt(n))
pub fn divisors(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i * i != n { res.push(n / i); }
        }
        i += 1;
        if res.len()>5{break};
    }
    res.sort_unstable();
    res
}

fn f(a:usize,b:usize)->(usize,Vec<usize>,usize){
    if b==0 {
        if a!=1{
            (MOD,Vec::new(),0)
        }
        else{
            (0,Vec::new(),0)
        }
    }else if b==1{
        (a-2,vec![1,a-1],a)
    }
    else{
        let mut r = f(b,a%b);
        r.1.push(a / b);
        (r.0+a/b-1,r.1,r.2+a/b)
    }
}

const MOD: usize = 1_000_000_007;
fn main() {
    let (n,r):(usize,usize) = int2();
    if n==1 && r==1{
        print!("0\nT\n");
    }
    else if let Some(v) = (0..r).filter_map(|i|{let o = f(r,i);if o.0<MOD && o.2==n{Some(o)} else {None}}).min_by_key(|p|(*p).0){
        print!("{}\n",v.0);
        let mut k = true;
        for i in  v.1{
            for _ in 0..i{ print!("{}",if k {"T"} else {"B"});}
            k = !k;
        }
    }
    else {
        print!("IMPOSSIBLE\n");
    }
}
