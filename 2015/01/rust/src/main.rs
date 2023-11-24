use std::panic;

fn main() {
    let input = include_str!("../../input.txt").trim();

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let mut floor = 0;

    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            ch => panic!("Invalid Char <{}>", ch),
        };
    }
    floor
}

fn part_2(input: &str) -> usize {
    let mut floor = 0;

    for (index, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            ch => panic!("Invalid Char <{}>", ch),
        };

        if floor == -1 {
            return index + 1;
        }
    }
    panic!()
}
