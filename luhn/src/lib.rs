/// Check a digits checksum.
pub fn is_valid(code: &str) -> bool {
    // drop spaces and map everything else to its digit
    let digits = code
        .chars()
        .filter(|&c| c != ' ')
        .map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    // any None options here means we have invalid characters
    if digits.iter().any(|&n| n == None) {
        return false;
    }

    // single digit code cannot be valid
    digits.len() > 1
        && (digits
            .iter()
            .filter_map(|&n| n)
            .rev()
            .enumerate()
            .map(|(i, n)| if i % 2 == 1 && n < 9 { (n * 2) % 9 } else { n })
            .sum::<u32>())
            % 10
            == 0
}
