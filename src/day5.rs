use crate::intcode::Program;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let prog = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let outputs = Program::new(prog, vec![1]).collect::<Vec<_>>();

    *outputs.last().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let prog = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let outputs = Program::new(prog, vec![5]).collect::<Vec<_>>();

    *outputs.last().unwrap()
}
