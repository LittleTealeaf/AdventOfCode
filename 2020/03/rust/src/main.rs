fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    map: Vec<Vec<bool>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }

    fn part_1(&self) -> usize {
        let steps = self.map.len();
        (0..steps)
            .filter(|i| self.map[*i][(i * 3) % self.map[*i].len()])
            .count()
    }

    fn part_2(&self) -> u64 {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .map(|(dx, dy)| {
                (0..(self.map.len() / dy + self.map.len() % dy))
                    .filter(|i| {
                        self.map
                            .get(i * dy)
                            .unwrap_or_else(|| self.map.last().unwrap())
                            [(*i * dx) % self.map[0].len()]
                    })
                    .count() as u64
            })
            .product()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 7);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 336);
}
