use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::future::pending;
use std::io::{stdin, stdout, Write};
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Div, Sub, SubAssign};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::thread::current;
use std::vec;

fn int<T: FromStr>() -> T {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok().unwrap()
}
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
fn int3<T, U,V>() -> (T, U,V) where
    T: FromStr,
    U: FromStr,
    V: FromStr,
    T::Err: Debug,
    U::Err: Debug,
    V::Err: Debug, {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut it = input.split_whitespace();
    (
        it.next().unwrap().parse::<T>().unwrap(),
        it.next().unwrap().parse::<U>().unwrap(),
        it.next().unwrap().parse::<V>().unwrap(),
    )
}
fn int4<T, U,V,W>() -> (T, U,V,W) where
    T: FromStr,
    U: FromStr,
    V: FromStr,
    W: FromStr,
    T::Err: Debug,
    U::Err: Debug,
    V::Err: Debug,
    W::Err: Debug, {
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
    M::Err: Debug, {
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
#[inline]
fn array<T: FromStr,B:FromIterator<T>>() -> B {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
}
fn int_array<T: FromStr,B:FromIterator<T>>() -> (T,B)  where <T as FromStr>::Err: Debug{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut ll = input.trim().split_whitespace();
    (ll.next().unwrap().parse::<T>().unwrap(),ll.map(|x| x.parse::<T>().ok().unwrap()).collect())
}
fn word()->String{
    let mut inp = String::new();
    stdin().read_line(&mut  inp).unwrap();
    inp.trim().to_string()
}
fn word2() -> (String,String) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut a = input.trim().split_whitespace();
    (a.next().unwrap().to_string(),a.next().unwrap().to_string())
}
fn int_word<T>() -> (T, String) where
    T: FromStr,
    T::Err: Debug, {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut it = input.split_whitespace();
    (
        it.next().unwrap().parse::<T>().unwrap(),
        it.next().unwrap().to_owned()
    )
}


/// Extended Euclidean Algorithm
/// Returns (g, x, y) such that a*x + b*y = g
pub fn extended_gcd_iterative(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut x, mut last_x) = (0, 1);
    let (mut y, mut last_y) = (1, 0);

    while b != 0 {
        let q = a / b;

        // Update a and b
        (a, b) = (b, a % b);

        // Update coefficients without extra named variables
        (x, last_x) = (last_x - q * x, x);
        (y, last_y) = (last_y - q * y, y);
    }

    (a, last_x, last_y)
}

///Bezout's Identity generalised
///O(n^2)
/// it is possible to optimise to O(n) will do it soon..
fn bezout_cofficients(arr:&[i64])->(i64,Vec<i64>){
    if arr.len()==0{
        (0,vec![])
    }
    else if arr.len()==1{
        (arr[0],vec![1])
    }
    else{
        let mut g = arr[0];
        let mut coff:Vec<i64> = vec![1];
        coff.reserve(arr.len());
        for i in 2..arr.len(){
            let x:i64;
            let y:i64;
            (g,x,y) = extended_gcd_iterative(g,arr[i]);
            for i in &mut coff{
                *i*=x;
            }
            coff.push(y);
        }
        (g,coff)
    }
}


/// General Modular Inverse (Extended Euclidean)  
/// Works even if m is NOT prime. Returns None if inverse doesn't exist.  
#[inline]
pub fn mod_inv_general(n: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd_iterative(n, m);
    if g != 1 { None } else { Some((x % m + m) % m) }
}

/// Least Common Multiple
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.
/// Time: O(log(min(a, b)))
#[inline]
pub fn lcm_i(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 { return 0; }
    ((a as i128 * b as i128) / gcd_i(a, b) as i128) as i64
}

/// Least Common Multiple  
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.  
/// Time: O(n log(min(a, b)))  
#[inline]
pub fn lcm_i_arr(a:&[i64]) -> i64 {
    let mut l = a[0];
    for &i in a {
        l = lcm_i(l, i);
    }l
}

