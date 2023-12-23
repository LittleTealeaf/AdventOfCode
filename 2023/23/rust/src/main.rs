use std::collections::{HashMap, HashSet};

fn main() {
    // let solution = Solution::new(include_str!("../../sample.txt"));
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Path {
    destination: Point,
    steps: usize,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Node {
    exits: Vec<Path>,
    slope: Option<Path>,
}

struct Solution {
    nodes: HashMap<Point, Node>,
    width: i32,
    height: i32,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut grid: HashMap<Point, char> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, ch)| {
                    (
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        ch,
                    )
                })
            })
            .collect();
        grid.remove(&Point { x: 1, y: 0 });

        let Point {
            x: width,
            y: height,
        } = grid.keys().fold(Point { x: 0, y: 0 }, |a, b| Point {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        });

        // println!("width: {}, height: {}", width, height);

        let mut nodes: HashMap<Point, Node> = grid
            .iter()
            .filter_map(|(p, ch)| {
                (ch != &'#').then_some((
                    *p,
                    Node {
                        exits: [(-1, 0), (1, 0), (0, 1), (0, -1)]
                            .into_iter()
                            .filter_map(|(dx, dy)| {
                                let point = Point {
                                    x: p.x + dx,
                                    y: p.y + dy,
                                };

                                if *grid.get(&point)? != '#' {
                                    Some(Path {
                                        destination: point,
                                        steps: 1,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect(),
                        slope: match ch {
                            'v' => Some(Path {
                                destination: Point { x: p.x, y: p.y + 1 },
                                steps: 1,
                            }),
                            '^' => Some(Path {
                                destination: Point { x: p.x, y: p.y - 1 },
                                steps: 1,
                            }),
                            '<' => Some(Path {
                                destination: Point { x: p.x - 1, y: p.y },
                                steps: 1,
                            }),
                            '>' => Some(Path {
                                destination: Point { x: p.x + 1, y: p.y },
                                steps: 1,
                            }),
                            _ => None,
                        },
                    },
                ))
            })
            .collect();

        // println!("Starting Length: {}", nodes.len());

        'modified: loop {
            let keys = nodes.keys().cloned().collect::<Vec<_>>();
            // println!("Length: {}", keys.len());
            for key in keys {
                if key.y != height && !(key.y == 1 && key.x == 1) {
                    let node = nodes.get(&key).unwrap().clone();
                    if node.slope.is_none() {
                        match node.exits.len() {
                            2 => {
                                // println!("Swap Removing {:?} with neighbors {:?}", key, node.exits);
                                {
                                    let a = nodes.get_mut(&node.exits[0].destination).unwrap();
                                    // println!("\tUpdating {:?}", node.exits[0]);
                                    // println!("\t{:?}", a);
                                    let position =
                                        a.exits.iter().position(|i| i.destination == key).unwrap();
                                    a.exits[position].destination = node.exits[1].destination;
                                    a.exits[position].steps += node.exits[1].steps;
                                    if let Some(slope) = &mut a.slope {
                                        if slope.destination == key {
                                            slope.destination = node.exits[1].destination;
                                            slope.steps += node.exits[1].steps;
                                            // *slope = node.exits[1];
                                        }
                                    }
                                    // println!("\t{:?}", a);
                                }
                                {
                                    let b = nodes.get_mut(&node.exits[1].destination).unwrap();
                                    // println!("\tUpdating {:?}", node.exits[1]);
                                    // println!("\t{:?}", b);
                                    let position =
                                        b.exits.iter().position(|i| i.destination == key).unwrap();
                                    // b.exits[position] = node.exits[0];
                                    b.exits[position].destination = node.exits[0].destination;
                                    b.exits[position].steps += node.exits[0].steps;
                                    if let Some(slope) = &mut b.slope {
                                        if slope.destination == key {
                                            slope.destination = node.exits[0].destination;
                                            slope.steps += node.exits[0].steps;
                                        }
                                    }
                                    // println!("\t{:?}", b);
                                }
                                // println!("Deleted {:?}", nodes.remove(&key));
                                nodes.remove(&key);
                                continue 'modified;
                            }
                            1 => {
                                // println!(
                                //     "Dead-End Removing {:?} with Neighbor {:?}",
                                //     key, node.exits
                                // );
                                let rec = nodes.get_mut(&node.exits[0].destination).unwrap();
                                // println!("\tUpdating {:?}", node.exits[0]);
                                // println!("\t{:?}", rec);
                                rec.exits.retain(|p| p.destination != key);
                                // println!("\t{:?}", rec);
                                // println!("Deleted {:?}", nodes.remove(&key));
                                nodes.remove(&key);
                                continue 'modified;
                            }
                            _ => {}
                        }
                    }
                }
            }

            break;
        }

        // println!("Ending Length: {}", nodes.len());

        Self {
            nodes,
            width,
            height,
        }
    }
}

#[derive(Clone, Debug)]
struct SearchNode {
    visited: HashSet<Point>,
    steps: usize,
    current: Point,
}

impl Solution {
    fn part_1(&self) -> usize {
        let mut frontier = vec![SearchNode {
            visited: HashSet::new(),
            current: Point { y: 1, x: 1 },
            steps: 1,
        }];

        let mut max_length = 0;

        while let Some(state) = frontier.pop() {
            if state.current.y == self.height {
                max_length = max_length.max(state.steps);
                continue;
            }
            let node = self.nodes.get(&state.current).unwrap();

            if let Some(path) = node.slope {
                if !state.visited.contains(&path.destination) {
                    let mut state = state.clone();
                    state.visited.insert(state.current);
                    state.steps += path.steps;
                    state.current = path.destination;
                    frontier.push(state);
                }
            } else {
                frontier.extend(
                    node.exits
                        .iter()
                        .filter(|&path| !state.visited.contains(&path.destination))
                        .map(|path| {
                            let mut state = state.clone();
                            state.visited.insert(state.current);
                            state.steps += path.steps;
                            state.current = path.destination;
                            state
                        }),
                )
            }
        }

        max_length
    }

    fn part_2(&self) -> usize {
        let mut frontier = vec![SearchNode {
            visited: HashSet::new(),
            current: Point { y: 1, x: 1 },
            steps: 1,
        }];

        let mut max_length = 0;

        while let Some(state) = frontier.pop() {
            if state.current.y == self.height {
                max_length = max_length.max(state.steps);
                continue;
            }
            let node = self.nodes.get(&state.current).unwrap();

            frontier.extend(
                node.exits
                    .iter()
                    .filter(|&path| !state.visited.contains(&path.destination))
                    .map(|path| {
                        let mut state = state.clone();
                        state.visited.insert(state.current);
                        state.steps += path.steps;
                        state.current = path.destination;
                        state
                    }),
            );
        }

        max_length
    }
}
