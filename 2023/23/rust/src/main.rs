use std::collections::{HashMap, HashSet};

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope { dx: i32, dy: i32 },
}

struct Solution {
    map: HashMap<Point, Tile>,
}

#[derive(Clone)]
struct Node {
    visited: HashSet<Point>,
    current: Point,
}

impl Node {
    fn move_to(&self, point: Point) -> Self {
        let mut node = self.clone();
        node.visited.insert(node.current);
        node.current = point;
        node
    }
}
const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().map(move |(x, ch)| {
                        (
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            match ch {
                                '#' => Tile::Forest,
                                '.' => Tile::Path,
                                '>' => Tile::Slope { dx: 1, dy: 0 },
                                '<' => Tile::Slope { dx: -1, dy: 0 },
                                '^' => Tile::Slope { dx: 0, dy: -1 },
                                'v' => Tile::Slope { dx: 0, dy: 1 },
                                _ => panic!(),
                            },
                        )
                    })
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let mut frontier = vec![Node {
            visited: HashSet::from([Point { x: 1, y: 0 }]),
            current: Point { y: 1, x: 1 },
        }];
        let mut max_length = 0;

        let goal_y = self.map.keys().map(|p| p.y).max().unwrap();

        while let Some(state) = frontier.pop() {
            if state.current.y == goal_y {
                max_length = max_length.max(state.visited.len());
                continue;
            }

            match self.map.get(&state.current) {
                Some(Tile::Path) => frontier.extend(
                    OFFSETS
                        .into_iter()
                        .map(|(dx, dy)| Point {
                            x: state.current.x + dx,
                            y: state.current.y + dy,
                        })
                        .filter(|point| {
                            !state.visited.contains(point)
                                && !matches!(self.map.get(point).unwrap(), Tile::Forest)
                        })
                        .map(|p| state.move_to(p)),
                ),

                Some(Tile::Slope { dx, dy }) => {
                    let next = Point {
                        x: state.current.x + dx,
                        y: state.current.y + dy,
                    };
                    if !state.visited.contains(&next) {
                        frontier.push(state.move_to(next));
                    }
                }
                _ => {}
            }
        }

        max_length
    }

    fn part_2(&self) -> usize {
        let mut frontier = vec![Node {
            visited: HashSet::from([Point { x: 1, y: 0 }]),
            current: Point { y: 1, x: 1 },
        }];
        let mut max_length = 0;

        let goal_y = self.map.keys().map(|p| p.y).max().unwrap();

        while let Some(state) = frontier.pop() {
            if state.current.y == goal_y {
                max_length = max_length.max(state.visited.len());
                continue;
            }

            match self.map.get(&state.current) {
                Some(Tile::Path) | Some(Tile::Slope { dx: _, dy: _ }) => frontier.extend(
                    OFFSETS
                        .into_iter()
                        .map(|(dx, dy)| Point {
                            x: state.current.x + dx,
                            y: state.current.y + dy,
                        })
                        .filter(|point| {
                            !matches!(self.map.get(point).unwrap(), Tile::Forest)
                                && !state.visited.contains(point)
                        })
                        .map(|p| state.move_to(p)),
                ),
                _ => {}
            }
        }

        max_length
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 94);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 154);
}
