use std::mem;
use std::random::random;

/// Greatest Common Divisor  
/// Time: O(log(min(a, b)))  
#[inline]
fn gcd(mut a: u64, mut b: u64) -> u64 {
	while b != 0 {
		let r = a % b;
		a = b;
		b = r;
	}
	a
}
/// for i64  
/// Greatest Common Divisor  
/// Time: O(log(min(a, b)))  
#[inline]
fn gcd_i(mut a: i64, mut b: i64) -> i64 {
	while b != 0 {
		let r = a % b;
		a = b;
		b = r;
	}
	a
}
/// for i64  
/// Greatest Common Divisor  
/// Time: O(n log(min(a, b)))  
#[inline]
fn gcd_i_arr(a:&[i64]) -> i64 {
	let mut g = a[0];
	for &i in a {
		g = gcd_i(g, i);
	}g
}

/// for i64  
/// Greatest Common Divisor  
/// Time: O(n log(min(a, b)))  
#[inline]
fn gcd_arr(a:&[u64]) -> u64 {
	let mut g = a[0];
	for &i in a {
		g = gcd(g, i);
	}g
}

/// Least Common Multiple  
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.  
/// Time: O(log(min(a, b)))  
#[inline]
fn lcm(a: u64, b: u64) -> u64 {
	if a == 0 || b == 0 { return 0; }
	((a * b) / gcd(a, b)) 
}

/// Least Common Multiple
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.
/// Time: O(log(min(a, b)))
#[inline]
fn lcm_i(a: i64, b: i64) -> i64 {
	if a == 0 || b == 0 { return 0; }
	((a as i128 * b as i128) / gcd_i(a, b) as i128) as i64
}

/// Least Common Multiple  
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.  
/// Time: O(n log(min(a, b)))  
#[inline]
fn lcm_i_arr(a:&[i64]) -> i64 {
	let mut l = a[0];
	for &i in a {
		l = lcm_i(l, i);
	}l
}

/// Least Common Multiple  
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.  
/// Time: O(n log(min(a, b)))  
#[inline]
fn lcm_arr(a:&[u64]) -> u64 {
	let mut l = a[0];
	for &i in a {
		l = lcm(l, i);
	}l
}


/// Extended Euclidean Algorithm  
/// Returns (g, x, y) such that a*x + b*y = g  
#[inline]
fn extended_gcd_iterative(mut a: i64, mut b: i64) -> (i64, i64, i64) {
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

/// Safe Modular Multiplication (a * b % m)  
/// Uses u128 to strictly prevent overflow.  
#[inline]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
	((a as u128 * b as u128) % m as u128) as u64
}

/// Modular Exponentiation (base^exp % m)  
/// Time: O(log exp)  
#[inline]
fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
	let mut res = 1;
	base %= m;
	while exp > 0 {
		if exp % 2 == 1 { res = mul_mod(res, base, m); }
		base = mul_mod(base, base, m);
		exp /= 2;
	}
	res
}

/// Modular Inverse (Fermat's Little Theorem)  
/// REQUIRES: m is Prime.  
/// Time: O(log m)  
#[inline]
fn mod_inv(n: u64, m: u64) -> u64 {
	mod_pow(n, m - 2, m)
}

/// General Modular Inverse (Extended Euclidean)  
/// Works even if m is NOT prime. Returns None if inverse doesn't exist.  
#[inline]
fn mod_inv_general(n: i64, m: i64) -> Option<i64> {
	let (g, x, _) = extended_gcd_iterative(n, m);
	if g != 1 { None } else { Some((x % m + m) % m) }
}

/// Primality Test  
/// Optimization: Checks 2, 3, then iterates 6k +/- 1.  
/// Time: O(sqrt(n))  
#[inline]
fn is_prime(n: u64) -> bool {
	if n <= 1 { return false; }
	if n <= 3 { return true; }
	if n % 2 == 0 || n % 3 == 0 { return false; }
	let mut i = 5;
	while i * i <= n {
		if n % i == 0 || n % (i + 2) == 0 { return false; }
		i += 6;
	}
	true
}

/// Prime Factorization  
/// Returns vector of (prime, exponent).  
/// Optimization: Handles 2, then skips even numbers.  
/// Time: O(sqrt(n))  
/// 1 cannot be a prime factor  
#[inline]
fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
	let mut factors = Vec::new();
	if n % 2 == 0 {
		let mut cnt = 0;
		while n % 2 == 0 { n /= 2; cnt += 1; }
		factors.push((2, cnt));
	}
	let mut i = 3;
	while i * i <= n {
		if n % i == 0 {
			let mut cnt = 0;
			while n % i == 0 { n /= i; cnt += 1; }
			factors.push((i, cnt));
		}
		i += 2;
	}
	if n > 1 { factors.push((n, 1)); }
	factors
}

