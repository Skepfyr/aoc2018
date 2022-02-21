use ahash::AHashSet;
use eyre::{bail, Result};
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day01.txt"),
    part1,
    part2,
};

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    Ok(parse(input).into_iter().sum::<i32>().to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let changes = parse(input);
    if changes.is_empty() {
        bail!("Must have some frequency changes");
    }
    let mut frequencies = changes.iter().copied().cycle().scan(0, |acc, d| {
        *acc += d;
        Some(*acc)
    });
    let mut seen = AHashSet::default();
    loop {
        let freq = frequencies.next().unwrap();
        if !seen.insert(freq) {
            return Ok(freq.to_string());
        }
    }
}
