use std::collections::HashMap;

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
            prefix: Vec<bool>,
            content: Vec<bool>,
        }

        impl Loop {
            fn get(&self, index: usize) -> bool {
                if index < self.start {
                    self.prefix[index]
                } else {
                    self.content[(index - self.start) % self.content.len()]
                }
            }
        }

        let loops = self
            .map
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
                    if let Some(i) = path.iter().position(|i| i == &next) {
                        if path.len() % self.instructions.len() == i % self.instructions.len() {
                            return Loop {
                                start: i,
                                prefix: path[..i].iter().map(|i| i.ends_with('Z')).collect(),
                                content: path[i..].iter().map(|i| i.ends_with('Z')).collect(),
                            };
                        } else {
                            path.push(next);
                        }
                    } else {
                        path.push(next);
                    }
                }
            })
            .collect::<Vec<_>>();

        let check_loop_length = loops
            .iter()
            .map(|l| l.content.len())
            .reduce(|a, b| a.lcm(&b))
            .unwrap();
        let last_start = loops.iter().map(|l| l.start).max().unwrap();

        // Alternate solution could be to merge together loops

        let max_length = check_loop_length + last_start;
        'l: for i in (0..=max_length).rev() {
            for l in &loops {
                if !l.get(i) {
                    continue 'l;
                }
            }
            return i;
        }

        0
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
