use std::cmp::Ordering;

use ahash::AHashSet;
use eyre::{eyre, Result};
use itertools::Itertools;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day06.txt"),
    part1,
    part2,
};

fn parse(input: &str) -> Result<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| eyre!("No comma found on line: {:?}", line))?;
            Ok((x.trim().parse()?, y.trim().parse()?))
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    score: usize,
    owner: Option<usize>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            score: usize::MAX,
            owner: None,
        }
    }
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let points = parse(input)?;
    let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();
    let mut grid = vec![vec![Cell::default(); max_x + 1]; max_y + 1];
    for (point, (x, y)) in points.into_iter().enumerate() {
        for (j, row) in grid.iter_mut().enumerate() {
            for (i, cell) in row.iter_mut().enumerate() {
                let dist = x.abs_diff(i) + y.abs_diff(j);
                match dist.cmp(&cell.score) {
                    Ordering::Less => {
                        cell.score = dist;
                        cell.owner = Some(point);
                    }
                    Ordering::Equal => {
                        cell.owner = None;
                    }
                    Ordering::Greater => {}
                }
            }
        }
    }

    let mut infinite = AHashSet::new();
    infinite.extend(grid[0].iter().filter_map(|cell| cell.owner));
    infinite.extend(grid.iter().filter_map(|row| row[0].owner));
    infinite.extend(grid.iter().filter_map(|row| row[max_x].owner));
    infinite.extend(grid[max_y].iter().filter_map(|cell| cell.owner));

    let counts = grid
        .into_iter()
        .flatten()
        .filter_map(|cell| cell.owner)
        .filter(|point| !infinite.contains(point))
        .counts();
    Ok(counts.values().max().unwrap().to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let points = parse(input)?;
    let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();
    let mut safe_points = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            let total_distance: usize = points
                .iter()
                .copied()
                .map(|(i, j)| i.abs_diff(x) + j.abs_diff(y))
                .sum();
            if total_distance < 10000 {
                safe_points += 1;
            }
        }
    }
    Ok(safe_points.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_area() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        assert_eq!("17", &part1(input).unwrap());
    }
}
