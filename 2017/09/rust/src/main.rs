fn main() {
    println!("Part 1: {}", part_1(include_str!("../../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../../input.txt")));
}

fn part_1(chars: &str) -> i32 {
    let mut depth = 0;
    let mut garbage = false;
    let mut score = 0;
    let mut iter = chars.chars();

    while let Some(c) = iter.next() {
        if c == '!' {
            iter.next();
        } else if garbage {
            if c == '>' {
                garbage = false;
            }
        } else if c == '{' {
            depth += 1;
        } else if c == '}' {
            score += depth;
            depth -= 1;
        } else if c == '<' {
            garbage = true;
        }
    }

    score
}

fn part_2(chars: &str) -> i32 {
    let mut garbage = false;
    let mut count = 0;
    let mut iter = chars.chars();

    while let Some(c) = iter.next() {
        if c == '!' {
            iter.next();
        } else if garbage {
            if c == '>' {
                garbage = false;
            } else {
                count += 1;
            }
        } else if c == '<' {
            garbage = true;
        }
    }

    count
}
