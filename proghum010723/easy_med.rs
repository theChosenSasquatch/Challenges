///
/// THIS IS JUST FOR REFERENCE
/// 
/// What could have been possible, if I didn't go for hard
///

const CHECKS: [(fn(usize, (&mut Vec<usize>, &mut usize)) -> bool, &'static str); 5] = [
	(|i, _| i % 3 == 0, "Fizz"),
	(|i, _| i % 5 == 0, "Buzz"),
	(|i, _| i % 7 == 0, "Rizz"),
	(|i, _| i % 11 == 0, "Jazz"),
	(|i, _| (120 / i) * i == 120, "Dizz")
];

fn main() {
	let (mut primes, mut last_sqrt) = (Vec::with_capacity(25), 1);
	for i in 1..=MAX_NUM {
		let mut paren = false;
		for check in CHECKS {
			let res = check.0(i, (&mut primes, &mut last_sqrt));
			paren |= res;
			print!("{}", ["", check.1][res as usize]);
		}
		
		println!("{}", [&i.to_string(),""][paren as usize]);
	}
}
