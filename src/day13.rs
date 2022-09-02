use core::fmt;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::Add,
    str::FromStr,
};

use eyre::{bail, eyre, Result};
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day13.txt"),
    part1,
    part2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    row: usize,
    column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.column, self.row)
    }
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Location {
                row: self.row - 1,
                column: self.column,
            },
            Direction::South => Location {
                row: self.row + 1,
                column: self.column,
            },
            Direction::East => Location {
                row: self.row,
                column: self.column + 1,
            },
            Direction::West => Location {
                row: self.row,
                column: self.column - 1,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Track {
    Straight,
    /// Turns like ╯╭
    TurnA,
    /// Turns like ╮╰
    TurnB,
    TurnAny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NextTurn {
    Left,
    Forward,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CartState {
    next_turn: NextTurn,
    direction: Direction,
}

impl CartState {
    fn new(direction: Direction) -> Self {
        Self {
            next_turn: NextTurn::Left,
            direction,
        }
    }

    fn step(self, track: Track) -> Self {
        use Direction::*;
        let mut next_turn = self.next_turn;
        let direction = match (track, self.direction) {
            (Track::Straight, dir) => dir,
            (Track::TurnA, North) => East,
            (Track::TurnA, South) => West,
            (Track::TurnA, East) => North,
            (Track::TurnA, West) => South,
            (Track::TurnB, North) => West,
            (Track::TurnB, South) => East,
            (Track::TurnB, East) => South,
            (Track::TurnB, West) => North,
            (Track::TurnAny, dir) => {
                let direction = match (next_turn, dir) {
                    (NextTurn::Left, North) => West,
                    (NextTurn::Left, West) => South,
                    (NextTurn::Left, South) => East,
                    (NextTurn::Left, East) => North,
                    (NextTurn::Forward, dir) => dir,
                    (NextTurn::Right, North) => East,
                    (NextTurn::Right, East) => South,
                    (NextTurn::Right, South) => West,
                    (NextTurn::Right, West) => North,
                };
                next_turn = match next_turn {
                    NextTurn::Left => NextTurn::Forward,
                    NextTurn::Forward => NextTurn::Right,
                    NextTurn::Right => NextTurn::Left,
                };
                direction
            }
        };
        Self {
            next_turn,
            direction,
        }
    }
}

#[derive(Debug)]
struct State {
    map: Vec<Vec<Option<Track>>>,
    carts: BTreeMap<Location, CartState>,
}

impl FromStr for State {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut carts = BTreeMap::new();
        let map = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(column, &symbol)| {
                        Ok(Some(match symbol {
                            b' ' => return Ok(None),
                            b'|' | b'-' => Track::Straight,
                            b'/' => Track::TurnA,
                            b'\\' => Track::TurnB,
                            b'+' => Track::TurnAny,
                            b'^' => {
                                carts.insert(
                                    Location { row, column },
                                    CartState::new(Direction::North),
                                );
                                Track::Straight
                            }
                            b'v' => {
                                carts.insert(
                                    Location { row, column },
                                    CartState::new(Direction::South),
                                );
                                Track::Straight
                            }
                            b'>' => {
                                carts.insert(
                                    Location { row, column },
                                    CartState::new(Direction::East),
                                );
                                Track::Straight
                            }
                            b'<' => {
                                carts.insert(
                                    Location { row, column },
                                    CartState::new(Direction::West),
                                );
                                Track::Straight
                            }
                            _ => bail!("Unexpected symbol in map: {:?}", symbol),
                        }))
                    })
                    .collect()
            })
            .collect::<Result<_>>()?;
        Ok(Self { map, carts })
    }
}

impl State {
    fn step(&mut self) -> Result<Vec<Location>> {
        let mut crashes = Vec::new();
        for old_loc in self.carts.keys().copied().collect::<Vec<_>>() {
            let cart = match self.carts.remove(&old_loc) {
                Some(cart) => cart,
                None => continue,
            };
            let track = self.map[old_loc.row][old_loc.column]
                .ok_or_else(|| eyre!("Fell off track at {}", old_loc))?;
            let new_cart = cart.step(track);
            let new_loc = old_loc + new_cart.direction;
            match self.carts.entry(new_loc) {
                Entry::Vacant(entry) => {
                    entry.insert(new_cart);
                }
                Entry::Occupied(entry) => {
                    entry.remove();
                    crashes.push(new_loc);
                }
            }
        }
        Ok(crashes)
    }
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let mut state: State = input.parse()?;
    let collision = loop {
        let crashes = state.step()?;
        if let Some(&collision) = crashes.first() {
            break collision;
        }
    };
    Ok(collision.to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let mut state: State = input.parse()?;
    while state.carts.len() > 1 {
        state.step()?;
    }
    let final_loc = state.carts.into_keys().next().unwrap();
    Ok(final_loc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        /->-\
        |   |  /----\
        | /-+--+-\  |
        | | |  | v  |
        \-+-/  \-+--/
          \------/
    "#;

    #[test]
    fn test_part1() {
        assert_eq!("15,4", part1(EXAMPLE).unwrap());
    }
}
