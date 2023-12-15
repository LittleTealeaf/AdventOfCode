use std::collections::HashMap;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    tokens: Vec<String>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            tokens: input
                .trim()
                .split(',')
                .map(|token| token.to_string())
                .collect(),
        }
    }

    fn part_1(&self) -> i32 {
        self.tokens
            .iter()
            .map(|token| puzzle_hash(token.as_str()))
            .sum()
    }

    fn part_2(&self) -> i32 {
        struct Lens<'a> {
            label: &'a str,
            focus: i32,
        }
        let mut map: HashMap<i32, Vec<Lens>> = HashMap::new();
        for token in &self.tokens {
            let index = token.find('=').or(token.find('-')).unwrap();
            let hash = puzzle_hash(&token[..index]);
            let entry = map.entry(hash).or_default();

            if token.contains('-') {
                if let Some(i) = entry.iter().position(|l| l.label == &token[..index]) {
                    entry.remove(i);
                }
            } else if let Some(i) = entry.iter().position(|l| l.label == &token[..index]) {
                entry[i].focus = token[index + 1..].parse().unwrap();
            } else {
                entry.push(Lens {
                    label: &token[..index],
                    focus: token[index + 1..].parse().unwrap(),
                });
            }
        }

        map.into_iter()
            .flat_map(|(b, lens)| {
                lens.into_iter()
                    .enumerate()
                    .map(move |(index, lens)| (b + 1) * (index as i32 + 1) * (lens.focus))
            })
            .sum()
    }
}

fn puzzle_hash(input: &str) -> i32 {
    input
        .chars()
        .map(|c| c as i32)
        .fold(0, |current, char| ((current + char) * 17) % 256)
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 1320);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 145);
}
