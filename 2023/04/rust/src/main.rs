fn main() {
    let solution = Solution {
        input: include_str!("../../input.txt").to_string(),
    };

    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

struct Solution {
    input: String,
}

impl Solution {
    fn part_1(&self) -> i32 {
        self.input
            .lines()
            .map(|line| {
                let mut line = line.to_string();
                while line.contains("  ") {
                    line = line.replace("  ", " ");
                }
                let mut tokens = line.split(' ');
                let _ = tokens.next();
                let _ = tokens.next();

                let mut winning = Vec::new();

                for a in tokens.by_ref() {
                    if a == "|" {
                        break;
                    } else {
                        winning.push(a);
                    }
                }

                let mut score = 0;

                for n in tokens {
                    if winning.contains(&n) {
                        if score == 0 {
                            score = 1;
                        } else {
                            score *= 2;
                        }
                    }
                }

                score
            })
            .sum()
    }

    fn part_2(&self) -> i32 {
        let cards = self
            .input
            .lines()
            .map(|line| {
                let mut line = line.to_string();
                while line.contains("  ") {
                    line = line.replace("  ", " ");
                }
                let mut tokens = line.split(' ');
                let _ = tokens.next();
                let _ = tokens.next();

                let mut winning = Vec::new();

                for a in tokens.by_ref() {
                    if a == "|" {
                        break;
                    } else {
                        winning.push(a);
                    }
                }

                let mut count = 0;

                for n in tokens {
                    if winning.contains(&n) {
                        count += 1;
                    }
                }

                count
            })
            .collect::<Vec<_>>();

        let mut stack = (0..(cards.len())).collect::<Vec<_>>();
        let mut count = 0;

        while let Some(number) = stack.pop() {
            count += 1;
            for i in 0..cards[number] {
                stack.push(1 + number + i);
            }
        }

        count
    }
}

#[test]
fn test_part_1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string();
    let solution = Solution { input };

    assert_eq!(solution.part_1(), 13);
}

#[test]
fn test_part_2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string();

    let solution = Solution { input };

    assert_eq!(solution.part_2(), 30);
}
