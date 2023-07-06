use std::sync::OnceLock;
use regex::Regex;

const MAX_NUM: usize = 100;

const CHECKS: [(&str, fn(usize) -> bool); 6] = [
	("Fizz", |i| i % 3 == 0),
	("Buzz", |i| i % 5 == 0),
	("Rizz", |i| i % 7 == 0),
	("Jazz", |i| i % 11 == 0),
	("Dizz", |i| (120 / i) * i == 120),
	("Prizz", difficult),
];

fn difficult(i: usize) -> bool {
	static PRIMES: OnceLock<Vec<usize>> = OnceLock::new();
	let primes = PRIMES.get_or_init(|| download_primes());
	primes.binary_search(&i).map(|x| (primes[x]+1..).find(|x| x % 7 == 0 || x % 11 == 0).unwrap() <= primes[x+1]) == Ok(true)
}

fn download_primes() -> Vec<usize> {
	let prime_site = reqwest::blocking::get("https://www.mathsisfun.com/numbers/prime-numbers-to-10k.html").expect("Failed to download prime numbers").text().unwrap();
	Regex::new(r#"<span class="boxa">(\d+)</span>"#).unwrap().captures_iter(&prime_site).into_iter().map(|x| x.get(1).unwrap().as_str().parse::<usize>().unwrap()).collect()
}

fn main() {
	for i in 1..=MAX_NUM {
		println!("{}", [&i.to_string(),""][CHECKS.iter().filter(|(_, check)| check(i)).fold(0, |_, (y, _)| {print!("{y}"); 1})])
	}
}