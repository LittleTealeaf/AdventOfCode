fn main() {
    let instructions = parse_instructions(include_str!("../../input.txt"));

    println!("Part 1: {}", part_1(&instructions));
    part_2(&instructions);
}

enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split(' ');
            let command = tokens.next().unwrap();

            match command {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(tokens.next().unwrap().parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

fn part_1(instructions: &Vec<Instruction>) -> i32 {
    let mut x = 1;

    let mut sum = 0;
    let cycles = vec![20, 60, 100, 140, 180, 220];

    let mut cycle = 1;

    for instruction in instructions {
        if cycles.contains(&cycle) {
            sum += x * cycle;
        }
        if let Instruction::AddX(n) = instruction {
            cycle += 1;
            if cycles.contains(&cycle) {
                sum += x * cycle;
            }
            x += n;
        }
        cycle += 1;
    }

    sum
}

fn part_2(instructions: &Vec<Instruction>) {
    let mut x: i32 = 1;

    let mut cycle: i32 = 1;
    let mut display = [false; 240];

    for instruction in instructions {
        display[cycle as usize - 1] = x.abs_diff((cycle - 1) % 40) < 2;
        if let Instruction::AddX(n) = instruction {
            cycle += 1;
            display[cycle as usize - 1] = x.abs_diff((cycle - 1) % 40) < 2;
            x += n;
        }
        cycle += 1;
    }

    for (i, v) in display.into_iter().enumerate() {
        if i % 40 == 0 {
            print!("\n");
        }
        if v {
            print!("#");
        } else {
            print!(" ");
        }
    }
}
