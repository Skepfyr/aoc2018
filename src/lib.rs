use eyre::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

pub struct Solution {
    pub input: &'static str,
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
}

pub const SOLUTIONS: [Solution; 16] = [
    day01::SOLUTION,
    day02::SOLUTION,
    day03::SOLUTION,
    day04::SOLUTION,
    day05::SOLUTION,
    day06::SOLUTION,
    day07::SOLUTION,
    day08::SOLUTION,
    day09::SOLUTION,
    day10::SOLUTION,
    day11::SOLUTION,
    day12::SOLUTION,
    day13::SOLUTION,
    day14::SOLUTION,
    day15::SOLUTION,
    day16::SOLUTION,
];
