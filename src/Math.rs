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

/// Least Common Multiple
/// Optimization: Divides by GCD first to avoid overflow, casts to u128 for safety.
/// Time: O(log(min(a, b)))
#[inline]
pub fn lcm(a: u64, b: u64) -> u64 {
	if a == 0 || b == 0 { return 0; }
	((a as u128 * b as u128) / gcd(a, b) as u128) as u64
}

/// Extended Euclidean Algorithm
/// Returns (g, x, y) such that a*x + b*y = g
#[inline]
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
	if b == 0 {
		(a, 1, 0)
	} else {
		let (g, x1, y1) = ext_gcd(b, a % b);
		(g, y1, x1 - (a / b) * y1)
	}
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
	let (g, x, _) = ext_gcd(n, m);
	if g != 1 { None } else { Some((x % m + m) % m) }
}

/// Primality Test
/// Optimization: Checks 2, 3, then iterates 6k +/- 1.
/// Time: O(sqrt(n))
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

/// Linear Sieve
/// Returns (spf, primes)
/// spf[i] = Smallest Prime Factor of i
/// primes = List of all primes up to n
/// Time: O(nlog(log(n)))
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