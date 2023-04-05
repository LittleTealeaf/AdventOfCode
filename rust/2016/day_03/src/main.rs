fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut values = line
                .split(" ")
                .filter_map(|line| line.parse::<i32>().ok())
                .collect::<Vec<_>>();
            values.sort();
            let long_edge = values.pop().unwrap();

            long_edge < values.into_iter().sum()
        })
        .count()
}

fn part_2(input: &str) -> usize {
    let mut sections = Vec::new();
    let mut iter = input.lines();

    while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
        sections.push(vec![a, b, c]);
    }

    sections
        .into_iter()
        .map(|section| {
            let values = section
                .into_iter()
                .map(|line| {
                    line.split(" ")
                        .filter_map(|line| line.parse::<i32>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            [
                [values[0][0], values[1][0], values[2][0]],
                [values[0][1], values[1][1], values[2][1]],
                [values[0][2], values[1][2], values[2][2]],
            ]
            .iter_mut()
            .map(|triangle| {
                triangle.sort();
                triangle[2] < triangle[1] + triangle[0]
            })
            .filter(|i| *i)
            .count()
        })
        .sum()
}
