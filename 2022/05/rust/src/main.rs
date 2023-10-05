fn main() {
    let (state, instructions) = parse_input(include_str!("../../input.txt"));

    println!("Part 1: {}", part_1(state.clone(), instructions.clone()));
    println!("Part 2: {}", part_2(state, instructions));
}

#[derive(Copy, Clone)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut lines = input.lines();

    let mut state = Vec::new();
    let mut instructions = Vec::new();
    let mut columns = 0;

    loop {
        let line = lines.next().unwrap();

        if columns == 0 {
            columns = (line.len() + 1) / 4;
            state = vec![Vec::new(); columns];
        } else if line == "" {
            break;
        }

        let chars = line.chars().collect::<Vec<_>>();

        for i in 0..columns {
            let index = 4 * i + 1;
            let ch = chars[index];
            if ch != ' ' {
                state[i].push(ch);
            }
        }
    }

    while let Some(line) = lines.next() {
        let mut words = line.split(" ");
        words.next();
        let count = words.next().unwrap().parse::<usize>().unwrap();
        words.next();
        let from = words.next().unwrap().parse::<usize>().unwrap();
        words.next();
        let to = words.next().unwrap().parse::<usize>().unwrap();
        instructions.push(Instruction { count, from, to })
    }

    for i in 0..columns {
        state[i].reverse();
    }

    (state, instructions)
}

fn part_1(mut state: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions.into_iter() {
        for _ in 0..instruction.count {
            if let Some(ch) = state[instruction.from - 1].pop() {
                state[instruction.to - 1].push(ch);
            }
        }
    }

    state
        .into_iter()
        .filter_map(|col| col.into_iter().rev().next())
        .collect()
}

fn part_2(mut state: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions.into_iter() {
        let mut stack = Vec::new();

        for _ in 0..instruction.count {
            if let Some(ch) = state[instruction.from - 1].pop() {
                stack.push(ch);
            }
        }

        stack.reverse();

        state[instruction.to - 1].extend(stack);
    }

    state
        .into_iter()
        .filter_map(|col| col.into_iter().rev().next())
        .collect()
}
