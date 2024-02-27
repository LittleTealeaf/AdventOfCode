fn part_1(spreadsheet: &[Vec<i32>]) -> u32 {
    spreadsheet
        .iter()
        .map(|row| {
            row.iter()
                .max()
                .unwrap()
                .abs_diff(*row.iter().min().unwrap())
        })
        .sum()
}

fn part_2(spreadsheet: &[Vec<i32>]) -> i32 {
    spreadsheet
        .iter()
        .map(|row| {
            for i in 0..row.len() {
                for j in 0..row.len() {
                    if i != j && row[i] % row[j] == 0 {
                        return row[i] / row[j];
                    }
                }
            }
            0
        })
        .sum()
}

fn main() {
    let spreadsheet = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.split('\t')
                .filter_map(|token| token.trim().parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&spreadsheet));
    println!("Part 2: {}", part_2(&spreadsheet));
}
