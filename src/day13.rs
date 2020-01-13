use crate::intcode::Program;

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse())
        .chain(std::iter::repeat(Ok(0)).take(1024))
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(program: &[i64]) -> usize {
    Program::new(program.to_vec(), Vec::new())
        .skip(2)
        .step_by(3)
        .filter(|&id| id == 2)
        .count()
}
