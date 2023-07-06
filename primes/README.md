# Primes

This isn't a formal challenge, but I was working on the first Challenge, which is partially dependent on prime-finding,
so I wanted to explore optimizations for finding all primes < N.

The algorithm that I've come up checks each number `n` for prime-ness by checking if any prime `2 <= p <= sqrt(n)` divides into `n` evenly.
