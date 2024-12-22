use std::{
    collections::{BinaryHeap, HashMap},
    ops::AddAssign,
};

fn main() {
    let input = include_str!("../../input.txt");
    let solution = Solution::new(input);
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Debug)]
struct Solution {
    items: Vec<(u32, u32)>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            items: input
                .lines()
                .filter_map(|line| {
                    let mut split = line.split(" ");
                    let a = split.next()?;
                    let mut split = split.skip(2);
                    let b = split.next()?;
                    Some((a.parse().ok()?, b.parse().ok()?))
                })
                .collect(),
        }
    }
}

impl Solution {
    fn part_1(&self) -> u32 {
        let mut list_a = BinaryHeap::new();
        let mut list_b = BinaryHeap::new();

        for (a, b) in &self.items {
            list_a.push(*a);
            list_b.push(*b);
        }

        let mut total = 0;

        while let (Some(a), Some(b)) = (list_a.pop(), list_b.pop()) {
            total += a.abs_diff(b);
        }

        total
    }
}

impl Solution {
    fn part_2(&self) -> u32 {
        // Create map
        let mut counts: HashMap<u32, u32> = HashMap::new();

        for (_, i) in &self.items {
            if let Some(tally) = counts.get_mut(i) {
                tally.add_assign(1);
            } else {
                counts.insert(*i, 1);
            }
        }

        self.items
            .iter()
            .filter_map(|(i, _)| counts.get(i).map(|j| i * j))
            .sum()
    }
}
