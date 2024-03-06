use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));

    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    lines: Vec<((i32, i32), (i32, i32))>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            lines: input
                .lines()
                .map(|line| {
                    let mut points = line.split(" -> ").map(|point| {
                        let (x, y) = point.split_once(',').unwrap();
                        (x.parse().unwrap(), y.parse().unwrap())
                    });
                    let (x1, y1) = points.next().unwrap();
                    let (x2, y2) = points.next().unwrap();
                    ((x1, y1), (x2, y2))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let mut map = HashMap::new();

        for ((x1, y1), (x2, y2)) in &self.lines {
            if x1 == x2 || y1 == y2 {
                for x in {
                    if x1 < x2 {
                        *x1..=*x2
                    } else {
                        *x2..=*x1
                    }
                } {
                    for y in {
                        if y1 < y2 {
                            *y1..=*y2
                        } else {
                            *y2..=*y1
                        }
                    } {
                        if let Some(count) = map.get_mut(&(x, y)) {
                            *count += 1;
                        } else {
                            map.insert((x, y), 1);
                        }
                    }
                }
            }
        }

        map.into_values().filter(|i| *i > 1).count()
    }

    fn part_2(&self) -> usize {
        let mut map = HashMap::new();

        for ((x1, y1), (x2, y2)) in &self.lines {
            let dx = match x1.cmp(x2) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            let dy = match y1.cmp(y2) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            let count = x1.abs_diff(*x2).max(y1.abs_diff(*y2)) as i32;

            for i in 0..=count {
                let x = x1 + dx * i;
                let y = y1 + dy * i;
                if let Some(count) = map.get_mut(&(x, y)) {
                    *count += 1;
                } else {
                    map.insert((x, y), 1);
                }
            }
        }

        map.into_values().filter(|i| *i > 1).count()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );
    assert_eq!(solution.part_1(), 5);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );

    assert_eq!(solution.part_2(), 12);
}
