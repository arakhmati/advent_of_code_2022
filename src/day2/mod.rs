#[derive(Debug, Clone, Eq, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn to_winning(&self) -> Self {
        use HandShape::*;
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn to_losing(&self) -> Self {
        use HandShape::*;
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

fn compute_your_hand_shape_part1(_: &HandShape, you: &str) -> HandShape {
    match you {
        "X" => HandShape::Rock,
        "Y" => HandShape::Paper,
        "Z" => HandShape::Scissors,
        _ => panic!(),
    }
}

fn compute_your_hand_shape_part2(opponent: &HandShape, you: &str) -> HandShape {
    match (opponent, you) {
        (shape, "X") => shape.to_losing(),
        (shape, "Y") => shape.clone(),
        (shape, "Z") => shape.to_winning(),
        _ => panic!(),
    }
}

fn compute_score(
    game: (&str, &str),
    compute_your_hand_shape: fn(&HandShape, &str) -> HandShape,
) -> u32 {
    let (opponent, you) = game;

    let opponent_hand_shape = match opponent {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scissors,
        _ => panic!(),
    };

    let your_hand_shape = compute_your_hand_shape(&opponent_hand_shape, you);

    let shape_score = match your_hand_shape {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    };

    let game_score = match (opponent_hand_shape, your_hand_shape) {
        (opponent, you) if opponent.to_winning() == you => 6,
        (opponent, you) if opponent.to_losing() == you => 0,
        _ => 3,
    };

    let score = shape_score + game_score;
    score
}

fn solution(compute_your_hand_shape: fn(&HandShape, &str) -> HandShape) -> u32 {
    let content = include_str!("input.txt");

    let lines = content.lines();

    let score = lines
        .map(|line| line.split(" "))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|game| compute_score(game, compute_your_hand_shape))
        .fold(0, std::ops::Add::add);
    score
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    #[test]
    fn test_solution_part1() {
        let score = solution(compute_your_hand_shape_part1);
        assert_eq!(score, 12740);
    }

    #[test]
    fn test_solution_part2() {
        let score = solution(compute_your_hand_shape_part2);
        assert_eq!(score, 11980);
    }
}
