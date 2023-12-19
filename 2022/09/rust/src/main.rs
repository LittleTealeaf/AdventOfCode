use std::collections::HashSet;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
struct Pos {
    x: i32,
    y: i32,
}

struct Solution {
    instructions: Vec<Instruction>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            instructions: input
                .lines()
                .map(|line| {
                    let mut tokens = line.trim().split(' ');
                    let direction = match tokens.next().unwrap() {
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        _ => panic!(),
                    };
                    let steps = tokens.next().unwrap().parse::<i32>().unwrap();
                    Instruction { direction, steps }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let mut head = Pos::default();
        let mut tail = Pos::default();

        let mut positions = HashSet::from([Pos::default()]);

        for instruction in &self.instructions {
            for _ in 0..instruction.steps {
                match instruction.direction {
                    Direction::Up => {
                        head.y += 1;
                    }
                    Direction::Down => {
                        head.y -= 1;
                    }
                    Direction::Left => {
                        head.x -= 1;
                    }
                    Direction::Right => {
                        head.x += 1;
                    }
                }

                let x_off = head.x.abs_diff(tail.x) > 1;
                let y_off = head.y.abs_diff(tail.y) > 1;
                match (x_off, y_off) {
                    (true, false) => {
                        tail.x = (tail.x + head.x) / 2;
                        tail.y = head.y;
                    }
                    (false, true) => {
                        tail.y = (tail.y + head.y) / 2;
                        tail.x = head.x;
                    }
                    (true, true) => {
                        tail.x = (tail.x + head.x) / 2;
                        tail.y = (tail.y + head.y) / 2;
                    }
                    _ => {}
                }

                positions.insert(tail);
            }
        }

        positions.len()
    }

    fn part_2(&self) -> usize {
        let mut nodes = [Pos { x: 0, y: 0 }; 10];

        let mut positions = vec![Pos { x: 0, y: 0 }];

        for instr in &self.instructions {
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
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample_1.txt"));
    assert_eq!(solution.part_1(), 13);
}

#[test]
fn test_part_2_sample_1() {
    let solution = Solution::new(include_str!("../../sample_1.txt"));
    assert_eq!(solution.part_2(), 1);
}

#[test]
fn test_part_2_sample_2() {
    let solution = Solution::new(include_str!("../../sample_2.txt"));
    assert_eq!(solution.part_2(), 36);
}
