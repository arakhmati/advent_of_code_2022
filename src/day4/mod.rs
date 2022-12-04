#[derive(Debug)]
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn from_str(interval: &str) -> Self {
        let mut split = interval.split("-");
        let start: usize = split.next().unwrap().parse().unwrap();
        let end: usize = split.next().unwrap().parse().unwrap();
        Self { start, end }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end <= other.end && self.end >= other.start)
    }
}

fn fully_contains(interval_0: &Interval, interval_1: &Interval) -> bool {
    interval_0.fully_contains(interval_1) || interval_1.fully_contains(interval_0)
}

fn overlaps(interval_0: &Interval, interval_1: &Interval) -> bool {
    interval_0.overlaps(interval_1) || interval_1.overlaps(interval_0)
}

fn solution(predicate: fn(&Interval, &Interval) -> bool) -> u32 {
    let content = include_str!("input.txt");

    let result = content
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            let interval_0 = Interval::from_str(split.next().unwrap());
            let interval_1 = Interval::from_str(split.next().unwrap());
            let condition = predicate(&interval_0, &interval_1);
            condition as u32
        })
        .fold(0, std::ops::Add::add);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let score = solution(fully_contains);
        assert_eq!(score, 433);
    }

    #[test]
    fn test_solution_part2() {
        let score = solution(overlaps);
        assert_eq!(score, 852);
    }
}
