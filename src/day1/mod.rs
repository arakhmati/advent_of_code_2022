use std::collections::BTreeSet;
use std::num::ParseIntError;

use itertools::Itertools;

#[allow(dead_code)]
fn solution() -> Result<(u32, u32), ParseIntError> {
    let content = include_str!("input.txt");

    let lines = 
        content
        .split("\n")
        .map(| line | line.trim());

    let calories_per_elf: BTreeSet<u32> = 
        lines
        .group_by(| line | line.is_empty())
        .into_iter()
        .filter(|(is_empty, _)| !(*is_empty) )
        .map(
            | (_, group) | {
                group
                .map(| element | element.parse::<u32>())
                .fold_ok(0, std::ops::Add::add)
            })
        .try_collect()?;

    let top_1 = calories_per_elf.iter().max().unwrap().clone();
    let top_3_sum = calories_per_elf.iter().rev().take(3).fold(0, std::ops::Add::add);

    Ok((top_1, top_3_sum))
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn test_solution() {
        let result = solution();
        assert_eq!(result.unwrap(), (66719, 198551));
    }
}