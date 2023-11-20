fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input.clone()));
}

fn part_1(mut input: Vec<i32>) -> usize {
    let mut steps = 0;
    let mut i = 0;
    let len = input.len();

    while let Some(jump) = input.get_mut(if i >= 0 { i as usize } else { len }) {
        steps += 1;
        i += *jump;
        *jump += 1;
    }

    steps
}

fn part_2(mut input: Vec<i32>) -> usize {
    let mut steps = 0;
    let mut i = 0;
    let len = input.len();

    while let Some(jump) = input.get_mut(if i >= 0 { i as usize } else { len }) {
        steps += 1;
        i += *jump;
        if *jump >= 3 {
            *jump -= 1;
        } else {
            *jump += 1;
        }
    }

    steps
}
