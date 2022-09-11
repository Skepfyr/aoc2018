use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashSet},
    fmt::{self, Display, Write},
};

use bitvec::vec::BitVec;
use eyre::{bail, eyre, Result};
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day15.txt"),
    part1,
    part2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        usize::cmp(&self.y, &other.y).then_with(|| usize::cmp(&self.x, &other.x))
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Allegiance {
    Elf,
    Goblin,
}

#[derive(Debug)]
struct Unit {
    allegiance: Allegiance,
    health: u8,
    attack: u8,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self.allegiance {
            Allegiance::Elf => 'E',
            Allegiance::Goblin => 'G',
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile<'a> {
    Wall,
    Floor(Option<&'a Unit>),
}

impl<'a> Tile<'a> {
    fn is_open(self) -> bool {
        matches!(self, Tile::Floor(None))
    }

    fn unit(self) -> Option<&'a Unit> {
        match self {
            Tile::Wall => None,
            Tile::Floor(unit) => unit,
        }
    }
}

impl fmt::Display for Tile<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => f.write_char('#'),
            Tile::Floor(None) => f.write_char('.'),
            Tile::Floor(Some(unit)) => Display::fmt(unit, f),
        }
    }
}

#[derive(Debug)]
struct Map {
    round: u32,
    width: usize,
    height: usize,
    grid: BitVec,
    units: BTreeMap<Location, Unit>,
}

impl Map {
    fn builder() -> MapBuilder {
        MapBuilder::default()
    }

    fn get(&self, location: Location) -> Tile<'_> {
        if location.x >= self.width || location.y >= self.height {
            panic!(
                "{} is out of bounds of map of size ({}, {})",
                location, self.width, self.height
            );
        }
        if self.grid[location.y * self.width + location.x] {
            Tile::Wall
        } else {
            Tile::Floor(self.units.get(&location))
        }
    }

    fn adjacent_locations(&self, Location { x, y }: Location) -> impl Iterator<Item = Location> {
        [
            y.checked_sub(1).map(|y| Location { x, y }),
            x.checked_sub(1).map(|x| Location { x, y }),
            x.checked_add(1)
                .filter(|&x| x < self.width)
                .map(|x| Location { x, y }),
            y.checked_add(1)
                .filter(|&y| y < self.height)
                .map(|y| Location { x, y }),
        ]
        .into_iter()
        .flatten()
    }

    fn step_round(&mut self) -> RoundResult {
        let mut elves_killed = 0;
        for mut unit_loc in self.units.keys().copied().collect::<Vec<_>>() {
            let unit = match self.units.remove(&unit_loc) {
                Some(unit) => unit,
                None => continue,
            };

            let mut targets_remain = false;

            let in_range: HashSet<_> = self
                .units
                .iter()
                .filter(|(_, target)| target.allegiance != unit.allegiance)
                .inspect(|_| targets_remain = true)
                .flat_map(|(&location, _)| self.adjacent_locations(location))
                .filter(|&location| self.get(location).is_open() || location == unit_loc)
                .collect();

            if !targets_remain {
                self.units.insert(unit_loc, unit);
                return RoundResult {
                    finished: true,
                    elves_killed,
                };
            }

            // Move
            if !in_range.contains(&unit_loc) {
                let mut best = None;
                let mut best_step = None;
                let mut distances = vec![vec![u32::MAX; self.width]; self.height];
                distances[unit_loc.y][unit_loc.x] = 0;
                for first_step in self.adjacent_locations(unit_loc) {
                    if !self.get(first_step).is_open() {
                        continue;
                    }
                    let mut min = None;
                    let mut open_set = BinaryHeap::new();
                    open_set.push(Reverse((1, first_step)));
                    while let Some(Reverse((dist, step))) = open_set.pop() {
                        if min.map(|(min_dist, _)| dist > min_dist).unwrap_or(false) {
                            break;
                        }
                        if in_range.contains(&step)
                            && min.map(|(_, min_dest)| step < min_dest).unwrap_or(true)
                        {
                            min = Some((dist, step));
                        }
                        for neighbour in self.adjacent_locations(step) {
                            if !self.get(neighbour).is_open() {
                                continue;
                            }
                            if dist + 1 < distances[neighbour.y][neighbour.x] {
                                distances[neighbour.y][neighbour.x] = dist + 1;
                                open_set.push(Reverse((dist + 1, neighbour)));
                            }
                        }
                    }
                    match (min, best) {
                        (Some((min_dist, min_dest)), Some((best_dist, best_dest))) => {
                            if min_dist < best_dist
                                || (min_dist == best_dist && min_dest < best_dest)
                            {
                                best = min;
                                best_step = Some(first_step);
                            }
                        }
                        (Some(_), None) => {
                            best = min;
                            best_step = Some(first_step);
                        }
                        (None, _) => {}
                    }
                }
                if let Some(best_step) = best_step {
                    unit_loc = best_step;
                }
            }
            let old = self.units.insert(unit_loc, unit);
            debug_assert!(old.is_none());
            let unit = self.units.get(&unit_loc).unwrap();

            // Attack
            let weakest_enemy = self
                .adjacent_locations(unit_loc)
                .filter_map(|loc| {
                    self.get(loc)
                        .unit()
                        .filter(|target| target.allegiance != unit.allegiance)
                        .map(|unit| (loc, unit))
                })
                .min_by_key(|(_, unit)| unit.health);
            if let Some((enemy_loc, _)) = weakest_enemy {
                let attack = unit.attack;
                let enemy = self.units.get_mut(&enemy_loc).unwrap();
                match enemy.health.checked_sub(attack) {
                    None | Some(0) => {
                        if enemy.allegiance == Allegiance::Elf {
                            elves_killed += 1;
                        }
                        self.units.remove(&enemy_loc);
                    }
                    Some(health) => {
                        enemy.health = health;
                    }
                }
            }
        }
        self.round += 1;
        RoundResult {
            finished: false,
            elves_killed,
        }
    }

    fn score(&self) -> u32 {
        let remaining_health: u32 = self.units.iter().map(|(_, unit)| unit.health as u32).sum();
        self.round * remaining_health
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                fmt::Display::fmt(&self.get(Location { x, y }), f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct RoundResult {
    finished: bool,
    elves_killed: usize,
}

#[derive(Debug)]
struct MapBuilder {
    elf_attack: u8,
    goblin_attack: u8,
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            elf_attack: 3,
            goblin_attack: 3,
        }
    }
}

