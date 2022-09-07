use eyre::Result;
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day14.txt"),
    part1,
    part2,
};

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let input: usize = input.trim().parse()?;
    let mut recipes = vec![3, 7];
    let mut elves = [0, 1];
    while recipes.len() < input + 10 {
        let new_recipes: usize = elves.iter().map(|&elf| recipes[elf]).sum();
        recipes.extend(
            new_recipes
                .to_string()
                .chars()
                .map(|c: char| c.to_digit(10).unwrap() as usize),
        );
        for elf in &mut elves {
            *elf = (*elf + 1 + recipes[*elf]) % recipes.len();
        }
    }
    let recipes: String = recipes[input..][..10]
        .iter()
        .map(|&recipe| char::from_digit(recipe as u32, 10).unwrap())
        .collect();
    Ok(recipes)
}

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let input: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut recipes = vec![3, 7];
    let mut elves = [0, 1];
    let start = 'outer: loop {
        let new_recipes: usize = elves.iter().map(|&elf| recipes[elf]).sum();
        let new_recipes: Vec<_> = new_recipes
            .to_string()
            .chars()
            .map(|c: char| c.to_digit(10).unwrap() as usize)
            .collect();
        recipes.extend(new_recipes.iter().copied());
        for elf in &mut elves {
            *elf = (*elf + 1 + recipes[*elf]) % recipes.len();
        }
        let search_start = recipes.len().saturating_sub(input.len());
        for start in search_start.saturating_sub(new_recipes.len())..search_start {
            let slice = &recipes[start..][..input.len()];
            if slice == input {
                break 'outer start;
            }
        }
    };
    Ok(start.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("5941429882", part1("2018").unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!("2018", part2("59414").unwrap());
    }
}
