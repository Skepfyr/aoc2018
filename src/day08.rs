use eyre::{eyre, Result};
use smallvec::SmallVec;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day08.txt"),
    part1,
    part2,
};

fn parse<R: Reduce>(input: &str, reducer: R) -> Result<R::Output> {
    let mut input = input
        .split_ascii_whitespace()
        .map(move |num| -> Result<u32> { Ok(num.parse::<u32>()?) });
    reduce(&mut input, reducer)
}

fn reduce<I: Iterator<Item = Result<u32>>, R: Reduce>(
    input: &mut I,
    mut reducer: R,
) -> Result<R::Output> {
    let children = input
        .next()
        .ok_or(eyre!("Missing number of child nodes"))??;
    let metadata_len = input
        .next()
        .ok_or(eyre!("Missing number of metadata entries"))?? as usize;
    for _ in 0..children {
        let mut called = false;
        reducer.child(|child_reducer| {
            called = true;
            reduce(input, child_reducer)
        })?;
        if !called {
            reduce(input, NullReduce)?;
        }
    }
    let mut metadata_count = 0;
    let res = reducer.metadata(
        input
            .by_ref()
            .inspect(|_| metadata_count += 1)
            .take(metadata_len),
    )?;
    for _ in metadata_count..metadata_len {
        input.next().ok_or_else(|| eyre!("Missing metadata"))??;
    }
    Ok(res)
}

trait Reduce
where
    Self: Sized,
{
    type Output;

    fn child(&mut self, child: impl FnOnce(Self) -> Result<Self::Output>) -> Result<()>;
    fn metadata(self, metadata: impl Iterator<Item = Result<u32>>) -> Result<Self::Output>;
}

struct NullReduce;
impl Reduce for NullReduce {
    type Output = ();

    fn child(&mut self, child: impl FnOnce(Self) -> Result<Self::Output>) -> Result<()> {
        child(NullReduce)
    }

    fn metadata(self, _metadata: impl Iterator<Item = Result<u32>>) -> Result<Self::Output> {
        Ok(())
    }
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    #[derive(Default)]
    struct SumReduce(u32);
    impl Reduce for SumReduce {
        type Output = u32;

        fn child(&mut self, child: impl FnOnce(Self) -> Result<Self::Output>) -> Result<()> {
            self.0 += child(Self::default())?;
            Ok(())
        }

        fn metadata(self, metadata: impl Iterator<Item = Result<u32>>) -> Result<Self::Output> {
            Ok(self.0 + metadata.sum::<Result<u32>>()?)
        }
    }
    parse(input, SumReduce::default()).map(|num| num.to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    #[derive(Default)]
    struct ValueReduce(SmallVec<[u32; 16]>);
    impl Reduce for ValueReduce {
        type Output = u32;

        fn child(&mut self, child: impl FnOnce(Self) -> Result<Self::Output>) -> Result<()> {
            self.0.push(child(Self::default())?);
            Ok(())
        }

        fn metadata(self, metadata: impl Iterator<Item = Result<u32>>) -> Result<Self::Output> {
            if self.0.is_empty() {
                metadata.sum()
            } else {
                metadata
                    .map(|datum| Ok(self.0.get(datum? as usize - 1).copied().unwrap_or(0)))
                    .sum()
            }
        }
    }
    parse(input, ValueReduce::default()).map(|num| num.to_string())
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
