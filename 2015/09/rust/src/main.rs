use std::{cmp::Reverse, collections::BinaryHeap};

use map::Map;
use node::Node;

mod map;
mod node;

pub fn part_1(map: &Map) -> i32 {
    let mut frontier: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

    for city in map.get_cities() {
        frontier.push(Reverse(Node {
            city: city.clone(),
            remaining: map
                .get_cities()
                .iter()
                .filter(|i| !(*i).eq(city))
                .cloned()
                .collect(),
            dist: 0,
        }));
    }

    while let Some(Reverse(node)) = frontier.pop() {
        if node.remaining.len() == 0 {
            return node.dist;
        }

        for i in 0..node.remaining.len() {
            let city = &node.remaining[i];
            let mut n = node.clone();
            let dist = map.get_distance(city.to_string(), n.city.clone()).unwrap();
            n.remaining.swap_remove(i);
            n.city = city.clone();
            n.dist += dist;
            frontier.push(Reverse(n));
        }
    }

    0
}

pub fn part_2(map: &Map) -> i32 {
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();

    for city in map.get_cities() {
        frontier.push(Node {
            city: city.clone(),
            remaining: map
                .get_cities()
                .iter()
                .filter(|i| !(*i).eq(city))
                .cloned()
                .collect(),
            dist: 0,
        });
    }

    let mut max = 0;

    while let Some(node) = frontier.pop() {
        if node.remaining.len() == 0 {
            max = max.max(node.dist);
        }

        for i in 0..node.remaining.len() {
            let city = &node.remaining[i];
            let mut n = node.clone();
            let dist = map.get_distance(city.to_string(), n.city.clone()).unwrap();
            n.remaining.swap_remove(i);
            n.city = city.clone();
            n.dist += dist;
            frontier.push(n);
        }
    }

    max
}

fn main() {
    let map: Map = include_str!("../../input.txt").into();

    println!("Part 1: {}", part_1(&map));
    println!("Part 2: {}", part_2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_map() -> Map {
        "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141".into()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&build_map()), 605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&build_map()), 982);
    }
}