/// Get All Divisors  
/// Returns a sorted list of all divisors of n.  
/// Time: O(sqrt(n))  
#[inline]
fn divisors(n: u64) -> Vec<u64> {
	let mut res = Vec::new();
	let mut i = 1;
	while i * i <= n {
		if n % i == 0 {
			res.push(i);
			if i * i != n { res.push(n / i); }
		}
		i += 1;
	}
	res.sort_unstable();
	res
}

/// computes divisors upto n (inclusive)  
/// O(nlog(n))
fn precompute_divisors(n:usize) -> Vec<Vec<usize>> {
	let mut divisors : Vec<Vec<usize>> = vec![Vec::with_capacity((n.ilog2() + 1) as usize); n+1];
	for i in 1..=n{
		for j in (i..=n).step_by(i){
			divisors[j].push(i);
		}
	}divisors
}

/// computes distinct prime factors up to n (inclusive)
/// O(n log(log(n)))
fn precompute_prime_factors(n: usize) -> Vec<Vec<usize>> {
	let mut prime_factors: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
	for i in 2..=n {
		if prime_factors[i].is_empty() {
			// i is prime, mark it as a factor for all its multiples
			for j in (i..=n).step_by(i) {
				prime_factors[j].push(i);
			}
		}
	}
	prime_factors
}

///number of divisors = ∏ (exponent+1)
/// 
fn number_of_divisors_from_prime_factors(a:&Vec<(u64,u32)>)->u32{
	a.iter().fold(1,|r,&i|i.1*(r+1))
}

///Sum = ∏ (prime^(exponent+1)-1)/(prime-1)
/// 
fn sum_of_divisors_from_prime_factors(a:&Vec<(u64,u32)>)->u64{
	a.iter().fold(1,|r,&i|r*(i.0.pow(i.1+1)-1)/(i.0-1))
}
/// Euler's Totient Function (Phi)  
/// Count of numbers <= n coprime to n.  
/// Time: O(sqrt(n))  
#[inline]
fn phi_euler(mut n: u64) -> u64 {
	let mut result = n;
	if n % 2 == 0 {
		while n % 2 == 0 { n /= 2; }
		result -= result / 2;
	}
	let mut i = 3;
	while i * i <= n {
		if n % i == 0 {
			while n % i == 0 { n /= i; }
			result -= result / i;
		}
		i += 2;
	}
	if n > 1 { result -= result / n; }
	result
}

/// Linear Sieve;  
/// Returns (spf, primes);  
/// `spf[i]` = Smallest Prime Factor of i;  
/// primes = List of all primes up to n;  
/// Time: O(n)  
#[inline]
fn linear_sieve(n: usize) -> (Vec<usize>, Vec<usize>) {
	let mut spf = vec![0; n + 1];
	let mut primes = Vec::new();

	// spf[1] is technically undefined or 1, leaving as 0 or 1 is fine usually.
	// 0 implies no prime factor found yet.

	for i in 2..=n {
		if spf[i] == 0 {
			spf[i] = i;
			primes.push(i);
		}
		for &p in &primes {
			if p > spf[i] || i * p > n {
				break;
			}
			spf[i * p] = p;
		}
	}
	(spf, primes)
}

