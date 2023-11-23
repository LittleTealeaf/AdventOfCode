fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .filter_map(|line| Instruction::try_from(line).ok())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Instruction {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value.trim().split(" ");
        let direction = match tokens.next().unwrap() {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }?;
        let steps = tokens.next().unwrap().parse::<i32>().unwrap();

        Ok(Self { direction, steps })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn part_1(input: &Vec<Instruction>) -> usize {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    let mut positions = vec![Pos { x: 0, y: 0 }];

    for instruction in input {
        for _ in 0..instruction.steps {
            match instruction.direction {
                Direction::Up => {
                    head.y += 1;
                    if tail.y < head.y - 1 {
                        tail.y = head.y - 1;
                        tail.x = head.x;
                    }
                }
                Direction::Down => {
                    head.y -= 1;
                    if tail.y > head.y + 1 {
                        tail.y = head.y + 1;
                        tail.x = head.x;
                    }
                }
                Direction::Left => {
                    head.x -= 1;
                    if tail.x > head.x + 1 {
                        tail.x = head.x + 1;
                        tail.y = head.y;
                    }
                }
                Direction::Right => {
                    head.x += 1;
                    if tail.x < head.x - 1 {
                        tail.x = head.x - 1;
                        tail.y = head.y;
                    }
                }
            }

            if !positions.contains(&tail) {
                positions.push(tail)
            }
        }
    }

    positions.len()
}

#[test]
fn test_part_1() {
    let instructions = vec![
        Instruction {
            direction: Direction::Right,
            steps: 4,
        },
        Instruction {
            direction: Direction::Up,
            steps: 4,
        },
        Instruction {
            direction: Direction::Left,
            steps: 3,
        },
        Instruction {
            direction: Direction::Down,
            steps: 1,
        },
        Instruction {
            direction: Direction::Right,
            steps: 4,
        },
        Instruction {
            direction: Direction::Down,
            steps: 1,
        },
        Instruction {
            direction: Direction::Left,
            steps: 5,
        },
        Instruction {
            direction: Direction::Right,
            steps: 2,
        },
    ];

    assert_eq!(part_1(&instructions), 13);
}

fn part_2(input: &Vec<Instruction>) -> usize {
    let mut nodes = [Pos { x: 0, y: 0 }; 10];

    let mut positions = vec![Pos { x: 0, y: 0 }];

    for instr in input {
        for _ in 0..instr.steps {
            match instr.direction {
                Direction::Up => nodes[0].y += 1,
                Direction::Down => nodes[0].y -= 1,
                Direction::Left => nodes[0].x -= 1,
                Direction::Right => nodes[0].x += 1,
            }

            for i in 1..10 {
                let hor_off = nodes[i - 1].x.abs_diff(nodes[i].x) > 1;
                let ver_off = nodes[i - 1].y.abs_diff(nodes[i].y) > 1;

                match (hor_off, ver_off) {
                    (true, false) => {
                        nodes[i].x = (nodes[i].x + nodes[i - 1].x) / 2;
                        nodes[i].y = nodes[i - 1].y;
                    }
                    (false, true) => {
                        nodes[i].y = (nodes[i].y + nodes[i - 1].y) / 2;
                        nodes[i].x = nodes[i - 1].x;
                    }
                    (true, true) => {
                        nodes[i].x = (nodes[i].x + nodes[i - 1].x) / 2;
                        nodes[i].y = (nodes[i].y + nodes[i - 1].y) / 2;
                    }
                    _ => {}
                }
            }

            if !positions.contains(&nodes[9]) {
                positions.push(nodes[9]);
            }
        }
    }

    positions.len()
}

#[test]
fn test_1_as_2() {
    let instructions = vec![
        Instruction {
            direction: Direction::Right,
            steps: 4,
        },
        Instruction {
            direction: Direction::Up,
            steps: 4,
        },
        Instruction {
            direction: Direction::Left,
            steps: 3,
        },
        Instruction {
            direction: Direction::Down,
            steps: 1,
        },
        Instruction {
            direction: Direction::Right,
            steps: 4,
        },
        Instruction {
            direction: Direction::Down,
            steps: 1,
        },
        Instruction {
            direction: Direction::Left,
            steps: 5,
        },
        Instruction {
            direction: Direction::Right,
            steps: 2,
        },
    ];

    assert_eq!(part_2(&instructions), 1);
}

#[test]
fn test_part_2() {
    let instructions = vec![
        Instruction {
            direction: Direction::Right,
            steps: 5,
        },
        Instruction {
            direction: Direction::Up,
            steps: 8,
        },
        Instruction {
            direction: Direction::Left,
            steps: 8,
        },
        Instruction {
            direction: Direction::Down,
            steps: 3,
        },
        Instruction {
            direction: Direction::Right,
            steps: 17,
        },
        Instruction {
            direction: Direction::Down,
            steps: 10,
        },
        Instruction {
            direction: Direction::Left,
            steps: 25,
        },
        Instruction {
            direction: Direction::Up,
            steps: 20,
        },
    ];

    assert_eq!(part_2(&instructions), 36);
}
