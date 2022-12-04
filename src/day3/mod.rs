use std::collections::btree_set::BTreeSet;

use itertools::Itertools;

fn item_to_priority(repeated_item: char) -> u32 {
    let a: u32 = 'a'.into();
    let z: u32 = 'z'.into();
    let uppercase_a: u32 = 'A'.into();
    let uppercase_z: u32 = 'Z'.into();

    let repeated_item: u32 = repeated_item.into();
    let repeated_item = match repeated_item {
        item if a <= item && item <= z => item - (a - 1),
        item if uppercase_a <= item && item <= uppercase_z => {
            item - ((uppercase_a - 1) - (z - a) - 1)
        }
        _ => panic!(),
    };
    repeated_item
}

fn preprocess_lines_part1(lines: std::str::Lines) -> impl Iterator<Item = &str> {
    lines
        .map(|line| {
            let len = line.len() / 2;
            let (first, second) = line.split_at(len);
            [first, second]
        })
        .flatten()
}

fn preprocess_lines_part2(lines: std::str::Lines) -> impl Iterator<Item = &str> {
    lines
}

fn solution<'a, PreprocessLinesOutput>(
    preprocess_lines: impl Fn(std::str::Lines<'a>) -> PreprocessLinesOutput,
    num_chunks: usize,
) -> u32
where
    PreprocessLinesOutput: Iterator<Item = &'a str>,
{
    let content = include_str!("input.txt");

    let lines = content.lines();
    let result = preprocess_lines(lines)
        .chunks(num_chunks)
        .into_iter()
        .map(|mut chunk| {
            let first_set = chunk.next().unwrap().chars().collect::<BTreeSet<_>>();
            let rest = chunk;
            let intersection = rest.fold(first_set, |current_set, string| {
                &current_set & &string.chars().collect()
            });
            intersection.iter().next().unwrap().clone()
        })
        .map(|repeated_item| item_to_priority(repeated_item))
        .fold(0, std::ops::Add::add);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let score = solution(preprocess_lines_part1, 2);
        assert_eq!(score, 8349);
    }

    #[test]
    fn test_solution_part2() {
        let score = solution(preprocess_lines_part2, 3);
        assert_eq!(score, 2681);
    }
}
