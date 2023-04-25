use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .filter(|line| {
            let mut set = HashSet::new();
            for item in line.split(" ") {
                if !set.insert(item) {
                    return false;
                }
            }
            return true;
        })
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .filter(|line| {
            let mut set = HashSet::new();
            for item in line.split(" ").map(Password::from) {
                if !set.insert(item) {
                    return false;
                }
            }
            true
        })
        .count()
}

#[derive(Eq, PartialEq, Hash)]
struct Password {
    counts: [usize; 26],
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        let mut counts = [0; 26];
        for c in value.chars() {
            counts[c as usize - 'a' as usize] += 1;
        }
        Self { counts }
    }
}
