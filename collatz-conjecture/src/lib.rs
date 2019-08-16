pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
    let mut x = n;

    while x > 1 {
        if x % 2 == 0 {
            x /= 2
        } else {
            x = 3 * x + 1
        }
        steps += 1
    }

    match (n, steps) {
        (0, _) => None,
        _ => Some(steps),
    }
}
