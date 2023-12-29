fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    digits: Vec<Vec<u32>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            digits: input
                .lines()
                .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> u32 {
        let digits = self
            .digits
            .iter()
            .map(|number| {
                number
                    .iter()
                    .map(|val| if val == &0 { (1, 0) } else { (0, 1) })
                    .collect::<Vec<_>>()
            })
            .reduce(|a, b| {
                (0..a.len())
                    .map(|i| (a[i].0 + b[i].0, a[i].1 + b[i].1))
                    .collect::<Vec<_>>()
            })
            .unwrap();

        let gamma = u32::from_str_radix(
            digits
                .iter()
                .map(|(a, b)| if a > b { "0" } else { "1" })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        let epsilon = u32::from_str_radix(
            digits
                .iter()
                .map(|(a, b)| if a > b { "1" } else { "0" })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        gamma * epsilon
    }

    fn part_2(&self) -> u32 {
        let width = self.digits[0].len();

        let mut oxygen_number = Vec::new();
        let mut co2_number = Vec::new();

        let mut oxygen = self.digits.iter().collect::<Vec<_>>();
        let mut co2 = self.digits.iter().collect::<Vec<_>>();

        for i in 0..width {
            {
                let (zeros, ones) = oxygen
                    .iter()
                    .map(|num| num[i])
                    .map(|i| if i == 0 { (1, 0) } else { (0, 1) })
                    .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
                let number = if ones >= zeros { 1 } else { 0 };
                oxygen_number.push(number.to_string());
                oxygen.retain(|row| row[i] == number);
            }

            {
                if co2.len() == 1 {
                    co2_number.push(co2[0][i].to_string());
                } else {
                    let (zeros, ones) = co2
                        .iter()
                        .map(|num| num[i])
                        .map(|i| if i == 0 { (1, 0) } else { (0, 1) })
                        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

                    let number = if ones < zeros { 1 } else { 0 };
                    co2_number.push(number.to_string());
                    co2.retain(|row| row[i] == number);
                }
            }
        }

        let oxygen_result =
            u32::from_str_radix(oxygen_number.into_iter().collect::<String>().as_ref(), 2).unwrap();
        let co2_result =
            u32::from_str_radix(co2_number.into_iter().collect::<String>().as_ref(), 2).unwrap();

        oxygen_result * co2_result
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 198);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 230);
}
