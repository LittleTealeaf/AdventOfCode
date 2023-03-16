use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let values: Vec<&str> = value.split(",").collect();

        Self {
            x: values[0].parse().unwrap(),
            y: values[0].parse().unwrap(),
        }
    }
}

enum Action {
    Set(bool),
    Toggle,
}

struct Instruction {
    action: Action,
    from: Point,
    to: Point,
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let words: Vec<&str> = value.split(" ").collect();

        let action = {
            if words[0].eq("toggle") {
                Action::Toggle
            } else {
                Action::Set(words[1].eq("on"))
            }
        };

        let offset = match action {
            Action::Set(_) => 2,
            Action::Toggle => 1,
        };

        let from = Point::from(words[offset]);
        let to = Point::from(words[offset + 2]);

        Self { action, from, to }
    }
}

fn part_1(fileName: &str) -> Result<u64, Error> {
    let file = File::open(fileName)?;
    let lines = BufReader::new(file).lines();

    let instructions = lines.map(|line| -> Instruction { Instruction::from(line.unwrap()) });

    let mut state = [[false; 1000]; 1000];

    for instruction in instructions {
        for x in (instruction.from.x)..=(instruction.to.x) {
            for y in (instruction.from.y)..=(instruction.to.y) {
                match instruction.action {
                    Action::Set(value) => state[y][x] = value,
                    Action::Toggle => state[y][x] = !state[y][x]
                }
            }
        }
    }

    let mut count = 0;
    for row in state {
        for value in row {
            if value {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn main() {
    println!("Part 1: {}", part_1("input.txt").unwrap());
}
