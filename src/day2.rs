use crate::intcode;
use itertools::Itertools;

fn fix_program(program: &mut [i64], noun: i64, verb: i64) {
    program[1] = noun;
    program[2] = verb;
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(program: &[i64]) -> i64 {
    let mut program = program.to_vec();

    let (noun, verb) = (12, 2);
    fix_program(&mut program, noun, verb);

    let (mut pc, mut relative_base) = (0, 0);
    while let Some(_) = intcode::run_program(&mut program, &mut pc, &mut relative_base, &[]) {}

    program[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(program: &[i64]) -> i64 {
    for inputs in (1..99).permutations(2) {
        let mut program = program.to_vec();

        let (noun, verb) = (inputs[0], inputs[1]);
        fix_program(&mut program, noun, verb);

        let (mut pc, mut relative_base) = (0, 0);
        while let Some(_) = intcode::run_program(&mut program, &mut pc, &mut relative_base, &[]) {}

        if program[0] == 19_690_720 {
            return 100 * noun + verb;
        }
    }

    unreachable!()
}
