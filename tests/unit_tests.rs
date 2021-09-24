use primes::*;

#[test]
fn primesetbasics_expand() {
	let mut pset = TrialDivision::new();
	let ln = pset.list().len();
	pset.expand();

	assert_eq!(pset.list().len(), ln + 1);
}

#[test]
fn primeset_next_prime_from_number() {
	let mut pset = TrialDivision::new();
	let (_idx, p) = pset.find(10);

	assert_eq!(p, 11);
}

#[test]
fn primeset_iterator() {
	let mut pset = TrialDivision::new();
	let first_few = [2u64, 3, 5, 7, 11, 13, 17, 19, 23];
	for (m, &n) in pset.iter().zip(first_few.iter()) {
		assert_eq!(m, n)
	}
}

#[test]
fn primeset_find_primes() {
	let mut pset = TrialDivision::new();

	// pset is empty, so it needs to generate the primes
	assert_eq!(pset.find_vec(1000), None);

	let (idx, n) = (168, 1009);

	assert_eq!(pset.find(1000), (168, 1009));
	assert_eq!(pset.find(n), (idx, n));

	// We shouldn't have gone beyond 1009
	{
		let plst = pset.list();
		let plen = pset.len();

		assert_eq!(plen, idx + 1);
		assert_eq!(plst[plen - 1], n);
	}

	assert_eq!(pset.find_vec(1000), Some((idx, n)));
}
