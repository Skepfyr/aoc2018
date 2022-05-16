use eyre::Result;
use tracing::{debug, instrument};

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day05.txt"),
    part1,
    part2,
};

#[instrument(skip(input))]
fn react(input: impl IntoIterator<Item = char>) -> Vec<char> {
    let input = input.into_iter();
    let mut unreacted: Vec<char> = Vec::with_capacity(input.size_hint().0);
    for c in input {
        if !c.is_alphabetic() {
            continue;
        }
        match unreacted.last() {
            Some(&last) if last.eq_ignore_ascii_case(&c) && last != c => {
                unreacted.pop();
            }
            _ => unreacted.push(c),
        }
    }
    debug!(unreacted = %unreacted.iter().copied().collect::<String>());
    unreacted
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    Ok(react(input.chars()).len().to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let reacted = react(input.chars());
    let max = ('a'..='z')
        .into_iter()
        .map(|s| {
            react(
                reacted
                    .iter()
                    .copied()
                    .filter(|c| c.to_ascii_lowercase() != s),
            )
            .len()
        })
        .min()
        .unwrap();
    Ok(max.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_reacts() {
        let reacted = react("dabAcCaCBAcCcaDA".chars());
        assert_eq!("dabCBAcaDA", reacted.into_iter().collect::<String>())
    }
}
