use std::mem;

/// Greatest Common Divisor  
/// Time: O(log(min(a, b)))  
#[inline]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
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
pub fn gcd_i(mut a: i64, mut b: i64) -> i64 {
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
pub fn gcd_i_arr(a:&[i64]) -> i64 {
	let mut g = a[0];
	for &i in a {
		g = gcd_i(g, i);
	}g
}

/// for i64  
/// Greatest Common Divisor  
/// Time: O(n log(min(a, b)))  
#[inline]
pub fn gcd_arr(a:&[u64]) -> u64 {
	let mut g = a[0];
	for &i in a {
		g = gcd(g, i);
	}g
}

/// Least Common Multiple  
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.  
/// Time: O(log(min(a, b)))  
#[inline]
pub fn lcm(a: u64, b: u64) -> u64 {
	if a == 0 || b == 0 { return 0; }
	((a as u128 * b as u128) / gcd(a, b) as u128) as u64
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

/// Least Common Multiple  
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.  
/// Time: O(n log(min(a, b)))  
#[inline]
pub fn lcm_arr(a:&[u64]) -> u64 {
	let mut l = a[0];
	for &i in a {
		l = lcm(l, i);
	}l
}


/// Extended Euclidean Algorithm  
/// Returns (g, x, y) such that a*x + b*y = g  
#[inline]
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

/// Safe Modular Multiplication (a * b % m)  
/// Uses u128 to strictly prevent overflow.  
#[inline]
pub fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
	((a as u128 * b as u128) % m as u128) as u64
}

/// Modular Exponentiation (base^exp % m)  
/// Time: O(log exp)  
#[inline]
pub fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
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
pub fn mod_inv(n: u64, m: u64) -> u64 {
	mod_pow(n, m - 2, m)
}

/// General Modular Inverse (Extended Euclidean)  
/// Works even if m is NOT prime. Returns None if inverse doesn't exist.  
#[inline]
pub fn mod_inv_general(n: i64, m: i64) -> Option<i64> {
	let (g, x, _) = extended_gcd_iterative(n, m);
	if g != 1 { None } else { Some((x % m + m) % m) }
}

/// Primality Test  
/// Optimization: Checks 2, 3, then iterates 6k +/- 1.  
/// Time: O(sqrt(n))  
#[inline]
pub fn is_prime(n: u64) -> bool {
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
#[inline]
pub fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
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
pub fn divisors(n: u64) -> Vec<u64> {
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

/// Euler's Totient Function (Phi)  
/// Count of numbers <= n coprime to n.  
/// Time: O(sqrt(n))  
#[inline]
pub fn phi_euler(mut n: u64) -> u64 {
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
/// spf[i] = Smallest Prime Factor of i;  
/// primes = List of all primes up to n;  
/// Time: O(nlog(log(n)))  
#[inline]
pub fn linear_sieve(n: usize) -> (Vec<usize>, Vec<usize>) {
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

/// Linear Sieve to precompute euler totient for numbers upto n  
/// Time: O(nlog(log(n)))  
#[inline]
pub fn linear_sieve_totient(n: usize) -> (Vec<usize>, Vec<usize>) {
	let mut totient = [0;n+1];
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

/// Precompute Factorials and Inverse Factorials  
/// Returns (fact, inv_fact) vectors  
/// Time: O(n)  
#[inline]
pub fn precompute_factorials(n: usize, m: u64) -> (Vec<u64>, Vec<u64>) {
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
pub fn ncr(n: usize, r: usize, fact: &[u64], inv_fact: &[u64], m: u64) -> u64 {
	if r > n { return 0; }
	let num = fact[n];
	let den = mul_mod(inv_fact[r], inv_fact[n - r], m);
	mul_mod(num, den, m)
}

/// Calculate nPr efficiently  
/// Time: O(1)  
#[inline]
pub fn npr(n: usize, r: usize, fact: &[u64], inv_fact: &[u64], m: u64) -> u64 {
	if r > n { return 0; }
	mul_mod(fact[n], inv_fact[n - r], m)  
}

/// Lucas Theorem  
/// REQUIRES: p is prime (small)  
/// REQUIRES: fact and inv_fact are precomputed up to p-1  
/// Time: O(p + log_p(n))  
/// Calculates nCr where n and r are really big  
/// Precomputing factorial upto n is not possible
#[inline] 
pub fn ncr_lucas(mut n: u64, mut r: u64, p: u64, fact: &[u64], inv_fact: &[u64]) -> u64 {
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
pub fn convert_to_base(mut num: u64, to_base: u64) -> Result<Vec<u64>,u64> {
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
