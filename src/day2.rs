use crate::intcode;
use itertools::Itertools;

fn fix_program(program: &mut Vec<i64>, noun: i64, verb: i64) {
    program[1] = noun;
    program[2] = verb;
}

pub fn solve_part_a() -> std::io::Result<i64> {
    let mut program = std::fs::read_to_string("inputs/day2.txt")?
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let (noun, verb) = (12, 2);
    fix_program(&mut program, noun, verb);

    let (mut pc, mut relative_base) = (0, 0);
    while let Some(_) = intcode::run_program(&mut program, &mut pc, &mut relative_base, &[]) {}

    Ok(program[0])
}

pub fn solve_part_b() -> std::io::Result<i64> {
    let program = std::fs::read_to_string("inputs/day2.txt")?
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
            return Ok(100 * noun + verb);
        }
    }

    unreachable!()
}
