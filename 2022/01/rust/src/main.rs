fn main() {
    println!("Part 1: {}", part_1(include_str!("../../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../../input.txt")));
}

fn part_1(input: &str) -> u32 {
    let mut max = 0;
    let mut current = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(value) => current += value,
            Err(_) => {
                if current > max {
                    max = current;
                }
                current = 0;
            }
        }
    }
    max
}

fn part_2(input: &str) -> u32 {
    let mut elfs = input
        .split("\n\n")
        .map(|line| {
            line.lines()
                .filter_map(|l| l.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    elfs.sort();
    elfs.reverse();
    return elfs[0] + elfs[1] + elfs[2];
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(include_str!("../../test.txt")), 24000);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(include_str!("../../test.txt")), 45000);
}
