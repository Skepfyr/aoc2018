use std::{fs, path::PathBuf};

use aoc2018::SOLUTIONS;
use clap::Parser;
use color_eyre::Result;
use eyre::bail;

#[derive(Debug, Parser)]
struct Args {
    day: usize,
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    let args = Args::parse();

    if args.day > SOLUTIONS.len() {
        bail!("Only solutions for days 1-{} exist", SOLUTIONS.len());
    }
    let solution = &SOLUTIONS[args.day - 1];
    let input = match args.input {
        Some(path) => Box::leak(fs::read_to_string(path)?.into_boxed_str()),
        None => solution.input,
    };
    println!("Part 1: {}", (solution.part1)(input)?);
    println!("Part 2: {}", (solution.part2)(input)?);
    Ok(())
}
