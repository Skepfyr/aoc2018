use std::fmt::{self, Write};

use bitvec::prelude::*;
use eyre::{eyre, Result};
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day12.txt"),
    part1,
    part2,
};

struct Pots {
    pots: BitVec,
    zero: isize,
}

impl Pots {
    fn score(&self) -> u64 {
        self.pots
            .iter_ones()
            .map(|i| i as isize - self.zero as isize)
            .sum::<isize>() as u64
    }
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.zero >= 0 {
            for _ in 0..self.zero {
                f.write_char(' ')?;
            }
            f.write_str("0\n")?;
        } else {
            writeln!(f, "0 << {}", -self.zero)?;
        }
        for plant in &self.pots {
            f.write_char(if *plant { '#' } else { '.' })?;
        }
        f.write_char('\n')
    }
}

struct Plants {
    rules: u32,
    current: Pots,
    previous: Pots,
}

impl Plants {
    fn step(&mut self) -> Result<bool> {
        std::mem::swap(&mut self.current, &mut self.previous);
        self.current.pots.clear();

        let first = match self.previous.pots.first_one() {
            Some(first) => first,
            None => return Err(eyre!("All plants died")),
        };
        let last = self.previous.pots.last_one().unwrap();

        self.current.pots.extend_from_bitslice(bits![0; 4]);
        self.current.pots.extend(
            self.previous.pots[first - 4..=last + 4]
                .windows(5)
                .map(|slice| self.rules & (1 << slice.load_le::<u8>()) != 0),
        );
        self.current.pots.extend_from_bitslice(bits![0; 4]);
        self.current.zero = self.previous.zero + 6 - first as isize;
        Ok(self.current.pots != self.previous.pots)
    }
}

fn plant_state(c: char) -> Result<bool> {
    match c {
        '#' => Ok(true),
        '.' => Ok(false),
        c => Err(eyre!("Invalid plant state: {c}")),
    }
}

fn parse(input: &str) -> Result<Plants> {
    let mut lines = input.lines();
    let mut initial_state = BitVec::new();
    initial_state.extend_from_bitslice(bits![0; 4]);
    lines
        .next()
        .ok_or_else(|| eyre!("Empty input"))?
        .strip_prefix("initial state: ")
        .ok_or_else(|| eyre!("Must start with 'initial state: '"))?
        .chars()
        .map(plant_state)
        .try_for_each(|bit| -> Result<_> {
            initial_state.push(bit?);
            Ok(())
        })?;
    initial_state.extend_from_bitslice(bits![0; 4]);

    let spacing_line = lines.next().ok_or_else(|| eyre!("Missing rules"))?;
    if !spacing_line.is_empty() {
        return Err(eyre!("Spacing line not empty: {spacing_line:?}"));
    }
    let mut mask: u32 = 0;
    let mut rules: u32 = 0;
    for line in lines {
        let (pattern, result) = line
            .split_once(" => ")
            .ok_or_else(|| eyre!("rule '{line}' did not contain an arrow"))?;
        if pattern.len() != 5 {
            return Err(eyre!("Rule pattern '{pattern}' must be 5 characters long"));
        }
        if result.len() != 1 {
            return Err(eyre!("Rule output '{result}' must be 1 character"));
        }
        let pattern_num = pattern
            .chars()
            .map(plant_state)
            .try_fold(0, |acc, state| -> Result<u32> {
                Ok((acc >> 1) | ((state? as u32) << 4))
            })?;
        let result = plant_state(result.chars().next().unwrap())?;
        let bit = 1 << pattern_num;
        if mask & bit != 0 {
            return Err(eyre!("Pattern '{pattern}' specified twice."));
        }
        mask |= bit;
        rules |= if result { bit } else { 0 };
    }
    if mask != !0 {
        return Err(eyre!(
            "32 rules must be provided, only got {}",
            mask.count_ones()
        ));
    }
    let previous_state = BitVec::with_capacity(initial_state.capacity());
    Ok(Plants {
        rules,
        current: Pots {
            pots: initial_state,
            zero: 4,
        },
        previous: Pots {
            pots: previous_state,
            zero: 4,
        },
    })
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let mut state = parse(input)?;
    for _ in 0..20 {
        state.step()?;
    }
    Ok(state.current.score().to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let mut state = parse(input)?;
    let mut generation = 0u64;
    let generation = loop {
        generation += 1;
        if !state.step()? {
            break generation;
        }
    };
    let old_score = state.previous.score();
    let new_score = state.current.score();

    Ok((new_score + (new_score - old_score) * (50_000_000_000 - generation)).to_string())
}
