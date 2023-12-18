use std::collections::HashSet;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn offset(&self) -> (i64, i64) {
        match self {
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
            Self::Up => (0, 1),
            Self::Down => (0, -1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct DigPlan {
    direction: Direction,
    steps: usize,
    color: String,
    color_steps: u64,
    color_direction: Direction,
}

struct Solution {
    plan: Vec<DigPlan>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Trench {
    max: Pos,
    min: Pos,
}

impl Trench {
    fn new(a: Pos, b: Pos) -> Self {
        Self {
            max: Pos {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
            min: Pos {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
        }
    }

    fn contains(&self, pos: &Pos) -> bool {
        pos.x <= self.max.x && pos.x >= self.min.x && pos.y >= self.min.y && pos.y <= self.max.y
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.min.x >= other.min.x
            && self.min.x <= other.max.x
            && self.min.y >= other.min.y
            && self.min.y <= other.max.y
            || self.max.x >= other.min.x
                && self.max.x <= other.max.x
                && self.max.y >= other.min.y
                && self.max.y <= other.max.y
            || other.min.x >= self.min.x
                && other.min.x <= self.max.x
                && other.min.y >= self.min.y
                && other.min.y <= self.max.y
            || other.max.x >= self.min.x
                && other.max.x <= self.max.x
                && other.max.y >= self.min.y
                && other.max.y <= self.max.y
    }

    fn iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        (self.min.x..=self.max.x).flat_map(|x| (self.min.y..=self.max.y).map(move |y| Pos { x, y }))
    }

    fn edge_iter_pos(&self) -> impl Iterator<Item = Pos> + '_ {
        (self.min.x + 1..self.max.x)
            .flat_map(|x| [Pos { x, y: self.min.y }, Pos { x, y: self.max.y }])
            .chain(
                (self.min.y..=self.max.y)
                    .flat_map(|y| [Pos { y, x: self.min.x }, Pos { y, x: self.max.x }]),
            )
    }
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            plan: input
                .lines()
                .map(|line| {
                    let mut tokens = line.split(' ');
                    let direction = match tokens.next().unwrap() {
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        _ => panic!(),
                    };
                    let steps = tokens.next().unwrap().parse::<usize>().unwrap();
                    let color = tokens.next().unwrap()[2..8].to_string();
                    let color_steps = u64::from_str_radix(&color[0..5], 16).unwrap();
                    let color_direction = match &color[5..] {
                        "0" => Direction::Right,
                        "1" => Direction::Down,
                        "2" => Direction::Left,
                        "3" => Direction::Up,
                        _ => panic!(),
                    };
                    DigPlan {
                        direction,
                        color,
                        steps,
                        color_steps,
                        color_direction,
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let mut position = Pos { x: 0, y: 0 };

        let mut filled: HashSet<Pos> = [position].into_iter().collect();

        for instr in &self.plan {
            let (dx, dy) = instr.direction.offset();
            filled.extend((0..instr.steps).map(|i| Pos {
                x: position.x + dx * i as i64,
                y: position.y + dy * i as i64,
            }));

            position.x += dx * instr.steps as i64;
            position.y += dy * instr.steps as i64;
        }
        let mut y_max = i64::MIN;
        let mut y_min = i64::MAX;

        let mut x_max = i64::MIN;
        let mut x_min = i64::MAX;

        for Pos { x, y } in &filled {
            x_max = x_max.max(*x);
            x_min = x_min.min(*x);
            y_max = y_max.max(*y);
            y_min = y_min.min(*y);
        }

        let mut frontier = filled
            .iter()
            .flat_map(|Pos { x, y }| {
                [
                    (*x == x_min).then_some(Pos { x: x + 1, y: *y }),
                    (*x == x_max).then_some(Pos { x: x - 1, y: *y }),
                    (*y == y_min).then_some(Pos { x: *x, y: y + 1 }),
                    (*y == y_max).then_some(Pos { x: *x, y: y - 1 }),
                ]
            })
            .flatten()
            .collect::<Vec<_>>();

        while let Some(coord) = frontier.pop() {
            if filled.insert(coord) {
                frontier.extend(
                    [(-1, 0), (1, 0), (0, 1), (0, -1)]
                        .into_iter()
                        .map(|(dx, dy)| Pos {
                            x: coord.x + dx,
                            y: coord.y + dy,
                        }),
                );
            }
        }

        filled.len()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 62);
}
