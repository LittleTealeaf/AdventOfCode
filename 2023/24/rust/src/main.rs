fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!(
        "Part 1: {}",
        solution.part_1(200000000000000.0, 400000000000000.0)
    );
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Hailstone {
    point: Point,
    velocity: Point,
}

struct Solution {
    stones: Vec<Hailstone>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            stones: input
                .lines()
                .map(|line| {
                    let mut points = line.split(" @ ").map(|point| {
                        let mut tokens = point.split(',');
                        Point {
                            x: tokens.next().unwrap().trim().parse().unwrap(),
                            y: tokens.next().unwrap().trim().parse().unwrap(),
                            z: tokens.next().unwrap().trim().parse().unwrap(),
                        }
                    });
                    Hailstone {
                        point: points.next().unwrap(),
                        velocity: points.next().unwrap(),
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self, min: f64, max: f64) -> usize {
        let mut count = 0;

        for i in 0..(self.stones.len() - 1) {
            let a = &self.stones[i];

            let a_slope = a.velocity.y / a.velocity.x;
            let a_b = a.point.y - a.point.x * a_slope;

            for j in i + 1..self.stones.len() {
                let b = &self.stones[j];

                let b_slope = b.velocity.y / b.velocity.x;
                let b_b = b.point.y - b.point.x * b_slope;

                let denominator = b_slope - a_slope;
                if denominator == 0.0 {
                    continue;
                }

                let numerator = a_b - b_b;

                let x = numerator / denominator;
                let y = a_b + a_slope * x;

                if x < min || x > max || y < min || y > max {
                    continue;
                }

                if (a.velocity.x > 0.0 && x < a.point.x) || (a.velocity.x < 0.0 && x > a.point.x) {
                    continue;
                }

                if (a.velocity.y > 0.0 && y < a.point.y) || (a.velocity.y < 0.0 && y > a.point.y) {
                    continue;
                }

                if (b.velocity.x > 0.0 && x < b.point.x) || (b.velocity.x < 0.0 && x > b.point.x) {
                    continue;
                }

                if (b.velocity.y > 0.0 && y < b.point.y) || (b.velocity.y < 0.0 && y > b.point.y) {
                    continue;
                }

                count += 1;
            }
        }

        count
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(7.0, 27.0), 2);
}
