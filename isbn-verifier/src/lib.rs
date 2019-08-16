/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits = isbn
        .chars()
        .enumerate()
        .filter_map(|(n, c)| {
            if c == 'X' && n == isbn.len() - 1 {
                Some(10)
            } else {
                c.to_digit(10)
            }
        })
        .collect::<Vec<_>>();

    if digits.len() != 10 {
        return false;
    }

    (digits
        .iter()
        .zip((1..=10).rev())
        .map(|(x, y)| x * y)
        .sum::<u32>())
        % 11
        == 0
}
