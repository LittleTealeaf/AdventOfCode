use std::cmp::Ordering;

fn main() {
    let values = include_str!("../../input.txt")
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&values));
    println!("Part 2: {}", part_2(&values));
}

fn part_1(values: &Vec<i32>) -> i32 {
    let mut values = values.clone();
    values.sort();
    let mut a = 0;
    let mut b = values.len() - 1;

    loop {
        match (values[a] + values[b]).cmp(&2020) {
            Ordering::Less => {
                a += 1;
            }
            Ordering::Equal => {
                break;
            }
            Ordering::Greater => {
                b -= 1;
            }
        }
    }
    values[a] * values[b]
}

fn part_2(values: &Vec<i32>) -> i32 {
    for i in 0..values.len() - 2 {
        for j in i + 1..values.len() - 1 {
            for k in j + 1..values.len() {
                if values[i] + values[k] + values[j] == 2020 {
                    return values[i] * values[k] * values[j];
                }
            }
        }
    }
    panic!()
}