/// A general Linear Sieve for any multiplicative function.  
///
/// * `n` - The upper bound to compute up to (inclusive).  
/// * `f_pk` - A closure `|p, k|` that returns the value of f(p^k) -> T.  
/// * `mul` - A closure `|f_a, f_b|` defines `f(a) * f(b)` [when a and b are coprime].
/// * `I` - Identity element of `T` (i.e., `mul(I, a) = a` for all a).
///
/// * Returns - `(spf, exp, rem, primes, f)`  
/// `spf[i]` is the smallest prime factor of i, and `i = rem[i] * spf[i]^exp[i]`
///
/// Time Complexity: O(n)  
/// Space Complexity: O(n)  
///
/// # Examples
///
/// ```
/// // Example 1: Computing the Möbius function (μ)
/// let n = 10;
/// let (_spf, _exp, _rem, _primes, mobius) = linear_sieve_multiplicative_general(
///     n,
///     |_p, k| if k == 1 { -1 } else { 0 }, // μ(p^k) is -1 if k=1, else 0
///     |a, b| a * b,                        // Standard integer multiplication
///     1i32,                                // Identity element for i32
/// );
///
/// // mobius[1..=6] will be: [1, -1, -1, 0, -1, 1]
/// assert_eq!(mobius[6], 1); // μ(6) = μ(2) * μ(3) = (-1) * (-1) = 1
///
///
/// // Example 2: Computing Euler's Totient function (φ)
/// let n = 10;
/// let (_spf, _exp, _rem, _primes, totient) = linear_sieve_multiplicative_general(
///     n,
///     |p, k| p.pow(k as u32 - 1) * (p - 1), // φ(p^k) = p^(k-1) * (p-1)
///     |a, b| a * b,                         // Standard integer multiplication
///     1usize,                               // Identity element for usize
/// );
///
/// // totient[1..=6] will be: [1, 1, 2, 2, 4, 2]
/// assert_eq!(totient[6], 2); // φ(6) = φ(2) * φ(3) = 1 * 2 = 2
/// ```
fn linear_sieve_multiplicative_general<T: Copy, F: Fn(usize, usize) -> T, G: Fn(T, T) -> T>(n: usize, f_pk: F, mul: G, identity: T,) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<T>)
{
	if n == 0 {
		return (vec![], vec![], vec![], vec![], vec![]);
	}

	let mut spf = vec![0; n + 1];
	let mut exp = vec![0; n + 1];
	let mut rem = vec![0; n + 1];
	let mut f = vec![identity; n + 1];
	let mut primes = Vec::new();

	// Base case: f(1) is always the identity for any non-zero multiplicative function
	f[1] = identity;

	for i in 2..=n {
		if spf[i] == 0 {
			spf[i] = i;
			exp[i] = 1;
			rem[i] = 1;
			f[i] = f_pk(i, 1);
			primes.push(i);
		}

		for &p in &primes {
			if p > spf[i] || i * p > n {
				break;
			}

			spf[i * p] = p;

			if p == spf[i] {
				// Case 1: p divides i. 
				// The exponent increases, and the coprime remainder stays the same.
				exp[i * p] = exp[i] + 1;
				rem[i * p] = rem[i];

				// f(p^(k+1) * m) = mul(f(p^(k+1)), f(m))
				f[i * p] = mul(f_pk(p, exp[i * p]), f[rem[i * p]]);
				break;
			} else {
				// Case 2: p does not divide i.
				// p is a brand new prime factor, so its exponent is 1, and the remainder is i.
				exp[i * p] = 1;
				rem[i * p] = i;

				// f(p^1 * i) = mul(f(p^1), f(i))
				f[i * p] = mul(f[p], f[i]);
			}
		}
	}

	(spf, exp, rem, primes, f)
}

/// Precompute Factorials and Inverse Factorials  
/// Returns (fact, inv_fact) vectors  
/// Time: O(n)  
#[inline]
fn precompute_factorials(n: usize, m: u64) -> (Vec<u64>, Vec<u64>) {
	let mut fact = vec![1; n + 1];
	let mut inv_fact = vec![1; n + 1];

	for i in 1..=n {
		fact[i] = mul_mod(fact[i - 1], i as u64, m);
	}

	// Compute inverse of n! once using Fermat's Little Theorem
	inv_fact[n] = mod_pow(fact[n], m - 2, m);

	// Compute inverses backwards: 1/(i-1)! = (1/i!) * i
	for i in (1..=n).rev() {
		inv_fact[i - 1] = mul_mod(inv_fact[i], i as u64, m);
	}

	(fact, inv_fact)
}

/// Calculate nCr efficiently  
/// Time: O(1)  
#[inline]
fn ncr(n: usize, r: usize, fact: &[u64], inv_fact: &[u64], m: u64) -> u64 {
	if r > n { return 0; }
	let num = fact[n];
	let den = mul_mod(inv_fact[r], inv_fact[n - r], m);
	mul_mod(num, den, m)
}

/// Calculate nPr efficiently  
/// Time: O(1)  
#[inline]
fn npr(n: usize, r: usize, fact: &[u64], inv_fact: &[u64], m: u64) -> u64 {
	if r > n { return 0; }
	mul_mod(fact[n], inv_fact[n - r], m)  
}

