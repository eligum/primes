/*!
A basic library for working with prime numbers.

This library provides methods for generating primes, testing whether a number is prime, and
factorizing numbers. Most methods generate primes lazily, so only enough primes will be generated
for the given test, and primes are cached for later use.
*/

use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Index;
use std::slice;

pub trait PrimeSetBasics {
	/// Finds one more prime, and adds it to the list.
	fn expand(&mut self);

	/// Returns all primes found so far as a slice.
	fn list(&self) -> &[u64];
}

/**
A prime generator, using the Trial Division method.

Create with `let mut pset = TrialDivision::new()`, and then use `pset.iter()` to iterate over all primes.
**/
#[derive(Default, Clone)]
pub struct TrialDivision {
	lst: Vec<u64>,
}

pub struct PrimeSetIter<'a, P: PrimeSet> {
	p: &'a mut P,
	n: usize,
	expand: bool,
}

impl TrialDivision {
	/// A new prime generator, primed with 2 and 3.
	pub fn new() -> TrialDivision {
		TrialDivision { lst: vec![2, 3] }
	}
}

impl PrimeSetBasics for TrialDivision {
	/// Finds one more prime and adds it to the list.
	fn expand(&mut self) {
		let mut l: u64 = self.lst.last().unwrap() + 2;
		let mut remainder = 0;
		loop {
			for &n in &self.lst {
				remainder = l % n;
				if remainder == 0 || n * n > l {
					break;
				}
			}
			if remainder != 0 {
				self.lst.push(l);
				break;
			}
			l += 2;
		}
	}

	/// Returns all primes found so far as a slice.
	fn list(&self) -> &[u64] {
		&self.lst[..]
	}
}

pub trait PrimeSet: PrimeSetBasics + Sized {
	/// Number of primes found so far.
	fn len(&self) -> usize {
		self.list().len()
	}

	fn is_empty(&self) -> bool {
		self.list().is_empty()
	}

	/// Iterator over all primes not yet found.
	fn generator(&mut self) -> PrimeSetIter<Self> {
		let n = self.len();
		PrimeSetIter {
			p: self,
			n,
			expand: true,
		}
	}

	/// Iterator over all primes, starting with 2. If you don't care about the "state" of the
	/// `PrimeSet`, this is what you want!
	fn iter(&mut self) -> PrimeSetIter<Self> {
		PrimeSetIter {
			p: self,
			n: 0,
			expand: true,
		}
	}

	/// Iterator over just the primes found so far.
	fn iter_vec(&self) -> slice::Iter<u64> {
		self.list().iter()
	}

	/// Find the next largest prime from a number
    ///
    /// Returns `(idx, prime)`
    ///
    /// Note that if `n` is prime, then the output will be `(idx, n)`
	fn find(&mut self, n: u64) -> (usize, u64) {
		while n > *(self.list().last().unwrap_or(&0)) {
			self.expand();
		}
		self.find_vec(n).unwrap()
	}

    /// Find the next largest prime from a number, if it is within the already-found list
    ///
    /// Returns `(idx, prime)`
    ///
    /// Note that if `n` is prime, then the output will be `(idx, n)`
	fn find_vec(&self, n: u64) -> Option<(usize, u64)> {
		if n > *(self.list().last().unwrap_or(&0)) {
			return None;
		}

		let mut base: usize = 0;
		let mut lim: usize = self.len();

		// Binary search algorithm
		while lim != 0 {
			let idx = base + (lim >> 1);
			match self.list()[idx].cmp(&n) {
				Equal => return Some((idx, self.list()[idx])),
				Less => {
					base = idx + 1;
					lim -= 1;
				},
				Greater => (),
			}
			lim = lim >> 1;
		}
		Some((base, self.list()[base]))
	}

	/// Get the nth prime, even if we haven't found it yet.
	fn get(&mut self, index: usize) -> u64 {
		for _ in 0..(index as isize) + 1 - (self.len() as isize) {
			self.expand();
		}
		self.list()[index]
	}
}

// This line implements `PrimeSet` trait for all types in scope that implement `PrimeSetBasics`.
impl<P: PrimeSetBasics> PrimeSet for P {}

impl Index<usize> for TrialDivision {
	type Output = u64;
	fn index(&self, index: usize) -> &u64 {
		&self.list()[index]
	}
}

impl<'a, P: PrimeSet> Iterator for PrimeSetIter<'a, P> {
	type Item = u64;
	fn next(&mut self) -> Option<u64> {
		while self.n >= self.p.list().len() {
			if self.expand {
				self.p.expand();
			} else {
				return None;
			}
		}
		self.n += 1;

		Some(self.p.list()[self.n - 1])
	}
}


/// Find the first factor (other than 1) of a number.
fn firstfac(x: u64) -> u64 {
	if x % 2 == 0 {
		return 2;
	}
	for d in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
		if x % d == 0 {
			return d;
		}
	}
	// No factor found, it must be prime.
	x
}

/// Find all prime factors of a number.
pub fn factors(mut x: u64) -> Vec<u64> {
	if x <= 1 {
		return vec![];
	}
	let mut lst: Vec<u64> = Vec::new();
	loop {
		let d = firstfac(x);
		lst.push(d);
		if d == x {
			break;
		} else {
			x /= d;
		}
	}
	lst
}

/// Find all unique prime factors of a number.
pub fn factors_unique(mut x: u64) -> Vec<u64> {
	if x <= 1 {
		return vec![];
	}
	let mut lst: Vec<u64> = Vec::new();
	loop {
		let d = firstfac(x);
		lst.push(d);
		if d == x {
			break;
		}
		while x % d == 0 {
			x /= d;
		}
		if x == 1 {
			break;
		}
	}
	lst
}

/// Tests whether a number is prime. Checks every odd number up to `sqrt(n)`.
pub fn is_prime(n: u64) -> bool {
	n > 1 && firstfac(n) == n
}
