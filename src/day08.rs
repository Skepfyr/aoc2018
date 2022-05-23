use eyre::{bail, eyre, Result};
use smallvec::SmallVec;
use tracing::{debug, instrument};

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day08.txt"),
    part1,
    part2,
};

fn parse<T, R: FnMut(&[T], &[u32]) -> T>(input: &str, mut reducer: R) -> Result<T> {
    let mut input = input
        .split_ascii_whitespace()
        .map((move |num| Ok(num.parse::<u32>()?)) as fn(&str) -> Result<u32>);
    reduce(&mut input, &mut reducer)
}

fn reduce<I: Iterator<Item = Result<u32>>, T, R: FnMut(&[T], &[u32]) -> T>(
    input: &mut I,
    reducer: &mut R,
) -> Result<T> {
    let children = input
        .next()
        .ok_or(eyre!("Missing number of child nodes"))??;
    let metadata_len = input
        .next()
        .ok_or(eyre!("Missing number of metadata entries"))?? as usize;
    let children = (0..children)
        .map(|_| reduce(input, reducer))
        .collect::<Result<SmallVec<[T; 16]>>>()?;
    let metadata = input
        .take(metadata_len)
        .collect::<Result<SmallVec<[u32; 16]>, _>>()?;
    if metadata.len() != metadata_len {
        bail!("Missing metadata");
    }
    Ok(reducer(children.as_slice(), &metadata))
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    parse(input, |children, metadata| {
        children.iter().sum::<u32>() + metadata.iter().sum::<u32>()
    })
    .map(|num| num.to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    parse(input, |children, metadata| {
        if children.is_empty() {
            let sum = metadata.iter().sum();
            debug!(sum, "Leaf node");
            sum
        } else {
            let sum = metadata
                .iter()
                .map(|&entry| children.get(entry as usize - 1).copied().unwrap_or(0))
                .sum();
            debug!(sum, "Composite node");
            sum
        }
    })
    .map(|num| num.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn sum() {
        let sum = part1(INPUT).unwrap();
        assert_eq!("138", &sum);
    }

    #[test]
    fn value() {
        let value = part2(INPUT).unwrap();
        assert_eq!("66", &value);
    }
}
