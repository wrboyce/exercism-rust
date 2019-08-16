pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    let mut prime_factors = Vec::new();
    let mut candidates = 2..=n / 2 + 1;

    while x > 1 {
        let i = candidates.next().unwrap();
        while x % i == 0 {
            x /= i;
            prime_factors.push(i);
        }
    }

    prime_factors
}
