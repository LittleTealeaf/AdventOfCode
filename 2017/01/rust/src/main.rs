fn part_1(values: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..values.len() {
        if values[i] == values[(i + 1) % values.len()] {
            sum += values[i];
        }
    }

    sum
}

fn part_2(values: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..values.len() {
        if values[i] == values[(i + values.len() / 2) % values.len()] {
            sum += values[i];
        }
    }

    sum
}

fn main() {
    let values = include_str!("../../input.txt")
        .chars()
        .filter_map(|char| char.to_digit(10))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&values));
    println!("Part 2: {}", part_2(&values));
}
