use crate::intcode;
use itertools::Itertools;

fn fix_program(program: &mut Vec<i64>, noun: i64, verb: i64) {
    program[1] = noun;
    program[2] = verb;
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    let mut program = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let (noun, verb) = (12, 2);
    fix_program(&mut program, noun, verb);

    let (mut pc, mut relative_base) = (0, 0);
    while let Some(_) = intcode::run_program(&mut program, &mut pc, &mut relative_base, &[]) {}

    program[0]
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i64 {
    let program = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for inputs in (1..99).permutations(2) {
        let mut program_clone = program.clone();

        let (noun, verb) = (inputs[0], inputs[1]);
        fix_program(&mut program_clone, noun, verb);

        let (mut pc, mut relative_base) = (0, 0);
        while let Some(_) =
            intcode::run_program(&mut program_clone, &mut pc, &mut relative_base, &[])
        {}

        if program_clone[0] == 19_690_720 {
            return 100 * noun + verb;
        }
    }

    unreachable!()
}
