use crate::intcode;

pub fn solve_part_b() -> i64 {
    let prog = std::fs::read_to_string("inputs/day9.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .chain(std::iter::repeat(0).take(1024))
        .collect::<Vec<_>>();

    let outputs = intcode::Program::new(prog, vec![2]).collect::<Vec<_>>();

    *outputs.last().unwrap()
}
