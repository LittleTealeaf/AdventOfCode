use std::collections::{hash_map::RandomState, HashSet};

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    // println!("Part 2: {}", solution.part_2());
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Pos<T> {
    x: T,
    y: T,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Garden,
    Start,
    Rock,
}

struct Solution {
    map: Vec<Vec<Tile>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Tile::Rock,
                            '.' => Tile::Garden,
                            'S' => Tile::Start,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn part_1(&self) -> usize {
        let mut positions = HashSet::from([self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, tile)| (Pos { x, y }, tile))
            })
            .find_map(|(pos, tile)| matches!(tile, Tile::Start).then_some(pos))
            .unwrap()]);

        for _ in 0..64 {
            positions = HashSet::<Pos<usize>, RandomState>::from_iter(
                positions
                    .into_iter()
                    .filter(|pos| !matches!(self.map[pos.y][pos.x], Tile::Rock))
                    .flat_map(|pos| {
                        [
                            (pos.x > 0).then(|| Pos {
                                x: pos.x - 1,
                                y: pos.y,
                            }),
                            (pos.y > 0).then(|| Pos {
                                x: pos.x,
                                y: pos.y - 1,
                            }),
                            (pos.x < self.map[0].len() - 1).then(|| Pos {
                                x: pos.x + 1,
                                y: pos.y,
                            }),
                            (pos.y < self.map.len() - 1).then(|| Pos {
                                x: pos.x,
                                y: pos.y + 1,
                            }),
                        ]
                    })
                    .flatten(),
            );
        }

        positions
            .into_iter()
            .filter(|pos| !matches!(self.map[pos.y][pos.x], Tile::Rock))
            .count()
    }


    // TODO: Create a search algorithm that keeps track of the visited steps (except ones that have
    // all steps around it visited), and each time you visit a new state add it to one of two
    // alternating counts
    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn part_2(&self) -> usize {
        let width = self.map[0].len();
        let width_i64 = width as i64;
        let height = self.map[0].len();
        let height_i64 = height as i64;
        let mut positions = HashSet::from([self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(x, tile)| {
                    (
                        Pos {
                            x: x as i64,
                            y: y as i64,
                        },
                        tile,
                    )
                })
            })
            .find_map(|(pos, tile)| matches!(tile, Tile::Start).then_some(pos))
            .unwrap()]);

        for _ in 0..26_501_365 {
            positions = HashSet::<Pos<i64>, RandomState>::from_iter(
                positions
                    .into_iter()
                    .filter(|pos| {
                        !matches!(
                            self.map[((pos.y % height_i64 + height_i64) % height_i64) as usize]
                                [((pos.x % width_i64 + width_i64) % width_i64) as usize],
                            Tile::Rock
                        )
                    })
                    .flat_map(|pos| {
                        [(-1, 0), (1, 0), (0, -1), (0, 1)]
                            .into_iter()
                            .map(move |(dx, dy)| Pos {
                                x: pos.x + dx,
                                y: pos.y + dy,
                            })
                    }),
            )
        }

        positions
            .into_iter()
            .filter(|pos| {
                !matches!(
                    self.map[((pos.y % height_i64 + height_i64) % height_i64) as usize]
                        [((pos.x % width_i64 + width_i64) % width_i64) as usize],
                    Tile::Rock
                )
            })
            .count()
    }
}
