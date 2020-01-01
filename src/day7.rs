use crate::intcode;
use itertools::Itertools;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day7, part2)]
pub fn solve_part2(program: &[i64]) -> i64 {
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
            loop {
                if halted == 5 {
                    break;
                }

                let inputs = if i < 5 {
                    vec![phase_settings[i], 0]
                } else {
                    vec![last_output]
                };

                {
                    let i = i % 5;
                    if let Some(output) =
                        intcode::run_program(&mut amplifiers[i], &mut pcs[i], &mut 0, &inputs)
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
        .unwrap()
}
