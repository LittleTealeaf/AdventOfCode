fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(parse_input(input)));
    println!("Part 2: {}", part_2(parse_input(input)));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part_1(input: Vec<i32>) -> usize {
    let mut input = input.clone();
    let mut index = 0;
    let mut count = 0;

    loop {
        count += 1;
        input[index as usize] += 1;
        index += input[index as usize] - 1;

        if index < 0 || index >= input.len() as i32 {
            break;
        }
    }

    count
}

fn part_2(input: Vec<i32>) -> usize {
    let mut input = input.clone();
    let mut index = 0;
    let mut count = 0;

    loop {
        count += 1;
        let offset = input[index as usize];
        input[index as usize] += if offset >= 3 { -1 } else { 1 };
        index += offset;

        if index < 0 || index >= input.len() as i32 {
            break;
        }
    }

    count
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(vec![0, 3, 0, 1, -3]), 5);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(vec![0, 3, 0, 1, -3]), 10);
}
