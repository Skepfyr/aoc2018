use bytemuck::zeroed_box;
use eyre::Result;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day11.txt"),
    part1,
    part2,
};

const GRID_SIZE: usize = 300;

fn make_grid(serial: i32) -> [[i32; GRID_SIZE]; GRID_SIZE] {
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
    #[allow(clippy::needless_range_loop)]
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let rack_id = x as i32 + 11;
            let mut power_level = rack_id * (y as i32 + 1);
            power_level += serial;
            power_level *= rack_id;
            let hundreds_digit = (power_level / 100) % 10;
            grid[y][x] = hundreds_digit - 5;
        }
    }
    grid
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let grid = make_grid(input.trim().parse()?);
    let (x, y) = (1..=GRID_SIZE - 2)
        .flat_map(|y| (1..=GRID_SIZE - 2).map(move |x| (x, y)))
        .max_by_key(|&(x, y)| {
            grid[y - 1..y + 2]
                .iter()
                .flat_map(|row| row[x - 1..x + 2].iter().copied())
                .sum::<i32>()
        })
        .unwrap();
    Ok(format!("{x},{y}"))
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let mut grids: Box<[[[i32; GRID_SIZE]; GRID_SIZE]; GRID_SIZE + 1]> = zeroed_box();
    grids[1] = make_grid(input.trim().parse()?);
    for size in 2..=GRID_SIZE {
        for y in 0..=GRID_SIZE - size {
            for x in 0..=GRID_SIZE - size {
                grids[size][y][x] = if size % 2 == 0 {
                    let half_grid = &grids[size / 2];
                    half_grid[y][x]
                        + half_grid[y][x + size / 2]
                        + half_grid[y + size / 2][x]
                        + half_grid[y + size / 2][x + size / 2]
                } else {
                    let mid = size / 2;
                    let half_grid_up = &grids[mid + 1];
                    let half_grid_down = &grids[mid];
                    half_grid_up[y][x]
                        + half_grid_up[y + mid][x + mid]
                        + half_grid_down[y + mid + 1][x]
                        + half_grid_down[y][x + mid + 1]
                        - grids[1][y + mid][x + mid]
                };
            }
        }
    }
    let (x, y, size) = (1..=GRID_SIZE)
        .flat_map(|size| (1..=GRID_SIZE - size + 1).map(move |y| (y, size)))
        .flat_map(|(y, size)| (1..=GRID_SIZE - size + 1).map(move |x| (x, y, size)))
        .max_by_key(|&(x, y, size)| grids[size][y - 1][x - 1])
        .unwrap();
    Ok(format!("{x},{y},{size}"))
}
