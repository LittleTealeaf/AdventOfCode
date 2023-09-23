use std::num::ParseIntError;

#[derive(Debug)]
enum ParseError {
    InvalidTokenLength,
    IntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        ParseError::IntError(value)
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = value.split(",").collect::<Vec<_>>();
        Ok(Point {
            x: tokens[0].parse()?,
            y: tokens[1].parse()?,
        })
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

impl TryFrom<String> for Instruction {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value.split(" ").collect::<Vec<_>>();
        match tokens.len() {
            4 => Ok(Self {
                action: Action::Toggle,
                from: Point::try_from(tokens[1])?,
                to: Point::try_from(tokens[3])?,
            }),
            5 => Ok(Self {
                action: Action::Set(tokens[1].eq("on")),
                from: Point::try_from(tokens[2])?,
                to: Point::try_from(tokens[4])?,
            }),
            _ => Err(ParseError::InvalidTokenLength),
        }
    }
}

fn main() {
    {
        let instructions = include_str!("../../../../inputs/2015/06/input.txt")
            .lines()
            .into_iter()
            .map(String::from)
            .map(Instruction::try_from)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        println!("Part 1: {}", part_1(instructions))
    }
    {
        let instructions = include_str!("../../../../inputs/2015/06/input.txt")
            .lines()
            .into_iter()
            .map(String::from)
            .map(Instruction::try_from)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        println!("Part 2: {}", part_2(instructions))
    }
}

fn part_1(instructions: Vec<Instruction>) -> usize {
    let mut state = [[false; 1000]; 1000];

    for instruction in instructions {
        for x in (instruction.from.x)..=(instruction.to.x) {
            for y in (instruction.from.y)..=(instruction.to.y) {
                state[y][x] = match instruction.action {
                    Action::Toggle => !state[y][x],
                    Action::Set(value) => value,
                }
            }
        }
    }

    let mut total = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if state[y][x] {
                total += 1;
            }
        }
    }

    total
}
fn part_2(instructions: Vec<Instruction>) -> i64 {
    let mut state = [[0; 1000]; 1000];

    for instruction in instructions {
        for x in (instruction.from.x)..=(instruction.to.x) {
            for y in (instruction.from.y)..=(instruction.to.y) {
                state[y][x] = i64::max(
                    0,
                    state[y][x]
                        + match instruction.action {
                            Action::Toggle => 2,
                            Action::Set(value) => {
                                if value {
                                    1
                                } else {
                                    -1
                                }
                            }
                        },
                )
            }
        }
    }

    let mut total = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            total += state[y][x];
        }
    }

    total
}
