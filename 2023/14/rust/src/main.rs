fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    map: Vec<Vec<char>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn part_1(&self) -> usize {
        let mut map = self.map.clone();
        #[allow(clippy::never_loop)]
        let mut modified = true;
        while modified {
            modified = false;
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if i > 0 && map[i][j] == 'O' && map[i - 1][j] == '.' {
                        map[i - 1][j] = 'O';
                        map[i][j] = '.';
                        modified = true;
                    }
                }
            }
        }

        map.into_iter()
            .rev()
            .enumerate()
            .map(|(index, row)| {
                row.into_iter()
                    .filter_map(|ch| (ch == 'O').then_some(index + 1))
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn part_2(&self) -> usize {
        let mut map = self.map.clone();

        let mut prev_states: Vec<Vec<Vec<char>>> = vec![];

        let mut iter = 0;

        const LENGTH: usize = 1000000000;

        while iter < LENGTH {
            if let Some(index) = prev_states.iter().position(|p| p == &map) {
                let length = prev_states.len() - index;
                let left = LENGTH - iter;

                map = prev_states[index + left % length].clone();
                break;
            } else {
                prev_states.push(map.clone());
            }

            let mut modified = true;
            while modified {
                modified = false;
                for i in 1..map.len() {
                    for j in 0..map[i].len() {
                        if map[i][j] == 'O' && map[i - 1][j] == '.' {
                            map[i - 1][j] = 'O';
                            map[i][j] = '.';
                            modified = true;
                        }
                    }
                }
            }
            modified = true;
            while modified {
                modified = false;
                for j in 1..map[0].len() {
                    for row in &mut map {
                        if row[j] == 'O' && row[j - 1] == '.' {
                            row[j - 1] = 'O';
                            row[j] = '.';
                            modified = true;
                        }
                    }
                }
            }
            modified = true;
            while modified {
                modified = false;
                for i in (0..(map.len() - 1)).rev() {
                    for j in 0..map[i].len() {
                        if map[i][j] == 'O' && map[i + 1][j] == '.' {
                            map[i + 1][j] = 'O';
                            map[i][j] = '.';
                            modified = true;
                        }
                    }
                }
            }

            modified = true;
            while modified {
                modified = false;

                for j in (0..(map[0].len() - 1)).rev() {
                    for row in &mut map {
                        if row[j] == 'O' && row[j + 1] == '.' {
                            row[j + 1] = 'O';
                            row[j] = '.';
                            modified = true;
                        }
                    }
                }
            }
            iter += 1;
        }

        map.into_iter()
            .rev()
            .enumerate()
            .map(|(index, row)| {
                row.into_iter()
                    .filter_map(|ch| (ch == 'O').then_some(index + 1))
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 136);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 64);
}
