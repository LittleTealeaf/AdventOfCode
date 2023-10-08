use std::{cmp::Reverse, collections::BinaryHeap, thread, time::Duration};

use map::{Map, Point};

mod map;

fn main() {
    println!("Part 1: {}", part_1(Map::load_input()));
    println!("Part 2: {}", part_2(Map::load_input()));
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    point: Point,
    cost: usize,
    heuristic: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heuristic.partial_cmp(&other.heuristic)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

fn part_1(map: Map) -> usize {
    let mut heap = BinaryHeap::<Reverse<Node>>::new();

    let mut visited = Vec::new();

    heap.push(Reverse(Node {
        point: map.start.clone(),
        cost: 0,
        heuristic: map.dist_to_end(map.start),
    }));

    while let Some(Reverse(Node { point, cost, .. })) = heap.pop() {
        if point.eq(&map.end) {
            return cost;
        }

        if visited.contains(&point) {
            continue;
        }

        visited.push(point);

        let current_height = map.get(point).unwrap();

        let points = [
            (point.x > 0).then(|| Point {
                x: point.x - 1,
                y: point.y,
            }),
            (point.y > 0).then(|| Point {
                x: point.x,
                y: point.y - 1,
            }),
            Some(Point {
                x: point.x + 1,
                y: point.y,
            }),
            Some(Point {
                x: point.x,
                y: point.y + 1,
            }),
        ]
        .into_iter()
        .flatten()
        .filter_map(|p| map.get(p).map(|height| (p, height)));

        for (point, height) in points {
            if height <= current_height + 1 {
                heap.push(Reverse(Node {
                    point,
                    cost: cost + 1,
                    heuristic: cost + 1 + map.dist_to_end(point),
                }));
            }
        }
    }

    0
}

fn part_2(map: Map) -> usize {
    let mut heap = BinaryHeap::<Reverse<Node>>::new();

    let mut visited = Vec::new();

    for (y, row) in map.board.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if height == &0 {
                heap.push(Reverse(Node {
                    point: Point { x, y },
                    cost: 0,
                    heuristic: map.dist_to_end(Point { x, y }),
                }))
            }
        }
    }

    while let Some(Reverse(Node { point, cost, .. })) = heap.pop() {
        if point.eq(&map.end) {
            return cost;
        }

        if visited.contains(&point) {
            continue;
        }

        visited.push(point);

        let current_height = map.get(point).unwrap();

        let points = [
            (point.x > 0).then(|| Point {
                x: point.x - 1,
                y: point.y,
            }),
            (point.y > 0).then(|| Point {
                x: point.x,
                y: point.y - 1,
            }),
            Some(Point {
                x: point.x + 1,
                y: point.y,
            }),
            Some(Point {
                x: point.x,
                y: point.y + 1,
            }),
        ]
        .into_iter()
        .flatten()
        .filter_map(|p| map.get(p).map(|height| (p, height)));

        for (point, height) in points {
            if height <= current_height + 1 {
                heap.push(Reverse(Node {
                    point,
                    cost: cost + 1,
                    heuristic: cost + 1 + map.dist_to_end(point),
                }));
            }
        }
    }

    0
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(Map::load_test()), 29)
}
