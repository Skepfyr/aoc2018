use eyre::Result;

mod day1;

pub struct Solution {
    pub input: &'static str,
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
}

pub const SOLUTIONS: [Solution; 1] = [day1::SOLUTION];
