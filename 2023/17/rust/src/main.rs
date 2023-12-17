use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    map: Vec<Vec<u32>>,
}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    cost: u32,
    heuristic: u32,
    consecutive: usize,
    dx: i32,
    dy: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

impl Node {
    fn to_visited(self) -> (usize, usize, usize, i32, i32) {
        (self.x, self.y, self.consecutive, self.dx, self.dy)
    }
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }

    fn in_bounds(&self, x: usize, y: usize, dx: i32, dy: i32) -> bool {
        (dx == 0 || (dx == -1 && x > 0) || (dx == 1 && x < self.map[0].len() - 1))
            && (dy == 0 || (dy == -1 && y > 0) || (dy == 1 && y < self.map.len() - 1))
    }

    fn part_1(&self) -> u32 {
        let goal_x = self.map[0].len() - 1;
        let goal_y = self.map.len() - 1;

        let mut visited = HashSet::new();

        let mut frontier: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        frontier.push(Reverse(Node {
            x: 0,
            y: 1,
            cost: self.map[1][0],
            consecutive: 1,
            heuristic: self.map[1][0] + (goal_x as u32 + goal_y as u32 - 1),
            dx: 0,
            dy: 1,
        }));
        frontier.push(Reverse(Node {
            x: 1,
            y: 0,
            cost: self.map[0][1],
            consecutive: 1,
            heuristic: self.map[0][1] + (goal_x as u32 + goal_y as u32 - 1),
            dx: 1,
            dy: 0,
        }));

        while let Some(Reverse(node)) = frontier.pop() {
            let visit = node.to_visited();
            if visited.contains(&visit) {
                continue;
            }

            visited.insert(visit);

            if node.x == goal_x && node.y == goal_y {
                return node.cost;
            }

            let Node {
                x,
                y,
                cost,
                heuristic: _,
                dx,
                dy,
                consecutive,
            } = node;

            if node.consecutive < 3 && self.in_bounds(x, y, dx, dy) {
                let x = (node.x as i32 + node.dx) as usize;
                let y = (node.y as i32 + node.dy) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx,
                    dy,
                    cost: cost + self.map[y][x],
                    consecutive: consecutive + 1,
                    heuristic: cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }))
            }

            if self.in_bounds(x, y, dy, dx) {
                let x = (x as i32 + dy) as usize;
                let y = (y as i32 + dx) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx: dy,
                    dy: dx,
                    cost: cost + self.map[y][x],
                    consecutive: 1,
                    heuristic: cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }));
            }

            if self.in_bounds(x, y, -dy, -dx) {
                let x = (x as i32 - dy) as usize;
                let y = (y as i32 - dx) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx: -dy,
                    dy: -dx,
                    cost: cost + self.map[y][x],
                    consecutive: 1,
                    heuristic: cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }));
            }
        }

        0
    }

    fn part_2(&self) -> u32 {
        let goal_x = self.map[0].len() - 1;
        let goal_y = self.map.len() - 1;

        let mut visited = HashSet::new();

        let mut frontier: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        frontier.push(Reverse(Node {
            x: 0,
            y: 1,
            cost: self.map[1][0],
            consecutive: 1,
            heuristic: self.map[1][0] + (goal_x as u32 + goal_y as u32 - 1),
            dx: 0,
            dy: 1,
        }));
        frontier.push(Reverse(Node {
            x: 1,
            y: 0,
            cost: self.map[0][1],
            consecutive: 1,
            heuristic: self.map[0][1] + (goal_x as u32 + goal_y as u32 - 1),
            dx: 1,
            dy: 0,
        }));

        while let Some(Reverse(node)) = frontier.pop() {
            let visit = node.to_visited();
            if visited.contains(&visit) {
                continue;
            }

            visited.insert(visit);

            if node.x == goal_x && node.y == goal_y {
                return node.cost;
            }

            let Node {
                x,
                y,
                cost,
                heuristic: _,
                dx,
                dy,
                consecutive,
            } = node;

            if node.consecutive < 10 && self.in_bounds(x, y, dx, dy) {
                let x = (node.x as i32 + node.dx) as usize;
                let y = (node.y as i32 + node.dy) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx,
                    dy,
                    cost: cost + self.map[y][x],
                    consecutive: consecutive + 1,
                    heuristic: cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }))
            }

            if consecutive >= 4 && self.in_bounds(x, y, dy, dx) {
                let x = (x as i32 + dy) as usize;
                let y = (y as i32 + dx) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx: dy,
                    dy: dx,
                    cost: cost + self.map[y][x],
                    consecutive: 1,
                    heuristic: cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }));
            }

            if consecutive >= 4 && self.in_bounds(x, y, -dy, -dx) {
                let x = (x as i32 - dy) as usize;
                let y = (y as i32 - dx) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx: -dy,
                    dy: -dx,
                    cost: cost + self.map[y][x],
                    consecutive: 1,
                    heuristic: cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }));
            }
        }

        0
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 102);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 94);
}
