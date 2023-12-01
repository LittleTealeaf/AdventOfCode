fn main() {
    let input = include_str!("../../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|chars| {
            let chars = chars.chars().filter(|i| i.is_numeric()).collect::<Vec<_>>();
            [chars.first().unwrap(), chars.last().unwrap()]
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    const DIGITS: [(&str, char); 9] = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    input
        .lines()
        .map(|chars| {
            let string = chars.chars().collect::<Vec<_>>();

            let first_digit = {
                let mut ch = '\0';
                'l: for i in 0..string.len() {
                    if string[i].is_ascii_digit() {
                        ch = string[i];
                        break 'l;
                    } else {
                        for (word, dig) in DIGITS {
                            if i + word.len() <= string.len() {
                                let window = &string[i..i + word.len()].iter().collect::<String>();
                                if window == word {
                                    ch = dig;
                                    break 'l;
                                }
                            }
                        }
                    }
                }
                ch
            };

            let last_digit = {
                let mut ch = '\0';
                'l: for i in (0..string.len()).rev() {
                    if string[i].is_ascii_digit() {
                        ch = string[i];
                        break 'l;
                    } else {
                        for (word, dig) in DIGITS {
                            if i + word.len() <= string.len() {
                                let window = &string[i..i + word.len()].iter().collect::<String>();
                                if window == word {
                                    ch = dig;
                                    break 'l;
                                }
                            }
                        }
                    }
                }
                ch
            };

            [first_digit, last_digit]
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

#[test]
fn part_2_example() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part_2(input), 281);
}
