use std::collections::VecDeque;

use eyre::Result;
use recap::Recap;
use serde::Deserialize;
use tracing::{debug, instrument};

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day09.txt"),
    part1,
    part2,
};

#[derive(Debug, Clone, Copy, Deserialize, Recap)]
#[recap(regex = r#"(?P<players>\d+) players; last marble is worth (?P<marbles>\d+) points"#)]
struct Rules {
    players: usize,
    marbles: usize,
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    Ok(max_score(input.parse()?).to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let mut rules: Rules = input.parse()?;
    rules.marbles *= 100;
    Ok(max_score(rules).to_string())
}

fn max_score(rules: Rules) -> usize {
    let mut game = VecDeque::with_capacity(rules.marbles + 1);
    let mut player = 0;
    let mut players = vec![0; rules.players];
    game.push_back(0);
    for marble in 1..=rules.marbles {
        if marble % 23 == 0 {
            players[player] += marble;
            game.rotate_right(7);
            players[player] += game.pop_back().expect("This marble exists");
            game.rotate_left(1);
        } else {
            game.rotate_left(1);
            game.push_back(marble);
        }
        debug!(?game);
        player = (player + 1) % rules.players;
    }
    players.iter().copied().max().expect("> 0 players")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(
            32,
            max_score(Rules {
                players: 9,
                marbles: 25
            }),
        );
        assert_eq!(
            8317,
            max_score(Rules {
                players: 10,
                marbles: 1618
            }),
        );
        assert_eq!(
            146373,
            max_score(Rules {
                players: 13,
                marbles: 7999
            }),
        );
        assert_eq!(
            2764,
            max_score(Rules {
                players: 17,
                marbles: 1104
            }),
        );
        assert_eq!(
            54718,
            max_score(Rules {
                players: 21,
                marbles: 6111
            }),
        );
        assert_eq!(
            37305,
            max_score(Rules {
                players: 30,
                marbles: 5807
            }),
        );
    }
}
