use ahash::AHashMap;
use eyre::{bail, Result};
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day02.txt"),
    part1,
    part2,
};

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let (twos, threes) = input.lines().fold((0, 0), |(twos, threes), id| {
        let mut two = false;
        let mut three = false;
        let mut counts: AHashMap<char, u32> = AHashMap::default();
        id.chars().for_each(|c| *counts.entry(c).or_default() += 1);
        for (_, i) in counts {
            match i {
                2 => two = true,
                3 => three = true,
                _ => {}
            }
        }
        (
            twos + if two { 1 } else { 0 },
            threes + if three { 1 } else { 0 },
        )
    });
    Ok((twos * threes).to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let ids: Vec<_> = input.lines().collect();
    for (i, &id_a) in ids.iter().enumerate() {
        for &id_b in &ids[i + 1..] {
            let mut differences = Iterator::zip(id_a.chars(), id_b.chars())
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .map(|(i, _)| i);
            let difference = match differences.next() {
                Some(difference) => difference,
                None => continue,
            };
            if differences.next().is_some() {
                continue;
            }
            let mut ret = id_a.to_string();
            ret.remove(difference);
            return Ok(ret);
        }
    }
    bail!("No nearly matching ids found");
}
