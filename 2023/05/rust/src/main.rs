use std::cmp::Ordering;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));

    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Debug)]
struct Solution {
    seeds: Vec<usize>,
    maps: Vec<Vec<Mapping>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Mapping {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds = {
            let line = lines.next().unwrap();
            let seg = &line[7..];
            seg.split(' ')
                .map(|i| i.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        };
        let _ = lines.next();
        let _ = lines.next();

        let mut maps = Vec::new();
        let mut map = Vec::new();

        for line in lines {
            if line.is_empty() {
                maps.push(map);
                map = Vec::new();
            } else if !line.contains("map") {
                let mut tokens = line.split(' ');
                map.push(Mapping {
                    dest_start: tokens.next().unwrap().parse().unwrap(),
                    source_start: tokens.next().unwrap().parse().unwrap(),
                    length: tokens.next().unwrap().parse().unwrap(),
                })
            }
        }
        maps.push(map);

        Solution { maps, seeds }
    }

    fn part_1(&self) -> usize {
        let mut set = self.seeds.clone();

        for map in &self.maps {
            set = set
                .iter()
                .map(|val| {
                    for m in map {
                        if m.source_start <= *val && m.source_start + m.length > *val {
                            return val - m.source_start + m.dest_start;
                        }
                    }
                    *val
                })
                .collect::<Vec<_>>();
        }

        set.into_iter().min().unwrap()
    }

    fn part_2(&self) -> usize {
        #[derive(Clone, Copy)]
        struct Range {
            start: usize,
            length: usize,
        }

        let mut ranges = Vec::new();

        {
            let mut iter = self.seeds.iter();
            while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
                ranges.push(Range {
                    start: *a,
                    length: *b,
                });
            }
        }

        for map in &self.maps {
            ranges = ranges
                .iter()
                .flat_map(|range| {
                    let mut ranges = vec![*range];
                    let mut new_ranges = Vec::new();
                    for m in map {
                        ranges = ranges
                            .iter()
                            .flat_map(|range| {
                                let start_m = m.source_start;
                                let end_m = m.source_start + m.length;
                                let start_r = range.start;
                                let end_r = range.start + range.length;

                                match (start_m.cmp(&start_r), end_m.cmp(&end_r)) {
                                    (Ordering::Less, Ordering::Greater)
                                    | (Ordering::Less, Ordering::Equal)
                                    | (Ordering::Equal, Ordering::Greater)
                                    | (Ordering::Equal, Ordering::Equal) => {
                                        new_ranges.push(Range {
                                            start: start_m.max(start_r) - start_m + m.dest_start,
                                            length: range.length,
                                        });
                                        Vec::new()
                                    }
                                    (Ordering::Equal, Ordering::Less) => {
                                        new_ranges.push(Range {
                                            start: m.dest_start,
                                            length: m.length,
                                        });
                                        vec![Range {
                                            start: end_m,
                                            length: end_r - end_m,
                                        }]
                                    }
                                    (Ordering::Greater, Ordering::Equal) => {
                                        new_ranges.push(Range {
                                            start: m.dest_start,
                                            length: m.length,
                                        });
                                        vec![Range {
                                            start: start_r,
                                            length: start_m - start_r,
                                        }]
                                    }
                                    (Ordering::Greater, Ordering::Less) => {
                                        new_ranges.push(Range {
                                            start: m.dest_start,
                                            length: m.length,
                                        });
                                        vec![
                                            Range {
                                                start: range.start,
                                                length: start_m - range.start,
                                            },
                                            Range {
                                                start: end_m,
                                                length: end_r - end_m,
                                            },
                                        ]
                                    }
                                    (Ordering::Greater, Ordering::Greater) => {
                                        if start_m < end_r {
                                            new_ranges.push(Range {
                                                start: m.dest_start,
                                                length: end_r - start_m,
                                            });
                                            vec![Range {
                                                start: start_r,
                                                length: start_m - start_r,
                                            }]
                                        } else {
                                            vec![*range]
                                        }
                                    }
                                    (Ordering::Less, Ordering::Less) => {
                                        if end_m > start_r {
                                            new_ranges.push(Range {
                                                start: start_r - start_m + m.dest_start,
                                                length: end_m - start_r,
                                            });
                                            vec![Range {
                                                start: end_m,
                                                length: end_r - end_m,
                                            }]
                                        } else {
                                            vec![*range]
                                        }
                                    }
                                }
                            })
                            .collect();
                    }
                    for range in ranges {
                        new_ranges.push(range);
                    }
                    new_ranges
                })
                .collect::<Vec<_>>();
        }

        ranges.into_iter().map(|range| range.start).min().unwrap()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 35);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 46);
}
