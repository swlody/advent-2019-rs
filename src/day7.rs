use crate::intcode::*;
use itertools::Itertools;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| {
            x.parse()
                .unwrap_or_else(|_| panic!("Unable to parse \"{}\" as integer", x))
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(program: &[i64]) -> Option<i64> {
    (0..=4)
        .permutations(5)
        .map(move |phase_settings| {
            let mut last_output = Some(0);

            // TODO(sawlody) each copy of the program can run in parallel until it hits an input
            // without having a corresponding input. Can set up channels between the programs
            // which block until the input is provided by its previous program.
            for phase_setting in phase_settings {
                last_output =
                    Program::new(program.to_vec(), vec![phase_setting, last_output?]).last();
            }

            last_output
        })
        .flatten()
        .max()
}

#[aoc(day7, part2)]
pub fn solve_part2(program: &[i64]) -> Option<i64> {
    (5..=9)
        .permutations(5)
        .map(move |phase_settings| {
            let mut last_output = 0;

            let mut amplifiers = std::iter::repeat(program.to_vec())
                .take(5)
                .collect::<Vec<_>>();
            let mut pcs = [0; 5];

            let mut i = 0;
            let mut halted = 0;
            while halted != 5 {
                let inputs = if i < 5 {
                    vec![phase_settings[i], 0]
                } else {
                    vec![last_output]
                };

                {
                    let i = i % 5;
                    if let Some(output) =
                        run_program(&mut amplifiers[i], &mut pcs[i], &mut 0, &inputs)
                    {
                        last_output = output;
                    } else {
                        halted += 1;
                    }
                }

                i += 1;
            }

            last_output
        })
        .max()
}
