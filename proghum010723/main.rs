
const MAX_NUM: usize = 100;

const CHECKS: [(&'static str, fn(usize, &mut [Option<bool>; MAX_NUM+7], &mut usize) -> bool); 6] = [
	// Easy
	("Fizz", |i, _, _| i % 3 == 0),
	("Buzz", |i, _, _| i % 5 == 0),
	// Intermediate
	("Rizz", |i, _, _| i % 7 == 0),
	("Jazz", |i, _, _| i % 11 == 0),
	("Dizz", |i, _, _| (120 / i) * i == 120),
	// Difficult
	("Prizz", difficult),
];

fn main() {
	// primes[0] is never accessed, but it's easier to just have it there
	let (mut primes, mut last_sqrt) = ([None; MAX_NUM+7], 1);
	for i in 1..=MAX_NUM {
		let mut has_printed = 0;
		for (text, _) in CHECKS.iter().filter(|(_, check)| check(i, &mut primes, &mut last_sqrt)) {
			print!("{}", text);
			has_printed = 1;
		}
		println!("{}", [&i.to_string(),""][has_printed]);
	}
}

/// The difficult check is a bit more complicated, so it gets its own function
fn difficult(i: usize, cache: &mut [Option<bool>; MAX_NUM+7], last_sqrt: &mut usize) -> bool {
	// Find the next multiple of 7 or 11
	let next_mult = (i+1..).find(|x| x % 11 == 0 || x % 7 == 0).expect("How did this overflow??");

	is_prime(i, cache, last_sqrt) && !(i+1..next_mult).any(|x| is_prime(x, cache, last_sqrt))
}

// Check if `i` is prime, cache the result, and return it
fn is_prime(i: usize, primes: &mut [Option<bool>; MAX_NUM+7], last_sqrt: &mut usize) -> bool {
	// increment the sqrt if it's too low
	*last_sqrt += ((*last_sqrt+1).pow(2) <= i) as usize;

	// Get the cached result, or compute it. the `filter` will skip non-prime factors
	primes[i] = primes[i].or_else(|| Some([(2..=*last_sqrt).filter(|x| !matches!(primes[*x], Some(false))).find(|j| i % j == 0), Some(0)][(*last_sqrt == i) as usize].is_none()));

	primes[i].unwrap()
}

#[cfg(test)]
mod test {
    use crate::{is_prime, MAX_NUM};

	#[test]
	fn test_is_prime() {
		let (mut primes, mut last_sqrt) = ([None; MAX_NUM+7], 1);
		let mut confirmed = Vec::new();
		for i in 1..=100 {
			if is_prime(i, &mut primes, &mut last_sqrt) {
				confirmed.push(i);
			}
		}
		assert_eq!(last_sqrt, 10);

		assert_eq!(primes.iter().enumerate().filter_map(|(i, x)| if x == &Some(true) {Some(i)} else {None}).collect::<Vec<usize>>(), 
			vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
	}
}
