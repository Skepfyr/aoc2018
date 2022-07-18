use std::{collections::HashSet, str::FromStr};

use eyre::{bail, eyre, Result};
use nalgebra::Vector2;
use tracing::{debug, instrument};

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day10.txt"),
    part1,
    part2,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Point {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
}

impl Point {
    fn pos_at_time(&self, t: i32) -> Vector2<i32> {
        self.position + t * self.velocity
    }
}

impl FromStr for Point {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("position=<").ok_or_else(|| eyre!(""))?;
        let (x, s) = s.split_once(',').ok_or_else(|| eyre!(""))?;
        let (y, s) = s.split_once('>').ok_or_else(|| eyre!(""))?;
        let s = s.strip_prefix(" velocity=<").ok_or_else(|| eyre!(""))?;
        let (dx, s) = s.split_once(',').ok_or_else(|| eyre!(""))?;
        let (dy, s) = s.split_once('>').ok_or_else(|| eyre!(""))?;
        if !s.is_empty() {
            bail!("")
        }
        debug!(%s, %x, %y, %dx, %dy, "Parsed line");
        Ok(Point {
            position: Vector2::new(x.trim().parse()?, y.trim().parse()?),
            velocity: Vector2::new(dx.trim().parse()?, dy.trim().parse()?),
        })
    }
}

#[instrument(skip(input))]
fn parse(input: &str) -> Result<Vec<Point>> {
    input.lines().map(|line| line.trim().parse()).collect()
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let points = parse(input)?;
    let time = estimate_message_time(&points);
    let points: HashSet<_> = points.iter().map(|p| p.pos_at_time(time)).collect();
    let min_x = points.iter().map(|p| p[0]).min().unwrap();
    let max_x = points.iter().map(|p| p[0]).max().unwrap();
    let min_y = points.iter().map(|p| p[1]).min().unwrap();
    let max_y = points.iter().map(|p| p[1]).max().unwrap();
    let mut answer = String::with_capacity((1 + (max_x - min_x + 1) * (max_y - min_y)) as usize);
    answer.push('\n');
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            answer.push(if points.contains(&Vector2::new(x, y)) {
                '#'
            } else {
                ' '
            })
        }
        answer.push('\n');
    }
    Ok(answer)
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    Ok(estimate_message_time(&parse(input)?).to_string())
}

#[instrument]
fn estimate_message_time(points: &[Point]) -> i32 {
    let (sum_pos, sum_vel) = points.iter().copied().fold(
        (Vector2::<f64>::default(), Vector2::<f64>::default()),
        |(pos, vel), point| (pos + point.position.cast(), vel + point.velocity.cast()),
    );
    let num_points = points.len() as f64;
    let (mean_pos, mean_vel) = (sum_pos / num_points, sum_vel / num_points);
    let (pos_vel, mag_vel) = points
        .iter()
        .copied()
        .map(|Point { position, velocity }| {
            (position.cast() - mean_pos, velocity.cast() - mean_vel)
        })
        .fold((0f64, 0f64), |(pos_vel, mag_vel), (pos, vel)| {
            (pos_vel + pos.dot(&vel), mag_vel + vel.norm_squared())
        });
    let closest_approach = -pos_vel / mag_vel;
    let time = closest_approach.round() as i32;
    debug!(%closest_approach, %time, "Estimated message time");
    time
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
        position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn message() {
        let points = parse(TEST_INPUT).unwrap();
        assert_eq!(3, estimate_message_time(&points));
        assert_eq!(
            "
      #   #  ###
      #   #   #
      #   #   #
      #####   #
      #   #   #
      #   #   #
      #   #   #
      #   #  ###
",
            &part1(TEST_INPUT).unwrap()
        );
    }
}
