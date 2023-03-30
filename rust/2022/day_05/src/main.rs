struct Action {
    count: usize,
    start: usize,
    end: usize,
}

struct Input {
    stacks: Vec<Vec<char>>,
    actions: Vec<Action>,
}

fn parse_input(input: &str) -> Input {
    let (stack_str, actions_str) = input.split_once("\n\n").unwrap();

    // Stack
    let stacks = {
        let mut lines = stack_str.lines().collect::<Vec<_>>();
        let len = (lines.pop().unwrap().len() + 1) / 4;

        let mut stacks = Vec::new();

        for i in 0..len {
            let mut stack = Vec::new();
            for line in &lines {
                let ch = line.chars().collect::<Vec<char>>()[1 + i * 4];
                if ch != ' ' {
                    stack.push(ch);
                }
            }
            stack.reverse();
            stacks.push(stack);
        }

        stacks
    };

    let actions = actions_str
        .lines()
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            let count = tokens[1].parse().unwrap();
            let start = tokens[3].parse().unwrap();
            let end = tokens[5].parse().unwrap();

            Action { count, start, end }
        })
        .collect::<Vec<_>>();

    // Actions
    Input { stacks, actions }
}

fn main() {
    println!(
        "Part 1: {}",
        part_1(parse_input(include_str!("../input.txt")))
    );

    println!("Part 2: {}", part_2(parse_input(include_str!("../input.txt"))));
}

fn part_1(Input { actions, stacks }: Input) -> String {
    let mut stacks = stacks.clone();

    for Action { count, start, end } in actions {
        for _ in 0..count {
            let value = stacks.get_mut(start - 1).unwrap().pop().unwrap();
            stacks.get_mut(end - 1).unwrap().push(value);
        }
    }

    stacks
        .iter_mut()
        .map(Vec::pop)
        .map(Option::unwrap)
        .collect::<String>()
}


fn part_2(Input {actions, stacks}: Input) -> String {
    let mut stacks = stacks.clone();

    for Action { count, start, end } in actions {
        let mut values = Vec::new();
        for _ in 0..count {
            values.push(stacks.get_mut(start - 1).unwrap().pop().unwrap());
        }
        values.reverse();
        stacks.get_mut(end - 1).unwrap().extend(values);
    }

    stacks
        .iter_mut()
        .map(Vec::pop)
        .map(Option::unwrap)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input(include_str!("../test_input.txt"));
        let result = part_1(input);
        assert_eq!(result, String::from("CMZ"));
    }


    #[test]
    fn test_part_2() {
        let input = parse_input(include_str!("../test_input.txt"));
        let result = part_2(input);
        assert_eq!(result, String::from("MCD"));
    }
}
