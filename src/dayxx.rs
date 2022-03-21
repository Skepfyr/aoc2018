use eyre::Result;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/dayxx.txt"),
    part1,
    part2,
};

fn parse() -> Result<String> {}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    todo!()
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    todo!()
}
