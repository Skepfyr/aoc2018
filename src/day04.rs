use std::{ops::Range, str::FromStr};

use ahash::AHashMap;
use eyre::{bail, eyre, Result};
use time::{macros::format_description, PrimitiveDateTime};
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day04.txt"),
    part1,
    part2,
};

#[derive(Debug, Clone, Copy)]
enum Event {
    ShiftStart { time: PrimitiveDateTime, guard: u32 },
    FellAsleep { time: PrimitiveDateTime },
    WokeUp { time: PrimitiveDateTime },
}

impl Event {
    fn time(self) -> PrimitiveDateTime {
        match self {
            Event::ShiftStart { time, .. }
            | Event::FellAsleep { time }
            | Event::WokeUp { time } => time,
        }
    }
}

impl FromStr for Event {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time, action) = s.split_at(18);
        let time = PrimitiveDateTime::parse(
            &time[1..17],
            format_description!("[year]-[month]-[day] [hour]:[minute]"),
        )?;
        Ok(match &action[1..6] {
            "Guard" => {
                let guard = action[8..]
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse()?;
                Event::ShiftStart { time, guard }
            }
            "falls" => Event::FellAsleep { time },
            "wakes" => Event::WokeUp { time },
            action => bail!("Unrecognized action {:?}", action),
        })
    }
}

fn parse(input: &str) -> Result<AHashMap<u32, Vec<Range<u8>>>> {
    let mut events: Vec<Event> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;
    events.sort_unstable_by_key(|event| event.time());
    let mut events = events.into_iter();

    let mut guards = AHashMap::<_, Vec<_>>::new();
    let mut guard = match events.next() {
        Some(Event::ShiftStart { guard, .. }) => guard,
        _ => bail!("First event must be a shift start"),
    };
    while let Some(event) = events.next() {
        let sleep_time = match event {
            Event::ShiftStart { guard: new, .. } => {
                guard = new;
                continue;
            }
            Event::FellAsleep { time } => time,
            Event::WokeUp { time } => bail!("Woke up twice in a row at {}", time),
        };
        let wake_time = match events.next() {
            Some(Event::WokeUp { time }) => time,
            _ => bail!("Never woke up after {}", sleep_time),
        };
        guards
            .entry(guard)
            .or_default()
            .push(sleep_time.minute()..wake_time.minute())
    }
    Ok(guards)
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let guards = parse(input)?;
    let (guard, times) = guards
        .into_iter()
        .max_by_key(|(_, times)| times.iter().map(|range| range.len()).sum::<usize>())
        .ok_or_else(|| eyre!("No events?"))?;
    let minute: u32 = (0..60)
        .max_by_key(|minute| times.iter().filter(|range| range.contains(minute)).count())
        .unwrap()
        .into();
    Ok((guard * minute).to_string())
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let guards = parse(input)?;
    let (guard, minute, _) = guards
        .into_iter()
        .map(|(guard, times)| {
            let (minute, count) = (0..60)
                .map(|minute| {
                    (
                        minute,
                        times.iter().filter(|range| range.contains(&minute)).count(),
                    )
                })
                .max_by_key(|&(_, count)| count)
                .unwrap();
            (guard, minute, count)
        })
        .max_by_key(|&(_, _, count)| count)
        .unwrap();
    Ok((guard * u32::from(minute)).to_string())
}
