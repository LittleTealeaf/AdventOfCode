use regex::Regex;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
}

struct Solution {
    text: Vec<String>,
}

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            text: input.lines().map(|i| i.to_string()).collect(),
        }
    }
}

impl Solution {
    fn part_1(&self) -> i32 {
        let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

        let text = self.text.iter().cloned().collect::<String>();

        regex
            .find_iter(&text)
            .map(|m| {
                let s = m.as_str();
                let s = &s[4..s.len() - 1];
                let product = s
                    .split(',')
                    .filter_map(|n| n.parse::<i32>().ok())
                    .product::<i32>();
                product
            })
            .sum::<i32>()
    }
}


#[test]
fn test_part_1() {
    assert_eq!(
        Solution::new("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
            .part_1(),
        161
    );
}
