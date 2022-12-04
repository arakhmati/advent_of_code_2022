use std::collections::BTreeSet;
use std::num::ParseIntError;

use itertools::Itertools;

#[allow(dead_code)]
fn solution(num_top_elfs: usize) -> Result<u32, ParseIntError> {
    let content = include_str!("input.txt");

    let lines = content.lines().map(|line| line.trim());

    let calories_per_elf: BTreeSet<u32> = lines
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(is_empty, _)| !(*is_empty))
        .map(|(_, group)| {
            group
                .map(|element| element.parse::<u32>())
                .fold_ok(0, std::ops::Add::add)
        })
        .try_collect()?;

    let top_n_sum = calories_per_elf
        .iter()
        .rev()
        .take(num_top_elfs)
        .fold(0, std::ops::Add::add);
    Ok(top_n_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let result = solution(1);
        assert_eq!(result.unwrap(), 66719);
    }

    #[test]
    fn test_solution_part2() {
        let result = solution(3);
        assert_eq!(result.unwrap(), 198551);
    }
}
