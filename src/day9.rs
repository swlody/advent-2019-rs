use crate::intcode;

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let prog = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .chain(std::iter::repeat(0).take(1024))
        .collect::<Vec<_>>();

    let outputs = intcode::Program::new(prog, vec![2]).collect::<Vec<_>>();

    *outputs.last().unwrap()
}
