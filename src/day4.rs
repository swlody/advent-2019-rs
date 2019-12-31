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

fn is_valid_password(digits: [u8; 6]) -> bool {
    let mut last_seen = digits[0];
    let mut has_valid_double = false;

    for &digit in &digits[1..] {
        use std::cmp::Ordering;
        match digit.cmp(&last_seen) {
            Ordering::Greater => return false,
            Ordering::Equal => has_valid_double = true,
            Ordering::Less => (),
        }

        last_seen = digit;
    }

    has_valid_double
}

#[aoc(day4, part1)]
pub fn solve_part1(_: &str) -> usize {
    (168_630..718_098).map(to_digits).fold(0, |acc, digits| {
        acc + if is_valid_password(digits) { 1 } else { 0 }
    })
}
