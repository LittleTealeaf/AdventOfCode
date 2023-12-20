use std::collections::{hash_map::RandomState, HashSet};

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Claim {
    id: usize,
    position: Point,
    dimension: Point,
}

struct Solution {
    claims: Vec<Claim>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            claims: input
                .lines()
                .map(|line| {
                    let mut tokens = line[1..].split(' ');
                    let id = tokens.next().unwrap().parse().unwrap();
                    let _ = tokens.next();
                    let position = {
                        let string = tokens.next().unwrap();
                        let (x_str, y_str) = &string[..string.len() - 1].split_once(',').unwrap();
                        Point {
                            x: x_str.parse().unwrap(),
                            y: y_str.parse().unwrap(),
                        }
                    };
                    let dimension = {
                        let (x_str, y_str) = tokens.next().unwrap().split_once('x').unwrap();
                        Point {
                            x: x_str.parse().unwrap(),
                            y: y_str.parse().unwrap(),
                        }
                    };
                    Claim {
                        id,
                        position,
                        dimension,
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let mut set = HashSet::new();

        HashSet::<Point, RandomState>::from_iter(
            self.claims
                .iter()
                .flat_map(|claim| {
                    (0..claim.dimension.x).flat_map(move |dx| {
                        (0..claim.dimension.y).map(move |dy| Point {
                            x: dx + claim.position.x,
                            y: dy + claim.position.y,
                        })
                    })
                })
                .filter(|point| !set.insert(*point)),
        )
        .len()
    }

    fn part_2(&self) -> usize {
        let mut set = HashSet::new();

        let multi_set = HashSet::<Point, RandomState>::from_iter(
            self.claims
                .iter()
                .flat_map(|claim| {
                    (0..claim.dimension.x).flat_map(move |dx| {
                        (0..claim.dimension.y).map(move |dy| Point {
                            x: dx + claim.position.x,
                            y: dy + claim.position.y,
                        })
                    })
                })
                .filter(|point| !set.insert(*point)),
        );

        self.claims
            .iter()
            .find_map(|claim| {
                (0..claim.dimension.x)
                    .flat_map(move |dx| {
                        (0..claim.dimension.y).map(move |dy| Point {
                            x: dx + claim.position.x,
                            y: dy + claim.position.y,
                        })
                    })
                    .all(|point| !multi_set.contains(&point))
                    .then_some(claim.id)
            })
            .unwrap()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 4);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 3);
}
