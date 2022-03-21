use eyre::Result;

mod day01;
mod day02;
mod day03;

pub struct Solution {
    pub input: &'static str,
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
}

pub const SOLUTIONS: [Solution; 3] = [day01::SOLUTION, day02::SOLUTION, day03::SOLUTION];
