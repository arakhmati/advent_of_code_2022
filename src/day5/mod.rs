#[derive(Debug)]
struct MoveConfig {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_input_file() -> (Vec<Vec<char>>, Vec<MoveConfig>) {
    let content = include_str!("input.txt");

    let mut lines = content.lines();

    let mut stacks_lines = vec![];
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        stacks_lines.push(line);
    }
    let mut reversed_stacks_lines = stacks_lines.iter().rev();
    let ids = reversed_stacks_lines
        .next()
        .unwrap()
        .clone()
        .split(" ")
        .filter(|element| !element.is_empty())
        .map(|element| element.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in reversed_stacks_lines {
        for id in ids.iter() {
            let index = id * 4 + 1;
            let character = line.chars().nth(index).unwrap();
            if character == ' ' {
                continue;
            }
            if stacks.len() <= id.clone() {
                stacks.push(vec![]);
            }
            stacks[id.clone()].push(character);
        }
    }

    let move_configs = lines
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            let quantity = tokens[1].parse::<usize>().unwrap();
            let from = tokens[3].parse::<usize>().unwrap() - 1;
            let to = tokens[5].parse::<usize>().unwrap() - 1;
            MoveConfig { quantity, from, to }
        })
        .collect::<Vec<_>>();

    (stacks, move_configs)
}

fn solution(multiple_crates: bool) -> String {
    let (mut stacks, move_configs) = parse_input_file();

    for move_config in move_configs {
        let from = move_config.from;
        let to = move_config.to;

        let mut items = vec![];
        for _ in 0..move_config.quantity {
            let item = stacks[from].pop().unwrap();
            items.push(item);
        }

        if multiple_crates {
            for item in items.iter().rev() {
                stacks[to].push(item.clone());
            }
        } else {
            for item in items {
                stacks[to].push(item.clone());
            }
        }
    }
    let mut result = vec![];
    for mut stack in stacks {
        result.push(stack.pop().unwrap().to_string());
    }
    let result = result.join("");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let score = solution(false);
        assert_eq!(score, "GFTNRBZPF");
    }

    #[test]
    fn test_solution_part2() {
        let score = solution(true);
        assert_eq!(score, "VRQWPDSGP");
    }
}
