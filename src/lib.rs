use eyre::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub struct Solution {
    pub input: &'static str,
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
}

pub const SOLUTIONS: [Solution; 7] = [
    day01::SOLUTION,
    day02::SOLUTION,
    day03::SOLUTION,
    day04::SOLUTION,
    day05::SOLUTION,
    day06::SOLUTION,
    day07::SOLUTION,
];
