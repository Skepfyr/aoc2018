use ahash::AHashMap;
use eyre::{bail, Result};
use recap::Recap;
use serde::Deserialize;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day03.txt"),
    part1,
    part2,
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy, Deserialize, Recap)]
#[recap(regex = r#"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)"#)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn points(self) -> impl Iterator<Item = (u32, u32)> {
        (self.x..self.x + self.width)
            .flat_map(move |x| (self.y..self.y + self.height).map(move |y| (x, y)))
    }
}

fn parse(input: &str) -> Result<Vec<Claim>> {
    input.lines().map(|line| Ok(line.try_into()?)).collect()
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let claims = parse(input)?;
    let mut claimed_points = AHashMap::<_, u32>::new();
    for claim in claims {
        for point in claim.points() {
            *claimed_points.entry(point).or_default() += 1;
        }
    }
    Ok(claimed_points
        .values()
        .filter(|&&v| v >= 2)
        .count()
        .to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let claims = parse(input)?;
    let mut claimed_points = AHashMap::<_, u32>::new();
    for claim in &claims {
        for point in claim.points() {
            *claimed_points.entry(point).or_default() += 1;
        }
    }
    for claim in claims {
        if claim.points().all(|point| claimed_points[&point] == 1) {
            return Ok(claim.id.to_string());
        }
    }
    bail!("No valid claim found")
}
