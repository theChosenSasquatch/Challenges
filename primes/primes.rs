const MAX_N: usize = 40_000_000;

type Primes = Box<[u32; MAX_N]>;
// type Primes = [u32; MAX_N];

fn main() {
	let (mut primes, mut ind): (Primes, _) = (Box::new([0; MAX_N]), 1);
	// primes[0] = 2; // 2 is the first prime number, but the loop below starts only iterates over odd numbers
	let mut last_sqrt: u32 = 1;

	for i in (3..).step_by(2) {
		last_sqrt += ((last_sqrt+1).pow(2) <= i) as u32;
		if !primes.iter().skip(1).take_while(|x| **x > 0 && **x <= last_sqrt).any(|x| i % x == 0) {
			primes[ind] = i;
			ind += 1;
			if ind == MAX_N {
				break;
			}
			#[cfg(debug_assertions)]
			println!("prime: {i}")
		}
	}
	println!("Prime number #{} is {}", MAX_N, primes[MAX_N-1]);
}