/// Lucas Theorem  
/// REQUIRES: p is prime (small)  
/// REQUIRES: fact and inv_fact are precomputed up to p-1  
/// Time: O(p + log_p(n))  
/// Calculates nCr where n and r are really big  
/// Precomputing factorial upto n is not possible  
/// # Statement  
/// it states that `nCr =  ∏ ni_C_ri (mod p)`  
/// where ni and ri are digits of n and r in base p  
#[inline] 
fn ncr_lucas(mut n: u64, mut r: u64, p: u64, fact: &[u64], inv_fact: &[u64]) -> u64 {
	if r > n { return 0; }

	let mut result = 1;

	while n > 0 || r > 0 {
		let ni = n % p;
		let ri = r % p;

		if ri > ni {
			return 0;
		}

		result = result * ncr(ni, ri, fact, inv_fact, p) % p;

		n /= p;
		r /= p;
	}

	result
}

/// General Chinese Remainder Theorem for an arbitrary number of equations.  
/// Works even if moduli are NOT pairwise coprime.  
/// uses 2 variable chinese_remainder_theorem function 
/// Returns Option<(final_remainder, final_lcm)> .i.e X=final_remainder (mod final lcm)
fn chinese_remainder_theorem_general(a: &[u64], m: &[u64]) -> Option<(u64, u64)> {
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
/// X = b\*x\*(m1/g) + a\*y\*(m2/g)  
/// *PROOF:*  
///    (X-a) divisible by m1  
///    (X-b) divisible by m2  
///    (b-a) is divisible by gcd(m1,m2)  
///    
///    X = b(1-y\*m2/g) + a(y\*m2/g) = b + (a-b)\*y\*m2/g  = b+k\*m2
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
		let a = a as u128;
		let y = y as u128;
		let m2 = m2 as u128;
		let g = g as u128;
		let b = b as u128;
		let x = x as u128;
		let m1 = m1 as u128;

		Some((((a*y*m2/g+b*x*m1/g)%lcm as u128) as u64,lcm))
	}
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

/// Converts a `u64` number to a vector of digits in the given base.  
/// The most significant digit will be at index 0.
#[inline] 
fn convert_to_base(mut num: u64, to_base: u64) -> Result<Vec<u64>,u64> {
	if to_base < 2 {
		return Err(to_base);
	}
	if num == 0 {
		return Ok(vec![0]);
	}

	let mut digits = Vec::with_capacity((num.ilog(to_base) + 3) as usize);

	while num > 0 {
		let remainder = num % to_base;
		digits.push(remainder);
		num /= to_base;
	}
	digits.reverse();
	Ok(digits)
}

/// Harmonic group (n) 
/// list of tuple (start,end,val)
/// such that all the numbers i between start and end (inclusive) floor(n/i) = val
#[inline]
fn harmonic_group(n:u64)->Vec<(u64,u64,u64)>{
	let ans : Vec<(u64,u64,u64)> =  Vec::with_capacity(2*n.isqrt() as usize);
	let mut start = 1;
	while start <= n{
		let val = n/ start;
		let end = n/val;
		ans.push((start,end,val));
		start = end+1;
	}
	ans
}
/// Harmonic group N = \[n_{1},n_{2},...,n_{i}\]  
/// list of tuple (start,end,vals) where vals is list of  
/// such that all the numbers i between start and end (inclusive) floor(N/i) = vals  
#[inline]
fn harmonic_group_extended(N:Vec<u64>)->Vec<(u64,u64,Vec<u64>)>{
	let &n = N.iter().max().unwrap();
	let ans : Vec<(u64,u64,Vec<u64>)> =  Vec::with_capacity(2*n.isqrt() as usize);
	let mut start = 1;
	while start <= n{
		let vals = N.iter().map(|&n|n/ start).collect();
		let end = (0..N.len()).map(|i|N[i]/vals[i]).min().unwrap();
		ans.push((start,end,vals));
		start = end+1;
	}
	ans
}


///binary exponenciation with a custom multiplication rule
#[inline]
fn binary_exponentiation<T:Clone,U:FnMut(&mut T,&T)>(base:T,exponent:usize,mut multiply:U)->Option<T>
{
	if exponent==0{
		return None;
	}
	else if exponent==1{
		return Some(base);
	}
	let mut Ans = base.clone();
	let mut bit = 1<<(usize::BITS-exponent.leading_zeros()-1);
	while bit>1{
		let temp = Ans.clone();
		multiply(&mut Ans,&temp);
		bit = bit>>1;
		if exponent&bit!=0{
			multiply(&mut Ans,&base);
		}
	}
	Some(Ans)
}
