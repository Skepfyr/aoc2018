use eyre::Result;

mod day01;
mod day02;
pub struct Solution {
    pub input: &'static str,
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
}

pub const SOLUTIONS: [Solution; 2] = [day01::SOLUTION, day02::SOLUTION];
