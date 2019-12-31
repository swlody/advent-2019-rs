fn fuel_sum(fuel: u64) -> u64 {
    let mut additional_fuel = fuel / 3;
    let mut acc = fuel;

    while additional_fuel > 2 {
        additional_fuel -= 2;
        acc += additional_fuel;
        additional_fuel /= 3;
    }

    acc
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .fold(0, |acc, x| acc + (x / 3) - 2)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .fold(0, |acc, x| acc + fuel_sum((x / 3) - 2))
}
