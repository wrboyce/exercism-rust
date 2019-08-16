pub struct Luhn(pub String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        luhn::is_valid(&self.0)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
