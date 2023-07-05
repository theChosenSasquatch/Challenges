
const MAX_NUM: usize = 100;
type Cache = ([Option<bool>; MAX_NUM+7], usize);

const CHECKS: [(&str, fn(usize, &mut Cache) -> bool); 6] = [
	// Easy
	("Fizz", |i, _| i % 3 == 0),
	("Buzz", |i, _| i % 5 == 0),
	// Intermediate
	("Rizz", |i, _| i % 7 == 0),
	("Jazz", |i, _| i % 11 == 0),
	("Dizz", |i, _| (120 / i) * i == 120),
	// Difficult
	("Prizz", difficult),
];

fn main() {
	// primes[0] is never accessed, but it's easier to just have it there
	let mut cache: Cache = ([None; MAX_NUM+7], 1);
	for i in 1..=MAX_NUM {
		println!("{}", [&i.to_string(),""][CHECKS.iter().filter(|(_, check)| check(i, &mut cache)).fold(0, |_, (y, _)| {print!("{y}"); 1})]);
	}
}

/// The difficult check is a bit more complicated, so it gets its own function
fn difficult(i: usize, cache: &mut Cache) -> bool {
	// Find the next multiple of 7 or 11
	let next_mult = (i+1..).find(|x| x % 11 == 0 || x % 7 == 0).expect("How did this overflow??");
	is_prime(i, cache) && !(i+1..next_mult).any(|x| is_prime(x, cache))
}

// Check if `i` is prime, cache the result, and return it
fn is_prime(i: usize, (primes, last_sqrt): &mut Cache) -> bool {
	// increment the sqrt if it's too low
	*last_sqrt += ((*last_sqrt+1).pow(2) <= i) as usize;

	// Get the cached result, or compute it. the `filter` will skip non-prime factors
	primes[i] = primes[i].or_else(|| Some([(2..=*last_sqrt).filter(|x| !matches!(primes[*x], Some(false))).find(|j| i % j == 0), Some(0)][(*last_sqrt == i) as usize].is_none()));
	primes[i].unwrap()
}

#[cfg(test)]
mod test {
    use crate::{is_prime, MAX_NUM, Cache};

	#[test]
	fn test_is_prime() {
		let mut cache: Cache = ([None; MAX_NUM+7], 1);
		let mut confirmed = Vec::new();
		for i in 1..=100 {
			if is_prime(i, &mut cache) {
				confirmed.push(i);
			}
		}
		assert_eq!(cache.1, 10);

		assert_eq!(cache.0.iter().enumerate().filter_map(|(i, x)| if x == &Some(true) {Some(i)} else {None}).collect::<Vec<usize>>(), 
			vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
	}
}
