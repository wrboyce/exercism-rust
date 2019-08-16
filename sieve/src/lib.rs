pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for n in 2..=((upper_bound + 1) as f64).sqrt() as u64 {
        if is_prime[n as usize] {
            for m in ((n * n)..=upper_bound).step_by(n as usize) {
                is_prime[m as usize] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(n, &p)| if p { Some(n as u64) } else { None })
        .collect()
}
