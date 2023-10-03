fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut ranges = line
                .split(",")
                .map(|range| {
                    let mut nums = range.split("-").map(|i| i.parse::<i32>().unwrap());
                    (nums.next().unwrap(), nums.next().unwrap())
                })
                .collect::<Vec<_>>();
            if ranges[1].1 - ranges[1].0 > ranges[0].1 - ranges[0].0 {
                ranges.reverse();
            }

            ranges[0].0 <= ranges[1].0 && ranges[0].1 >= ranges[1].1
        })
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut ranges = line
                .split(",")
                .map(|range| {
                    let mut nums = range.split("-").map(|i| i.parse::<i32>().unwrap());
                    (nums.next().unwrap(), nums.next().unwrap())
                })
                .collect::<Vec<_>>();

            if ranges[0].0 > ranges[1].0 {
                ranges.reverse();
            }

            ranges[0].1 >= ranges[1].0
        })
        .count()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(include_str!("../../example.txt")), 2);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(include_str!("../../example.txt")), 4)
}
