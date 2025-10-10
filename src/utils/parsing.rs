pub fn is_digits(input: &str) -> bool {
    input.chars().all(|c| c.is_digit(10))
}

pub fn parse_num(input: &str) -> Option<u32> {
    match input {
        "a" => Some(1),
        "an" => Some(1),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "thirteen" => Some(13),
        other => other.parse::<u32>().ok(),
    }
}
