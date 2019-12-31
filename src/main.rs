mod day1;
mod day2;
mod day4;
mod day5;
mod day6;
mod day7;
mod day9;
mod intcode;

use std::io::Result;

fn main() -> Result<()> {
    // TODO(sawlody) Make a macro or something to format solutions
    // TODO(sawlody) Some sort of progress bar - don't print table until all solutions are computed?
    println!("-----------------------------");
    println!("| {0: <3} | {1: <8} | {2: <8} |", "Day", "Part A", "Part B");
    println!("-----------------------------");
    println!(
        "| {0: <3} | {1: <8} | {2: <8} |",
        1,
        day1::solve_part_a()?,
        day1::solve_part_b()?
    );
    println!(
        "| {0: <3} | {1: <8} | {2: <8} |",
        2,
        day2::solve_part_a()?,
        day2::solve_part_b()?
    );
    println!("| {0: <3} | {1: <8} | {2: <8} |", 3, "  TODO", "  TODO");
    println!(
        "| {0: <3} | {1: <8} | {2: <8} |",
        4,
        day4::solve_part_a(),
        "  TODO"
    );
    println!(
        "| {0: <3} | {1: <8} | {2: <8} |",
        5,
        day5::solve_part_a(),
        day5::solve_part_b()
    );

    {
        let (part_a, part_b) = day6::solve();
        println!("| {0: <3} | {1: <8} | {2: <8} |", 6, part_a, part_b);
    }

    println!(
        "| {0: <3} | {1: <8} | {2: <8} |",
        7,
        "  TODO",
        day7::solve_part_b()
    );

    println!(
        "| {0: <3} | {1: <8} | {2: <8} |",
        9,
        "  TODO",
        day9::solve_part_b()
    );

    println!("-----------------------------");

    Ok(())
}
