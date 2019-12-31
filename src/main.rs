extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

mod intcode;
mod day1;
mod day2;
mod day4;
mod day5;
mod day6;
mod day7;
mod day9;

use aoc_runner_derive::aoc_main;

aoc_main! { year = 2019 }