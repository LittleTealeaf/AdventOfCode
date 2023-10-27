fn main() {
    println!("Part 1: {}", part_1(include_str!("../../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../../input.txt")));
}

fn part_1(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut window: i32 = lines.next().unwrap().parse().unwrap();
    let mut count = 0;

    for line in lines {
        let num = line.parse().unwrap();
        if num > window {
            count += 1;
        }

        window = num;
    }

    count
}

fn part_2(input: &str) -> i32 {
    let values = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut count = 0;

    for i in 3..values.len() {
        if values[i - 3..i].iter().sum::<i32>() < values[i - 2..=i].iter().sum::<i32>() {
            count += 1;
        }
    }

    count
}
