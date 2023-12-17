fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

struct Solution {
    passwords: Vec<Password>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            passwords: input
                .lines()
                .map(|line| {
                    let mut tokens = line.split(' ');
                    let mut range = tokens
                        .next()
                        .unwrap()
                        .split('-')
                        .map(|s| s.parse::<usize>().unwrap());
                    let letter = tokens.next().unwrap().chars().next().unwrap();
                    let password = tokens.next().unwrap().to_string();
                    Password {
                        min: range.next().unwrap(),
                        max: range.next().unwrap(),
                        letter,
                        password,
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        self.passwords
            .iter()
            .filter(
                |Password {
                     min,
                     max,
                     letter,
                     password,
                 }| {
                    let count = password.chars().filter(|c| c.eq(letter)).count();
                    count >= *min && count <= *max
                },
            )
            .count()
    }

    fn part_2(&self) -> usize {
        self.passwords
            .iter()
            .filter(
                |Password {
                     min,
                     max,
                     letter,
                     password,
                 }| {
                    let chars = password.chars().collect::<Vec<_>>();
                    chars.get(*min - 1).map(|i| i == letter).unwrap_or(false)
                        != chars.get(*max - 1).map(|i| i == letter).unwrap_or(false)
                },
            )
            .count()
    }
}
