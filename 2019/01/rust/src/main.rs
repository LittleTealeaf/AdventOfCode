fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));

    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Debug)]
struct Solution {
    modules: Vec<i32>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            modules: input.lines().filter_map(|line| line.parse().ok()).collect(),
        }
    }

    fn part_1(&self) -> i32 {
        self.modules.iter().map(|mass| 0.max(mass / 3 - 2)).sum()
    }

    fn part_2(&self) -> i32 {
        self.modules
            .iter()
            .map(|mass| {
                let mut mass = 0.max(*mass / 3 - 2);
                let mut fuel = 0;

                while mass > 0 {
                    fuel += mass;

                    mass = 0.max(mass / 3 - 2);
                }

                fuel
            })
            .sum()
    }
}
