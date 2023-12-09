fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    variables: Vec<Vec<i32>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            variables: input
                .lines()
                .map(|line| {
                    line.split(' ')
                        .map(|token| token.parse::<i32>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> i32 {
        fn predict_next(values: Vec<i32>) -> i32 {
            if values.iter().all(|i| i == &0) {
                return 0;
            }
            let mut diff = Vec::new();
            let mut iter = values.into_iter();
            let mut last = iter.next().unwrap();

            for val in iter {
                diff.push(val - last);
                last = val;
            }

            let next_diff = predict_next(diff);

            last + next_diff
        }

        self.variables.iter().map(|v| predict_next(v.clone())).sum()
    }

    fn part_2(&self) -> i32 {
        fn predict_next(values: Vec<i32>) -> i32 {
            if values.iter().all(|i| i == &0) {
                return 0;
            }
            let mut diff = Vec::new();
            let mut iter = values.into_iter();
            let mut last = iter.next().unwrap();
            let first = last;

            for val in iter {
                diff.push(val - last);
                last = val;
            }

            let next_diff = predict_next(diff);

            first - next_diff
        }

        self.variables.iter().map(|v| predict_next(v.clone())).sum()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 114);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 2);
}
