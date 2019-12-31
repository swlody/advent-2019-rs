use std::convert::TryInto;

fn to_digits(x: u32) -> [u8; 6] {
    let mut digits = [0; 6];
    let mut num = x;

    for digit in &mut digits {
        *digit = (num % 10).try_into().unwrap_or_else(|_| unreachable!());
        num /= 10;
    }

    digits
}

fn is_valid_password(digits: &[u8; 6]) -> bool {
    let mut last_seen = digits[0];
    let mut has_valid_double = false;

    for &digit in &digits[1..] {
        if digit > last_seen {
            return false;
        } else if digit == last_seen {
            has_valid_double = true;
        }

        last_seen = digit;
    }

    has_valid_double
}

pub fn solve_part_a() -> usize {
    (168_630..718_098)
        .map(to_digits)
        .filter(is_valid_password)
        .count()
}

#[allow(dead_code)]
pub fn solve_part_b() -> usize {
    0
}
