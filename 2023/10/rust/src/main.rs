use std::collections::HashMap;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));

    println!("Part 1: {}", solution.part_1());
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Start,
    Ground,
    Pipe {
        east: bool,
        west: bool,
        north: bool,
        south: bool,
    },
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            '|' => Ok(Self::Pipe {
                east: false,
                west: false,
                north: true,
                south: true,
            }),
            '-' => Ok(Self::Pipe {
                east: true,
                west: true,
                north: false,
                south: false,
            }),
            'L' => Ok(Self::Pipe {
                east: true,
                west: false,
                north: true,
                south: false,
            }),
            'J' => Ok(Self::Pipe {
                east: false,
                west: true,
                north: true,
                south: false,
            }),
            '7' => Ok(Self::Pipe {
                east: false,
                west: true,
                north: false,
                south: true,
            }),
            'F' => Ok(Self::Pipe {
                east: true,
                west: false,
                north: false,
                south: true,
            }),
            _ => Err(()),
        }
    }
}

struct Solution {
    map: Vec<Vec<Tile>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .map(|line| line.chars().map(|c| Tile::try_from(c).unwrap()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let (s_x, s_y) = self
            .map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|tile| matches!(tile, Tile::Start))
                    .map(|x| (x, y))
            })
            .unwrap();

        #[derive(Debug)]
        struct Node {
            coord: Coord,
            steps: usize,
        }

        let mut map: HashMap<Coord, usize> = HashMap::new();

        #[allow(clippy::unnecessary_lazy_evaluations)]
        let mut stack = [
            (s_x > 0 && matches!(self.map[s_y][s_x - 1], Tile::Pipe { east: true, .. }))
                .then(|| Coord { x: s_x - 1, y: s_y }),
            (s_y > 0 && matches!(self.map[s_y - 1][s_x], Tile::Pipe { north: true, .. }))
                .then(|| Coord { x: s_x, y: s_y - 1 }),
            (s_x < self.map[0].len() - 1
                && matches!(self.map[s_y][s_x + 1], Tile::Pipe { west: true, .. }))
            .then(|| Coord { x: s_x + 1, y: s_y }),
            (s_y < self.map.len() - 1
                && matches!(self.map[s_y + 1][s_x], Tile::Pipe { south: true, .. }))
            .then(|| Coord { x: s_x, y: s_y + 1 }),
        ]
        .into_iter()
        .flatten()
        .map(|coord| Node { coord, steps: 1 })
        .collect::<Vec<_>>();

        while let Some(node) = stack.pop() {
            if map
                .get(&node.coord)
                .map(|a| a <= &node.steps)
                .unwrap_or(false)
            {
                continue;
            }

            map.insert(node.coord, node.steps);

            if let Tile::Pipe {
                east,
                west,
                north,
                south,
            } = self.map[node.coord.y][node.coord.x]
            {
                if east {
                    stack.push(Node {
                        coord: Coord {
                            x: node.coord.x + 1,
                            y: node.coord.y,
                        },
                        steps: node.steps + 1,
                    })
                }
                if west {
                    stack.push(Node {
                        coord: Coord {
                            x: node.coord.x - 1,
                            y: node.coord.y,
                        },
                        steps: node.steps + 1,
                    })
                }
                if north {
                    stack.push(Node {
                        coord: Coord {
                            x: node.coord.x,
                            y: node.coord.y - 1,
                        },
                        steps: node.steps + 1,
                    })
                }
                if south {
                    stack.push(Node {
                        coord: Coord {
                            x: node.coord.x,
                            y: node.coord.y + 1,
                        },
                        steps: node.steps + 1,
                    })
                }
            }
        }

        *map.values().max().unwrap()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 8);
}
