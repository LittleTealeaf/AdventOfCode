fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
}

#[derive(Debug)]
struct Solution {
    reports: Vec<Vec<i32>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            reports: input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|i| i.parse().ok())
                        .collect()
                })
                .collect(),
        }
    }
}

impl Solution {
    fn part_1(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| {
                let diff: Vec<_> = report
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();

                !diff.contains(&0)
                    && (diff.iter().all(|i| (&1..=&3).contains(&i))
                        || diff.iter().all(|i| (&-3..=&-1).contains(&i)))
            })
            .count()
    }
}
