fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Solution {
    galaxies: Vec<Coordinate>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            galaxies: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, val)| {
                        (val == '#').then_some(Coordinate {
                            x: x as i64,
                            y: y as i64,
                        })
                    })
                })
                .collect(),
        }
    }

    fn part_1(&self) -> u64 {
        let mut galaxies = self.galaxies.clone();
        let min_x = galaxies.iter().map(|g| g.x).min().unwrap();
        let max_x = galaxies.iter().map(|g| g.x).max().unwrap();
        let min_y = galaxies.iter().map(|g| g.y).min().unwrap();
        let max_y = galaxies.iter().map(|g| g.y).max().unwrap();

        (min_x..=max_x)
            .filter(|x| !galaxies.iter().any(|g| g.x == *x))
            .enumerate()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(i, x)| {
                galaxies.iter_mut().for_each(|g| {
                    if g.x > x + i as i64 {
                        g.x += 1;
                    }
                });
            });
        (min_y..=max_y)
            .filter(|y| !galaxies.iter().any(|g| g.y == *y))
            .enumerate()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(i, y)| {
                galaxies.iter_mut().for_each(|g| {
                    if g.y > y + i as i64 {
                        g.y += 1;
                    }
                });
            });

        galaxies
            .iter()
            .flat_map(|a| {
                galaxies
                    .iter()
                    .filter(move |&b| (a != b))
                    .map(|b| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
            })
            .sum::<u64>()
            / 2
    }

    fn part_2(&self) -> u64 {
        const INCREASE: i64 = 1_000_000 - 1;

        let mut galaxies = self.galaxies.clone();
        let min_x = galaxies.iter().map(|g| g.x).min().unwrap();
        let max_x = galaxies.iter().map(|g| g.x).max().unwrap();
        let min_y = galaxies.iter().map(|g| g.y).min().unwrap();
        let max_y = galaxies.iter().map(|g| g.y).max().unwrap();

        (min_x..=max_x)
            .filter(|x| !galaxies.iter().any(|g| g.x == *x))
            .enumerate()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(i, x)| {
                galaxies.iter_mut().for_each(|g| {
                    if g.x > x + (i as i64 * INCREASE) {
                        g.x += INCREASE;
                    }
                });
            });
        (min_y..=max_y)
            .filter(|y| !galaxies.iter().any(|g| g.y == *y))
            .enumerate()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(i, y)| {
                galaxies.iter_mut().for_each(|g| {
                    if g.y > y + (i as i64 * INCREASE) {
                        g.y += INCREASE;
                    }
                });
            });

        galaxies
            .iter()
            .flat_map(|a| {
                galaxies
                    .iter()
                    .map(|b| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
            })
            .sum::<u64>()
            / 2
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 374);
}