/// for i64  
/// Greatest Common Divisor  
/// Time: O(log(min(a, b)))  
#[inline]
pub fn gcd_i(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// X = a1 (mod n1)  
/// X = a2 (mod n2)  
/// ...  
/// X = ak (mod nk)  
///
/// N = ∏ ni [i = 1,2,...,k]  
/// Ni = ∏ nj [j != i] = N/ni  
/// bi = inverse of Ni mod ni  
/// X = Σai\*Ni\*bi  
#[inline]
fn chinese_remainder_theorem_know_that_n_are_coprime(a:&[i64],n:&[i64])->i128
{
    assert_eq!(a.len(),n.len());
    let N = n.iter().fold(1,|a,&b|a*b);
    (0..n.len()).map(|i|{
        let ni = n[i];
        let Ni = N/ni;
        let bi = mod_inv_general(Ni,ni).unwrap();
        (a[i]*Ni*bi) as i128
    }).sum::<i128>()%(N as i128)
}

/// General Chinese Remainder Theorem for an arbitrary number of equations.  
/// Works even if moduli are NOT pairwise coprime.  
/// uses 2 variable chinese_remainder_theorem function 
/// Returns Option<(final_remainder, final_lcm)> .i.e X=final_remainder (mod final lcm)
pub fn chinese_remainder_theorem_general(a: &[u64], m: &[u64]) -> Option<(u64, u64)> {
    assert_eq!(a.len(), m.len(), "Arrays must be of equal length");
    if a.is_empty() {
        return None;
    }
    let mut curr_a = a[0];
    let mut curr_m = m[0];

    for i in 1..a.len() {
        (curr_a, curr_m) = chinese_remainder_theorem(curr_a, a[i], curr_m, m[i])?;
    }
    Some((curr_a, curr_m))
}

/// X = a (mod m1)  
/// X = b (mod m2)  
/// x\*m1+ y\*m2 = g  
/// X = a\*x\*(m1/g) + b\*y\*(m2/g)  

/// X = a (mod m1)  
/// X = b (mod m2)  
/// x\*m1+ y\*m2 = g  
/// X = a\*x\*(m1/g) + b\*y\*(m2/g)  

#[inline]
fn chinese_remainder_theorem(a:u64, b:u64, m1:u64, m2:u64)->Option<(u64,u64)>{

    let (g,x,y) = extended_gcd_iterative(m1 as i64,m2 as i64);
    let g = g as u64;
    let lcm = (m1/g*m2) as i64;
    let x = ((x%lcm+lcm)%lcm) as u64;
    let y = ((y%lcm+lcm)%lcm) as u64;
    let lcm = lcm as u64;
    if b.abs_diff(a)%g!=0 {None}
    else{
        Some((((a.widening_mul(y)*m2/g+b.widening_mul(x)*m1/g)%lcm as u128) as u64,lcm))
    }
}

fn main() {
    println!("=========================================");
    println!("🧪 RUNNING EDGE CASE TEST SUITE");
    println!("=========================================\n");

    // ---------------------------------------------------------
    // 1. GCD & LCM EDGE CASES
    // ---------------------------------------------------------
    println!("--- Testing GCD & LCM Edge Cases ---");

    assert_eq!(gcd_i(0, 5), 5);
    assert_eq!(gcd_i(12, 0), 12);
    assert_eq!(gcd_i(0, 0), 0);
    println!("✅ GCD with Zeros passed.");

    assert_eq!(lcm_i(0, 5), 0);
    assert_eq!(lcm_i(12, 0), 0);
    assert_eq!(lcm_i_arr(&[0, 5, 10]), 0);
    println!("✅ LCM with Zeros passed.");

    assert_eq!(gcd_i(7, 7), 7);
    assert_eq!(lcm_i(7, 7), 7);
    println!("✅ GCD/LCM with identical numbers passed.");


    // ---------------------------------------------------------
    // 2. MODULAR INVERSE EDGE CASES
    // ---------------------------------------------------------
    println!("\n--- Testing Modular Inverse Edge Cases ---");

    assert_eq!(mod_inv_general(5, 1), Some(0)); // Modulo 1
    assert_eq!(mod_inv_general(1, 13), Some(1)); // Inverse of 1
    assert_eq!(mod_inv_general(-2, 7), Some(3)); // Negative numbers
    println!("✅ Modular Inverse bounds and negatives passed.");


    // ---------------------------------------------------------
    // 3. STANDARD CRT (COPRIME ASSUMED) EDGE CASES
    // ---------------------------------------------------------
    println!("\n--- Testing Standard CRT (Coprime) ---");

    // Standard valid case: X = 2(mod 3), X = 3(mod 5), X = 2(mod 7) -> 23
    assert_eq!(
        chinese_remainder_theorem_know_that_n_are_coprime(&[2, 3, 2], &[3, 5, 7]),
        23
    );
    println!("✅ Standard coprime system passed.");

    // All remainders are 0 -> Answer should be 0
    assert_eq!(
        chinese_remainder_theorem_know_that_n_are_coprime(&[0, 0, 0], &[3, 5, 7]),
        0
    );
    println!("✅ Zero remainders passed.");

    // Single equation -> X = 5 (mod 11)
    assert_eq!(
        chinese_remainder_theorem_know_that_n_are_coprime(&[5], &[11]),
        5
    );
    println!("✅ Single equation passed.");


    // ---------------------------------------------------------
    // 4. GENERAL CRT EDGE CASES
    // ---------------------------------------------------------
    println!("\n--- Testing General CRT Edge Cases ---");

    // Empty Arrays
    assert_eq!(chinese_remainder_theorem_general(&[], &[]), None);
    println!("✅ Empty arrays passed (Returned None).");

    // Single Equation
    assert_eq!(
        chinese_remainder_theorem_general(&[5], &[7]),
        Some((5, 7))
    );
    println!("✅ Single equation passed.");

    // Zero remainders
    assert_eq!(
        chinese_remainder_theorem_general(&[0, 0], &[4, 6]),
        Some((0, 12))
    );
    println!("✅ Zero remainders passed.");

    // Redundant Equations
    assert_eq!(
        chinese_remainder_theorem_general(&[3, 3], &[5, 5]),
        Some((3, 5))
    );
    println!("✅ Redundant equations passed.");

    // Direct Contradiction
    assert_eq!(
        chinese_remainder_theorem_general(&[2, 3], &[5, 5]),
        None
    );
    println!("✅ Direct contradiction passed (Returned None).");

    // Modulo 1
    assert_eq!(
        chinese_remainder_theorem_general(&[0, 4], &[1, 7]),
        Some((4, 7))
    );
    println!("✅ Trivial Modulo 1 passed.");


    // ---------------------------------------------------------
    // 5. EXTREME LIMITS / OVERFLOW SAFETY
    // ---------------------------------------------------------
    println!("\n--- Testing Extreme Limits (General CRT u64 boundaries) ---");

    let m1: u64 = 1_000_000_007;
    let m2: u64 = 1_000_000_009;
    let a1: u64 = 123;
    let a2: u64 = 456;

    match chinese_remainder_theorem_general(&[a1, a2], &[m1, m2]) {
        Some((x, lcm)) => {
            assert_eq!(x % m1, a1);
            assert_eq!(x % m2, a2);
            assert_eq!(lcm, m1 * m2);
            println!("✅ Massive Moduli passed without overflowing! (X = {}, LCM = {})", x, lcm);
        },
        None => panic!("Massive Moduli test failed unexpectedly."),
    }

    println!("\n🎉 ALL EDGE CASES PASSED SUCCESSFULLY!");
}