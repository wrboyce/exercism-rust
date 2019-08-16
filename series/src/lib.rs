pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.chars().count() {
        return Vec::new();
    }
    (0..=digits.chars().count() - len)
        .map(|n| String::from(&digits[n..n + len]))
        .collect()
}
