#![allow(dead_code)]

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Solution {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self {
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    fn get_number(&self, y: usize, x: usize) -> Option<((usize, usize), i32)> {
        if self.grid.get(y)?.get(x)?.is_ascii_digit() {
            let left = {
                let mut l = x;
                for x_i in (0..x).rev() {
                    if !self.grid[y][x_i].is_ascii_digit() {
                        break;
                    } else {
                        l = x_i
                    }
                }
                l
            };
            let right = {
                let mut l = x;
                for x_i in x + 1..self.width {
                    if !self.grid[y][x_i].is_ascii_digit() {
                        break;
                    } else {
                        l = x_i;
                    }
                }
                l
            };

            let slice = &self.grid[y][left..=right];

            Some((
                (y, left),
                slice.iter().collect::<String>().parse::<i32>().unwrap(),
            ))
        } else {
            None
        }
    }

    fn part_1(&self) -> i32 {
        (0..self.height)
            .map(|i| {
                (0..self.width)
                    .filter_map(|j| {
                        (!self.grid[i][j].is_ascii_digit() && self.grid[i][j] != '.').then(|| {
                            let mut nums = Vec::new();
                            [
                                (i > 0).then(|| (i - 1, j)),
                                (i > 0 && j > 0).then(|| (i - 1, j - 1)),
                                (j > 0).then(|| (i, j - 1)),
                                (j > 0 && i < self.height - 1).then(|| (i + 1, j - 1)),
                                (i < self.height - 1).then(|| (i + 1, j)),
                                (i < self.height - 1 && j < self.width - 1).then(|| (i + 1, j + 1)),
                                (j < self.width - 1).then(|| (i, j + 1)),
                                (j < self.width - 1 && i > 0).then(|| (i - 1, j + 1)),
                            ]
                            .into_iter()
                            .flatten()
                            .filter_map(|(y, x)| {
                                self.get_number(y, x).and_then(|(pos, num)| {
                                    (!nums.contains(&pos)).then(|| {
                                        nums.push(pos);
                                        num
                                    })
                                })
                            })
                            .sum::<i32>()
                        })
                    })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }

    fn part_2(&self) -> i32 {
        (0..self.height)
            .map(|i| {
                (0..self.width)
                    .filter_map(|j| {
                        (!self.grid[i][j].is_ascii_digit() && self.grid[i][j] != '.').then(
                            || {
                                let mut nums = Vec::new();
                                let nums = [
                                    (i > 0).then(|| (i - 1, j)),
                                    (i > 0 && j > 0).then(|| (i - 1, j - 1)),
                                    (j > 0).then(|| (i, j - 1)),
                                    (j > 0 && i < self.height - 1).then(|| (i + 1, j - 1)),
                                    (i < self.height - 1).then(|| (i + 1, j)),
                                    (i < self.height - 1 && j < self.width - 1)
                                        .then(|| (i + 1, j + 1)),
                                    (j < self.width - 1).then(|| (i, j + 1)),
                                    (j < self.width - 1 && i > 0).then(|| (i - 1, j + 1)),
                                ]
                                .into_iter()
                                .flatten()
                                .filter_map(|(y, x)| {
                                    self.get_number(y, x).and_then(|(pos, num)| {
                                        (!nums.contains(&pos)).then(|| {
                                            nums.push(pos);
                                            num
                                        })
                                    })
                                })
                                .collect::<Vec<_>>();
                                (nums.len() == 2).then(|| nums.into_iter().product::<i32>())
                            },
                        )?
                    })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }
}

#[test]
fn part_1_example() {
    let sol = Solution::new(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    );
    assert_eq!(sol.part_1(), 4361);
}

#[test]
fn part_2_example() {
    let sol = Solution::new(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    );
    assert_eq!(sol.part_2(), 467835);
}
