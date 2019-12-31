use crate::intcode;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .chain(std::iter::repeat(0).take(1024))
        .collect::<Vec<_>>()
}

#[aoc(day9, part2)]
pub fn solve_part2(program: &[i64]) -> i64 {
    let outputs = intcode::Program::new(program.to_vec(), vec![2]).collect::<Vec<_>>();

    *outputs.last().unwrap()
}
