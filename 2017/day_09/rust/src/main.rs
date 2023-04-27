fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let mut depth = 0;
    let mut score = 0;
    let mut garbage = false;

    let mut iter = input.trim().chars().into_iter();

    while let Some(c) = iter.next() {
        if c == '!' {
            iter.next();
        } else if garbage {
            if c == '>' {
                garbage = false;
            }
        } else if c == '<' {
            garbage = true;
        } else if c == '{' {
            depth += 1;
        } else if c == '}' {
            score += depth;
            depth -= 1;
        }
    }

    score
}

fn part_2(input: &str) -> i32 {
    let mut depth = 0;
    let mut garbage_count = 0;
    let mut garbage = false;

    let mut iter = input.trim().chars().into_iter();

    while let Some(c) = iter.next() {
        if c == '!' {
            iter.next();
        } else if garbage {
            if c == '>' {
                garbage = false;
            } else {
                garbage_count += 1;
            }
        } else if c == '<' {
            garbage = true;
        } else if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
        }
    }

    garbage_count
}

#[test]
fn test_part_1() {
    let test_cases = [
        ("{}", 1),
        ("{{{}}}", 6),
        ("{{},{}}", 5),
        ("{{{},{},{{}}}}", 16),
        ("{<a>,<a>,<a>,<a>}", 1),
        ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
        ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
        ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
    ];

    for (input, expected) in test_cases {
        let result = part_1(input);
        assert_eq!(
            result, expected,
            "Test Failed for '{}', Expected {} but found {}",
            input, result, expected
        );
    }
}

#[test]
fn test_part_2() {
    let test_cases = [
        ("<>", 0),
        ("<random characters>", 17),
        ("<<<<>", 3),
        ("<{!>}>", 2),
        ("<!!>", 0),
        ("<!!!>>", 0),
        ("<{o\"i!a,<{i<a>", 10),
    ];

    for (input, expected) in test_cases {
        let result = part_2(input);
        assert_eq!(
            result, expected,
            "Test Failed for '{}', Expected {} but found {}",
            input, result, expected
        );
    }
}