impl MapBuilder {
    fn elf_attack(&mut self, attack: u8) -> &mut Self {
        self.elf_attack = attack;
        self
    }

    fn parse(&self, s: &str) -> Result<Map> {
        let mut grid = BitVec::with_capacity(s.len());
        let mut units = BTreeMap::new();
        let width = s.lines().next().ok_or_else(|| eyre!("map is empty"))?.len();
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            if line.len() != width {
                bail!(
                    "Map must be rectangular, line {} is a different length to the first",
                    y + 1
                )
            }
            height += 1;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => grid.push(true),
                    '.' => grid.push(false),
                    'E' => {
                        units.insert(
                            Location { x, y },
                            Unit {
                                allegiance: Allegiance::Elf,
                                health: 200,
                                attack: self.elf_attack,
                            },
                        );
                        grid.push(false)
                    }
                    'G' => {
                        units.insert(
                            Location { x, y },
                            Unit {
                                allegiance: Allegiance::Goblin,
                                health: 200,
                                attack: self.goblin_attack,
                            },
                        );
                        grid.push(false)
                    }
                    _ => bail!("Unexpected character in map {:?}", c),
                }
            }
        }
        Ok(Map {
            round: 0,
            width,
            height,
            grid,
            units,
        })
    }
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let mut map = Map::builder().parse(input)?;
    while !map.step_round().finished {}
    Ok(format!("{}", map.score()))
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let mut attack = 3;
    let map = loop {
        attack += 1;
        let mut map = Map::builder().elf_attack(attack).parse(input)?;
        let succeeded = loop {
            let result = map.step_round();
            if result.elves_killed > 0 {
                break false;
            } else if result.finished {
                break true;
            }
        };
        if succeeded {
            break map;
        }
    };
    Ok(format!("{}", map.score()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement() {
        const MAP: &str = "\
            #########\n\
            #G..G..G#\n\
            #.......#\n\
            #.......#\n\
            #G..E..G#\n\
            #.......#\n\
            #.......#\n\
            #G..G..G#\n\
            #########\n\
        ";
        let mut map: Map = Map::builder().parse(MAP).unwrap();
        println!("{}", map);
        assert_eq!(MAP, format!("{}", map));
        map.step_round();
        println!("{}", map);
        assert_eq!(
            "\
            #########\n\
            #.G...G.#\n\
            #...G...#\n\
            #...E..G#\n\
            #.G.....#\n\
            #.......#\n\
            #G..G..G#\n\
            #.......#\n\
            #########\n\
            ",
            format!("{}", map)
        );
        map.step_round();
        println!("{}", map);
        assert_eq!(
            "\
            #########\n\
            #..G.G..#\n\
            #...G...#\n\
            #.G.E.G.#\n\
            #.......#\n\
            #G..G..G#\n\
            #.......#\n\
            #.......#\n\
            #########\n\
            ",
            format!("{}", map)
        );
        map.step_round();
        println!("{}", map);
        assert_eq!(
            "\
            #########\n\
            #.......#\n\
            #..GGG..#\n\
            #..GEG..#\n\
            #G..G...#\n\
            #......G#\n\
            #.......#\n\
            #.......#\n\
            #########\n\
            ",
            format!("{}", map)
        );
    }

    #[test]
    fn test_part1() {
        const MAP: &str = "\
            #######\n\
            #.G...#\n\
            #...EG#\n\
            #.#.#G#\n\
            #..G#E#\n\
            #.....#\n\
            #######\n\
        ";
        assert_eq!("27730", part1(MAP).unwrap());
    }

    #[test]
    fn test_part2() {
        const MAP: &str = "\
            #######\n\
            #.G...#\n\
            #...EG#\n\
            #.#.#G#\n\
            #..G#E#\n\
            #.....#\n\
            #######\n\
        ";
        assert_eq!("4988", part2(MAP).unwrap());
    }
}
