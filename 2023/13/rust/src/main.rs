fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
}

struct Solution {
    fields: Vec<Vec<Vec<bool>>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut fields = Vec::new();
        let mut field = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                fields.push(field);
                field = Vec::new();
            } else {
                field.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
            }
        }

        fields.push(field);

        Self { fields }
    }

    fn part_1(&self) -> usize {
        self.fields
            .iter()
            .map(|field| {
                'horizontal: for i in 0..(field.len() - 1) {
                    for j in 0..=i {
                        if let (Some(a), Some(b)) = (field.get(i - j), field.get(i + j + 1)) {
                            if a != b {
                                continue 'horizontal;
                            }
                        }
                    }
                    return (i + 1) * 100;
                }

                'vertical: for i in 0..(field[0].len() - 1) {
                    for j in 0..=i {
                        if i >= j && i + j + 1 < field[0].len() {
                            for row in field {
                                if row[i - j] != row[i + j + 1] {
                                    continue 'vertical;
                                }
                            }
                        }
                    }
                    return i + 1;
                }

                0
            })
            .sum()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 405);
}
