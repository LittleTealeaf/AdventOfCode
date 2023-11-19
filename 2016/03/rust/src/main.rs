fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut values = line
                .trim()
                .split(" ")
                .filter_map(|i| i.parse::<i32>().ok())
                .collect::<Vec<_>>();
            values.sort();
            values[0] + values[1] > values[2]
        })
        .count()
}

fn part_2(input: &str) -> usize {
    let mut lines = input
        .lines()
        .map(|line| line.trim().split(" ").filter_map(|i| i.parse::<i32>().ok()));
    let mut count = 0;
    while let (Some(mut a), Some(mut b), Some(mut c)) = (lines.next(), lines.next(), lines.next()) {
        while let (Some(a_i), Some(b_i), Some(c_i)) = (a.next(), b.next(), c.next()) {
            let mut values = vec![a_i, b_i, c_i];
            values.sort();
            if values[0] + values[1] > values[2] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../../input.txt")));
}
