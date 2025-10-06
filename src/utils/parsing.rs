pub fn is_digits(input: &str) -> bool {
    input.chars().all(|c| c.is_digit(10))
}
