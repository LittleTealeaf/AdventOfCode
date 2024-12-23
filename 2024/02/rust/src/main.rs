fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
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
    fn is_safe(report: &[i32]) -> bool {
        let diff: Vec<_> = report
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        !diff.contains(&0)
            && (diff.iter().all(|i| (&1..=&3).contains(&i))
                || diff.iter().all(|i| (&-3..=&-1).contains(&i)))
    }

    fn part_1(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| Solution::is_safe(report))
            .count()
    }
}

impl Solution {
    fn part_2(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| {
                Solution::is_safe(report)
                    || (0..report.len()).any(|i| {
                        let mut rpt = (*report).clone();
                        rpt.remove(i);
                        Solution::is_safe(&rpt)
                    })
            })
            .count()
    }
}
