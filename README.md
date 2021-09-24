# primes

This package provides an iterator over `all` primes, generating them lazily as they are needed.

The simplest usage is to create an `Iterator`:

```Rust
use primes::TrialDivision;

let mut pset = TrialDivision::new();

for (idx, n) in pset.iter().enumerate().take(10) {
	println!("Prime #{}: {}", idx, n);
}
```

For more examples, see [the full documentation](https://github.com/eligum/primes/wiki)!
