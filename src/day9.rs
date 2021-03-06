use crate::intcode::Program;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| {
            x.parse()
                .unwrap_or_else(|_| panic!("Unable to parse \"{}\" as integer", x))
        })
        .chain(std::iter::repeat(0).take(1024))
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
