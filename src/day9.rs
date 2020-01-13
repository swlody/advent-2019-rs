use crate::intcode::Program;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse())
        .chain(std::iter::repeat(Ok(0)).take(1024))
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(program: &[i64]) -> Option<i64> {
    Program::new(program.to_vec(), vec![1]).last()
}

#[aoc(day9, part2)]
pub fn solve_part2(program: &[i64]) -> Option<i64> {
    Program::new(program.to_vec(), vec![2]).last()
}
