
const CHECKS: [(fn(usize, (&mut [Option<bool>; MAX_NUM+11], &mut usize)) -> bool, &'static str); 6] = [
	(|i, _| i % 3 == 0, "Fizz"),
	(|i, _| i % 5 == 0, "Buzz"),
	(|i, _| i % 7 == 0, "Rizz"),
	(|i, _| i % 11 == 0, "Jazz"),
	(|i, _| (120 / i) * i == 120, "Dizz"),
	(difficult, "Prizz"),
];

const MAX_NUM: usize = 100;

fn main() {
	let (mut primes, mut last_sqrt) = ([None; MAX_NUM+11], 1);
	for i in 1..=MAX_NUM {
		let mut paren = false;
		for check in CHECKS {
			let res = check.0(i, (&mut primes, &mut last_sqrt));
			paren |= res;
			print!("{}", ["", check.1][res as usize]);
		}
		#[cfg(not(debug_assertions))]
		println!("{}", [&i.to_string(),""][paren as usize]);

		#[cfg(debug_assertions)]
		println!("{}", [&i.to_string(),&format!(" ({})", &i.to_string())][paren as usize]);
	}
}


fn difficult(i: usize, cache: (&mut [Option<bool>; MAX_NUM+11], &mut usize)) -> bool {
	let prepop_max = (i+1..).find(|x| x % 11 == 0 || x % 7 == 0).expect("How did this overflow??");

	!(i+1..prepop_max).any(|x| check_prime(x, cache.0, cache.1)) && check_prime(i, cache.0, cache.1)
}

fn check_prime(i: usize, primes: &mut [Option<bool>; MAX_NUM+11], last_sqrt: &mut usize) -> bool {
	// increment the sqrt if it's too low
	*last_sqrt += ((*last_sqrt+1).pow(2) <= i) as usize;

	// Get the cached result, or compute it
	let ret = primes[i].or(Some([(2..=*last_sqrt).find(|j| i % j == 0), Some(0)][(*last_sqrt == i) as usize].is_none()));

	// Reset the cache. Technically inefficient, but not a good way to do this without if statements
	primes[i] = ret;
	ret.unwrap()
}

#[cfg(test)]
mod test {
    use crate::check_prime;

	#[test]
	fn test_check_prime() {
		let (mut primes, mut last_sqrt) = ([None; 111], 1);
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