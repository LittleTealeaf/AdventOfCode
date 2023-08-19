fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<i32>>();

    println!("Part 1: {}", input.iter().map(|i| part_1(*i)).sum::<i32>());
    println!("Part 2: {}", input.iter().map(|i| part_2(*i)).sum::<i32>());
}

fn part_1(mass: i32) -> i32 {
    mass / 3 - 2
}

fn part_2(mass: i32) -> i32 {
    let mut total = 0;
    let mut mass = part_1(mass);
    while mass > 0 {
        total += mass;
        mass = part_1(mass);
    }
    total
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(12), 2);
    assert_eq!(part_1(14), 2);
    assert_eq!(part_1(1969), 654);
    assert_eq!(part_1(100756), 33583);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(14), 2);
    assert_eq!(part_2(1969), 966);
    assert_eq!(part_2(100756), 50346);
}
