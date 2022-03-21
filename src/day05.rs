use ahash::AHashSet;
use eyre::Result;
use tracing::{debug, instrument};

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day05.txt"),
    part1,
    part2,
};

fn react(input: impl IntoIterator<Item = char>) -> usize {
    let mut unreacted: Vec<char> = Vec::new();
    for c in input {
        if !c.is_alphabetic() {
            continue;
        }
        match unreacted.last() {
            Some(&last) if last.to_ascii_lowercase() == c.to_ascii_lowercase() && last != c => {
                unreacted.pop();
            }
            _ => unreacted.push(c),
        }
    }
    debug!(unreacted = %unreacted.iter().copied().collect::<String>());
    unreacted.len()
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    Ok(react(input.chars()).to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let symbols: AHashSet<char> = input.chars().map(|c| c.to_ascii_lowercase()).collect();
    let max = symbols
        .into_iter()
        .map(|s| react(input.chars().filter(|c| c.to_ascii_lowercase() != s)))
        .min()
        .unwrap();
    Ok(max.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_reacts() {
        assert_eq!("10", &part1("dabAcCaCBAcCcaDA").unwrap())
    }
}
