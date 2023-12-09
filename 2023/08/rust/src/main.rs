use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt")).unwrap();
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    instructions: Vec<usize>,
    map: HashMap<String, [String; 2]>,
}

impl Solution {
    fn new(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        let instructions = lines
            .next()?
            .trim()
            .chars()
            .map(|c| if c == 'L' { 0 } else { 1 })
            .collect();
        let _ = lines.next();

        let map = lines
            .map(|line| {
                let source = &line[0..3];
                let left = &line[7..10];
                let right = &line[12..15];

                (source.to_string(), [left.to_string(), right.to_string()])
            })
            .collect();

        Some(Self { instructions, map })
    }

    fn part_1(&self) -> usize {
        let mut steps = 0;
        let mut location = "AAA".to_string();
        while location != "ZZZ" {
            let direction = self.instructions[steps % self.instructions.len()];
            location = self
                .map
                .get(&location)
                .unwrap()
                .get(direction)
                .unwrap()
                .to_string();
            steps += 1;
        }
        steps
    }

    fn part_2(&self) -> usize {
        #[derive(Debug)]
        struct Loop {
            start: usize,
            length: usize,
            content: Vec<usize>,
        }

        self.map
            .keys()
            .filter(|n| n.ends_with('A'))
            .cloned()
            .map(|n| {
                let mut path = vec![n];
                loop {
                    let direction = self.instructions[(path.len() - 1) % self.instructions.len()];
                    let position = path.last().unwrap();
                    let next = self
                        .map
                        .get(position)
                        .unwrap()
                        .get(direction)
                        .unwrap()
                        .to_string();
                    if let Some(index) = path.iter().position(|i| i == &next) {
                        if path.len() % self.instructions.len() == index % self.instructions.len() {
                            return Loop {
                                start: index,
                                length: path.len() - index,
                                content: path
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(i, v)| v.ends_with('Z').then_some(i))
                                    .collect(),
                            };
                        } else {
                            path.push(next);
                        }
                    } else {
                        path.push(next);
                    }
                }
            })
            .tree_fold1(|a, b| {
                // Get the lcd loop length
                let length = a.length.lcm(&b.length);

                let mut a_c = Vec::new();
                for i in 0..(length / a.length) {
                    for item in &a.content {
                        if i == 0 || *item >= a.start {
                            a_c.push(item + i * a.length);
                        }
                    }
                }

                let mut content = Vec::new();
                for i in 0..(length / b.length) {
                    for item in &b.content {
                        let val = item + i * b.length;
                        if (i == 0 || *item >= b.start) && a_c.contains(&val) {
                            content.push(val);
                        }
                    }
                }

                let start = a.start.max(b.start);

                Loop {
                    start,
                    length,
                    content,
                }
            })
            .unwrap()
            .content
            .into_iter()
            .next()
            .unwrap()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample_1.txt")).unwrap();
    assert_eq!(solution.part_1(), 2);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample_2.txt")).unwrap();
    assert_eq!(solution.part_2(), 6);
}
