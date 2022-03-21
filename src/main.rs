use std::{fs, path::PathBuf};

use aoc2018::SOLUTIONS;
use clap::Parser;
use color_eyre::Result;
use eyre::bail;
use tracing_error::ErrorLayer;
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug, Parser)]
struct Args {
    day: usize,
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(ErrorLayer::default())
        .init();
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
