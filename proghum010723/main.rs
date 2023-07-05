
const MAX_NUM: usize = 100;

/// An array of function pointers (or closures) that take a number, a cache of primes, and the last computed sqrt
/// 
/// The Difficult tier requires a cache of primes (to avoid recomputing primes), so we need to pass it to everything 😩
/// 
/// `cache` is an array of tri-state `Option<bool>`s, where `None` means "not computed", `Some(true)` means "prime", and `Some(false)` means "not prime"
/// The length is MAX_NUM+7 because we need to cache the primes up to 100, plus look-aheads for the Difficult tier
const CHECKS: [(fn(usize, &mut [Option<bool>; MAX_NUM+7], &mut usize) -> bool, &'static str); 6] = [
	// Easy
	(|i, _, _| i % 3 == 0, "Fizz"),
	(|i, _, _| i % 5 == 0, "Buzz"),
	// Intermediate
	(|i, _, _| i % 7 == 0, "Rizz"),
	(|i, _, _| i % 11 == 0, "Jazz"),
	(|i, _, _| (120 / i) * i == 120, "Dizz"),
	// Difficult
	(difficult, "Prizz"),
];

fn main() {
	// primes[0] is never accessed, but it's easier to just have it there
	let (mut primes, mut last_sqrt) = ([None; MAX_NUM+7], 1);
	for i in 1..=MAX_NUM {
		let mut paren = false;
		for check in CHECKS {
			let res = check.0(i, &mut primes, &mut last_sqrt);
			paren |= res;
			// Avoid if statements by indexing into an array with the boolean instead
			print!("{}", ["", check.1][res as usize]);
		}
		println!("{}", [&i.to_string(),""][paren as usize]);
	}
}

/// The difficult check is a bit more complicated, so it gets its own function
fn difficult(i: usize, cache: &mut [Option<bool>; MAX_NUM+7], last_sqrt: &mut usize) -> bool {
	// These lookaheads pre-populate the cache, before `i` itself is checked
	let prepop_max = (i+1..).find(|x| x % 11 == 0 || x % 7 == 0).expect("How did this overflow??");

	// `i` is prime, but none of the lookaheads are
	check_prime(i, cache, last_sqrt) && !(i+1..prepop_max).any(|x| check_prime(x, cache, last_sqrt))
}

fn check_prime(i: usize, primes: &mut [Option<bool>; MAX_NUM+7], last_sqrt: &mut usize) -> bool {
	// increment the sqrt if it's too low
	*last_sqrt += ((*last_sqrt+1).pow(2) <= i) as usize;

	// Get the cached result, or compute it. the `filter` will skip non-prime factors
	primes[i] = primes[i].or_else(|| Some([(2..=*last_sqrt).filter(|x| !matches!(primes[*x], Some(false))).find(|j| i % j == 0), Some(0)][(*last_sqrt == i) as usize].is_none()));

	// Re-access the cache. Technically inefficient, but not a good way to do this without if statements
	primes[i].unwrap()
}

#[cfg(test)]
mod test {
    use crate::{check_prime, MAX_NUM};

	#[test]
	fn test_check_prime() {
		let (mut primes, mut last_sqrt) = ([None; MAX_NUM+7], 1);
		let mut confirmed = Vec::new();
		for i in 1..=100 {
			if check_prime(i, &mut primes, &mut last_sqrt) {
				confirmed.push(i);
			}
		}
		assert_eq!(last_sqrt, 10);

		assert_eq!(primes.iter().enumerate().filter_map(|(i, x)| if x == &Some(true) {Some(i)} else {None}).collect::<Vec<usize>>(), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
			31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
			73, 79, 83, 89, 97]);

	}
}