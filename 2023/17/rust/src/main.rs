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
        (self.heuristic + self.cost).cmp(&(other.heuristic + other.cost))
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

    fn astar<F, I>(&self, action_map: F) -> u32
    where
        F: Fn(Node) -> I,
        I: Iterator<Item = (i32, i32, bool)>,
    {
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
                    heuristic: (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                })
            })
            .collect();

        while let Some(Reverse(n)) = frontier.pop() {
            if !visited.insert((n.x, n.y, n.consecutive, n.dx, n.dy)) {
                continue;
            }

            if n.x == goal_x && n.y == goal_y {
                return n.cost;
            }

            frontier.extend(
                action_map(n)
                    .filter(|(dx, dy, _)| {
                        (dx == &0 || (dx == &-1 && n.x > 0) || (dx == &1 && n.x < goal_x))
                            && (dy == &0 || (dy == &-1 && n.y > 0) || (dy == &1 && n.y < goal_y))
                    })
                    .map(|(dx, dy, inc)| {
                        let x = (n.x as i32 + dx) as usize;
                        let y = (n.y as i32 + dy) as usize;
                        Reverse(Node {
                            x,
                            y,
                            dx,
                            dy,
                            cost: n.cost + self.map[y][x],
                            consecutive: if inc { n.consecutive + 1 } else { 1 },
                            heuristic: (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32,
                        })
                    }),
            );
        }
        panic!()
    }

    fn part_1(&self) -> u32 {
        self.astar(|n| {
            [
                (n.consecutive < 3).then_some((n.dx, n.dy, true)),
                Some((n.dy, n.dx, false)),
                Some((-n.dy, -n.dx, false)),
            ]
            .into_iter()
            .flatten()
        })
    }

    fn part_2(&self) -> u32 {
        self.astar(|n| {
            [
                (n.consecutive < 10).then_some((n.dx, n.dy, true)),
                (n.consecutive >= 4).then_some((n.dy, n.dx, false)),
                (n.consecutive >= 4).then_some((-n.dy, -n.dx, false)),
            ]
            .into_iter()
            .flatten()
        })
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
