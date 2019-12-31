use std::fs::File;
use std::io::{BufRead, BufReader, Result};

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

pub fn solve_part_a() -> Result<u64> {
    let file = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .fold(0, |acc, x| acc + (x / 3) - 2);

    Ok(result)
}

pub fn solve_part_b() -> Result<u64> {
    let file = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .fold(0, |acc, x| acc + fuel_sum((x / 3) - 2));

    Ok(result)
}
