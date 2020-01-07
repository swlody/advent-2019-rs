fn fuel_sum(mut fuel: u64) -> u64 {
    let mut additional_fuel = fuel / 3;

    while additional_fuel > 2 {
        additional_fuel -= 2;
        fuel += additional_fuel;
        additional_fuel /= 3;
    }

    fuel
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u64> {
    input
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(numbers: &[u64]) -> u64 {
    numbers.iter().fold(0, |acc, x| acc + (x / 3) - 2)
}

#[aoc(day1, part2)]
pub fn solve_part2(numbers: &[u64]) -> u64 {
    numbers.iter().fold(0, |acc, x| acc + fuel_sum((x / 3) - 2))
}
