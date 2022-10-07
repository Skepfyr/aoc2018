use eyre::Result;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/dayxx.txt"),
    part1,
    part2,
};

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    Ok("unsolved".to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    Ok("unsolved".to_string())
}
