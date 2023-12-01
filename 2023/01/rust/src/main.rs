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
        .map(|line| {
            let string = line.to_string();
            let chars = string.chars().collect::<Vec<_>>();

            [
                (0..string.len()).collect::<Vec<_>>(),
                (0..string.len()).rev().collect::<Vec<_>>(),
            ]
            .map(|range| {
                for i in range {
                    if chars[i].is_ascii_digit() {
                        return chars[i];
                    } else {
                        for (word, dig) in DIGITS {
                            if i + word.len() <= string.len() {
                                if &string[i..i + word.len()] == word {
                                    return dig;
                                }
                            }
                        }
                    }
                }
                panic!()
            })
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

#[test]
fn part_2_answer() {
    assert_eq!(part_2(include_str!("../../input.txt")), 55614);
}
