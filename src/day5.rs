use crate::intcode::Program;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day5, part1)]
pub fn solve_part1(program: &[i64]) -> i64 {
    *Program::new(program.to_vec(), vec![1])
        .collect::<Vec<_>>()
        .last()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(program: &[i64]) -> i64 {
    *Program::new(program.to_vec(), vec![5])
        .collect::<Vec<_>>()
        .last()
        .unwrap()
}
