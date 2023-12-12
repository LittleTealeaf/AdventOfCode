use std::collections::HashMap;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
struct Row {
    values: Vec<Tile>,
    damaged: Vec<usize>,
}

#[derive(Debug)]
struct Solution {
    rows: Vec<Row>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            rows: input
                .lines()
                .map(|line| {
                    let (values, damaged) = line.split_once(' ').unwrap();
                    Row {
                        values: values
                            .chars()
                            .map(|c| match c {
                                '.' => Tile::Operational,
                                '#' => Tile::Damaged,
                                '?' => Tile::Unknown,
                                _ => panic!(),
                            })
                            .collect(),
                        damaged: damaged
                            .split(',')
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect(),
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                fn recursive(mut row: Row) -> usize {
                    let mut value_iter = row.damaged.iter();
                    let mut count = 0;

                    for i in 0..row.values.len() {
                        let tile = &row.values[i];
                        match tile {
                            Tile::Operational => {
                                if count > 0
                                    && value_iter.next().map(|i| i != &count).unwrap_or(true)
                                {
                                    return 0;
                                }
                                count = 0;
                            }
                            Tile::Damaged => {
                                count += 1;
                            }
                            Tile::Unknown => {
                                row.values[i] = Tile::Damaged;
                                let mut values = recursive(row.clone());
                                row.values[i] = Tile::Operational;
                                values += recursive(row);
                                return values;
                            }
                        }
                    }

                    if value_iter.next().map(|l| l != &count).unwrap_or(count > 0)
                        || value_iter.next().is_some()
                    {
                        0
                    } else {
                        1
                    }
                }

                recursive(row.clone())
            })
            .sum()
    }

    fn part_2(&self) -> usize {
        self.rows
            .iter()
            .map(|source_row| {
                let mut row = Row {
                    values: source_row.values.clone(),
                    damaged: source_row.damaged.clone(),
                };

                for _ in 1..5 {
                    row.values.push(Tile::Unknown);
                    row.values.append(&mut source_row.values.clone());
                    row.damaged.append(&mut source_row.damaged.clone());
                }

                #[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
                struct Key {
                    damaged: Vec<usize>,
                    index: usize,
                    count: usize,
                }

                let mut memo: HashMap<Key, usize> = HashMap::new();

                fn recursive(mut row: Row, memo: &mut HashMap<Key, usize>) -> usize {
                    let mut value_iter = row.damaged.iter();
                    let mut count = 0;

                    for index in 0..row.values.len() {
                        let tile = &row.values[index];
                        match tile {
                            Tile::Operational => {
                                if count > 0
                                    && value_iter.next().map(|i| i != &count).unwrap_or(true)
                                {
                                    return 0;
                                }
                                count = 0;
                            }
                            Tile::Damaged => {
                                count += 1;
                            }
                            Tile::Unknown => {
                                let key = Key {
                                    damaged: value_iter.cloned().collect(),
                                    index,
                                    count,
                                };

                                if let Some(result) = memo.get(&key) {
                                    return *result;
                                } else {
                                    row.values[index] = Tile::Damaged;
                                    let mut values = recursive(row.clone(), memo);
                                    row.values[index] = Tile::Operational;
                                    values += recursive(row.clone(), memo);

                                    memo.insert(key, values);

                                    return values;
                                }
                            }
                        }
                    }

                    if value_iter.next().map(|l| l != &count).unwrap_or(count > 0)
                        || value_iter.next().is_some()
                    {
                        0
                    } else {
                        1
                    }
                }

                recursive(row.clone(), &mut memo)
            })
            .sum::<usize>()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 21);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 525152);
}
