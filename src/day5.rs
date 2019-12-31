use crate::intcode::Program;

pub fn solve_part_a() -> i64 {
    let prog = std::fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let outputs = Program::new(prog, vec![1]).collect::<Vec<_>>();

    *outputs.last().unwrap()
}

pub fn solve_part_b() -> i64 {
    let prog = std::fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let outputs = Program::new(prog, vec![5]).collect::<Vec<_>>();

    *outputs.last().unwrap()
}
