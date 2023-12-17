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

        let mut frontier: BinaryHeap<Reverse<Node>> = [(0, 1), (1, 0)]
            .into_iter()
            .map(|(x, y)| {
                Reverse(Node {
                    x,
                    y,
                    dx: x as i32,
                    dy: y as i32,
                    cost: self.map[y][x],
                    consecutive: 1,
                    heuristic: self.map[y][x] + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                })
            })
            .collect();

        while let Some(Reverse(n)) = frontier.pop() {
            let visit = n.to_visited();
            if visited.contains(&visit) {
                continue;
            }

            if n.x == goal_x && n.y == goal_y {
                return n.cost;
            }

            visited.insert(visit);

            [
                (n.consecutive < 3).then_some((n.dx, n.dy, true)),
                Some((n.dy, n.dx, false)),
                Some((-n.dy, -n.dx, false)),
            ]
            .into_iter()
            .flatten()
            .filter(|(dx, dy, _)| self.in_bounds(n.x, n.y, *dx, *dy))
            .for_each(|(dx, dy, inc)| {
                let x = (n.x as i32 + dx) as usize;
                let y = (n.y as i32 + dy) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx,
                    dy,
                    cost: n.cost + self.map[y][x],
                    consecutive: if inc { n.consecutive + 1 } else { 1 },
                    heuristic: n.cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }))
            });
        }

        0
    }

    fn part_2(&self) -> u32 {
        let goal_x = self.map[0].len() - 1;
        let goal_y = self.map.len() - 1;

        let mut visited = HashSet::new();

        let mut frontier: BinaryHeap<Reverse<Node>> = [(0, 1), (1, 0)]
            .into_iter()
            .map(|(x, y)| {
                Reverse(Node {
                    x,
                    y,
                    dx: x as i32,
                    dy: y as i32,
                    cost: self.map[y][x],
                    consecutive: 1,
                    heuristic: self.map[y][x] + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                })
            })
            .collect();

        while let Some(Reverse(n)) = frontier.pop() {
            let visit = n.to_visited();
            if visited.contains(&visit) {
                continue;
            }

            if n.x == goal_x && n.y == goal_y {
                return n.cost;
            }

            visited.insert(visit);

            [
                (n.consecutive < 10).then_some((n.dx, n.dy, true)),
                (n.consecutive >= 4).then_some((n.dy, n.dx, false)),
                (n.consecutive >= 4).then_some((-n.dy, -n.dx, false)),
            ]
            .into_iter()
            .flatten()
            .filter(|(dx, dy, _)| self.in_bounds(n.x, n.y, *dx, *dy))
            .for_each(|(dx, dy, inc)| {
                let x = (n.x as i32 + dx) as usize;
                let y = (n.y as i32 + dy) as usize;
                frontier.push(Reverse(Node {
                    x,
                    y,
                    dx,
                    dy,
                    cost: n.cost + self.map[y][x],
                    consecutive: if inc { n.consecutive + 1 } else { 1 },
                    heuristic: n.cost
                        + self.map[y][x]
                        + (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                }))
            });
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
