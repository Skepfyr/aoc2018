use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::{AHashMap, AHashSet};
use eyre::{Result, WrapErr};
use recap::Recap;
use serde::Deserialize;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day07.txt"),
    part1,
    part2,
};

#[derive(Debug)]
struct Graph {
    forward: AHashMap<char, AHashSet<char>>,
    backward: AHashMap<char, AHashSet<char>>,
}

fn parse(input: &str) -> Result<Graph> {
    let mut forward: AHashMap<char, AHashSet<char>> = AHashMap::new();
    let mut backward: AHashMap<char, AHashSet<char>> = AHashMap::new();
    for line in input.lines() {
        #[derive(Debug, Deserialize, Recap)]
        #[recap(
            regex = r#"Step (?P<dependency>[A-Z]) must be finished before step (?P<dependant>[A-Z]) can begin."#
        )]
        struct Line {
            dependant: char,
            dependency: char,
        }
        let line: Line = line.parse().wrap_err_with(|| line.to_owned())?;
        forward
            .entry(line.dependency)
            .or_default()
            .insert(line.dependant);
        forward.entry(line.dependant).or_default();
        backward
            .entry(line.dependant)
            .or_default()
            .insert(line.dependency);
        backward.entry(line.dependency).or_default();
    }
    Ok(Graph { forward, backward })
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let mut graph = parse(input)?;

    let mut queue: BinaryHeap<_> = graph
        .backward
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(&k, _)| Reverse(k))
        .collect();

    let mut answer = String::with_capacity(queue.len() + graph.forward.len());

    while let Some(Reverse(next)) = queue.pop() {
        answer.push(next);
        for dependant in graph.forward.remove(&next).into_iter().flatten() {
            let dependencies = graph.backward.get_mut(&dependant).unwrap();
            dependencies.remove(&next);
            if dependencies.is_empty() {
                graph.backward.remove(&dependant);
                queue.push(Reverse(dependant));
            }
        }
    }
    Ok(answer)
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let graph = parse(input)?;
    construction_time(graph, 5, 60).map(|time| time.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Task {
    // Order matter a lot here, sort first by time then name
    completion_time: u32,
    name: char,
}

fn construction_time(mut graph: Graph, workers: usize, base_time: u32) -> Result<u32> {
    let task_time = move |c: char| {
        if c == '#' {
            return 0;
        }
        assert!(c.is_ascii_uppercase());
        base_time + (c as u32 - 'A' as u32 + 1)
    };

    let starts = graph
        .backward
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(&k, _)| k)
        .collect();
    for &start in &starts {
        graph.backward.entry(start).or_default().insert('#');
    }
    graph.forward.insert('#', starts);

    let mut queue: BinaryHeap<_> = BinaryHeap::new();
    queue.push(Reverse(Task {
        completion_time: 0,
        name: '#',
    }));
    let mut workers: BinaryHeap<_> = std::iter::repeat(Reverse(0u32)).take(workers).collect();

    while let Some(Reverse(Task {
        completion_time,
        name: next,
    })) = queue.pop()
    {
        let Reverse(worker) = workers.pop().expect("there are no workers");
        let completion_time = std::cmp::max(completion_time, worker) + task_time(next);
        workers.push(Reverse(completion_time));
        for dependant in graph.forward.remove(&next).into_iter().flatten() {
            let dependencies = graph.backward.get_mut(&dependant).unwrap();
            dependencies.remove(&next);
            if dependencies.is_empty() {
                graph.backward.remove(&dependant);
                queue.push(Reverse(Task {
                    completion_time,
                    name: dependant,
                }));
            }
        }
    }
    Ok(workers
        .into_iter()
        .map(|Reverse(time)| time)
        .max()
        .expect("there are no workers"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;

    #[test]
    fn test_part1() {
        assert_eq!("CABDFE", &part1(INPUT).unwrap());
    }

    #[test]
    fn test_construction_time() {
        assert_eq!(15, construction_time(parse(INPUT).unwrap(), 2, 0).unwrap())
    }
}
