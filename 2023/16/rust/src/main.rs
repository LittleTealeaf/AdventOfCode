use std::collections::HashSet;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    map: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Down,
    Up,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Beam {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Beam {
    fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Beam {
                x: self.x,
                y: self.y - 1,
                direction,
            },
            Direction::Down => Beam {
                x: self.x,
                y: self.y + 1,
                direction,
            },
            Direction::Left => Beam {
                x: self.x - 1,
                y: self.y,
                direction,
            },
            Direction::Right => Beam {
                x: self.x + 1,
                y: self.y,
                direction,
            },
        }
    }
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn solve(&self, start: Beam) -> usize {
        let mut energized = HashSet::new();
        let h = self.map.len();
        let w = self.map[0].len();

        let mut visited = HashSet::new();

        let mut frontier = vec![start];

        while let Some(beam) = frontier.pop() {
            if beam.x < 0 || beam.x >= w as i32 || beam.y < 0 || beam.y >= h as i32 {
                continue;
            }

            if visited.contains(&beam) {
                continue;
            }

            visited.insert(beam);

            energized.insert((beam.x, beam.y));
            // energized[beam.y as usize][beam.x as usize] = true;

            match self.map[beam.y as usize][beam.x as usize] {
                '.' => frontier.push(beam.go(beam.direction)),
                '/' => frontier.push(beam.go(match beam.direction {
                    Direction::Down => Direction::Left,
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                })),
                '\\' => frontier.push(beam.go(match beam.direction {
                    Direction::Down => Direction::Right,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                })),
                '-' => frontier.append(&mut match beam.direction {
                    Direction::Down | Direction::Up => {
                        vec![beam.go(Direction::Right), beam.go(Direction::Left)]
                    }
                    direction => vec![beam.go(direction)],
                }),
                '|' => frontier.append(&mut match beam.direction {
                    Direction::Right | Direction::Left => {
                        vec![beam.go(Direction::Up), beam.go(Direction::Down)]
                    }
                    direction => vec![beam.go(direction)],
                }),
                _ => panic!(),
            }
        }

        energized.len()
    }

    fn part_1(&self) -> usize {
        self.solve(Beam {
            x: 0,
            y: 0,
            direction: Direction::Right,
        })
    }

    fn part_2(&self) -> usize {
        let h = self.map.len();
        let w = self.map[0].len();

        (0..w)
            .map(|x| {
                [
                    Beam {
                        x: x as i32,
                        y: 0,
                        direction: Direction::Down,
                    },
                    Beam {
                        x: x as i32,
                        y: h as i32 - 1,
                        direction: Direction::Up,
                    },
                ]
            })
            .chain((0..h).map(|y| {
                [
                    Beam {
                        y: y as i32,
                        x: 0,
                        direction: Direction::Right,
                    },
                    Beam {
                        y: y as i32,
                        x: w as i32 - 1,
                        direction: Direction::Left,
                    },
                ]
            }))
            .flatten()
            .map(|start| self.solve(start))
            .max()
            .unwrap()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 46);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 51);
}
