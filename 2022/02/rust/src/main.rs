fn main() {
    let input: Vec<&str> = include_str!("../../input.txt").lines().collect();

    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

fn part_1(lines: Vec<&str>) -> u32 {
    lines
        .into_iter()
        .map(|line| -> u32 {
            let mut chars = line.chars();
            let opponent = chars.next().unwrap() as u32 - 'A' as u32;
            let _ = chars.next();
            let player = chars.next().unwrap() as u32 - 'X' as u32;

            let decision_points = {
                match (opponent, player) {
                    (0, 1) | (1, 2) | (2, 0) => 6,
                    (0, 0) | (1, 1) | (2, 2) => 3,
                    (_, _) => 0,
                }
            };

            decision_points + player + 1
        })
        .sum::<u32>()
}

fn part_2(lines: Vec<&str>) -> u32 {
    lines
        .into_iter()
        .map(|line| -> u32 {
            let mut chars = line.chars();
            let opponent = chars.next().unwrap() as u32 - 'A' as u32;
            let _ = chars.next();
            let outcome = chars.next().unwrap() as u32 - 'X' as u32;

            let decision_points = outcome * 3;

            let move_points = match (opponent, outcome) {
                (0, 1) | (1, 0) | (2, 2) => 1,
                (0, 2) | (1, 1) | (2, 0) => 2,
                _ => 3,
            };

            decision_points + move_points
        })
        .sum::<u32>()
}

#[test]
fn test_part_1() {
    let input: Vec<&str> = include_str!("../../test.txt").lines().collect();

    assert_eq!(part_1(input), 15);
}

#[test]
fn test_part_2() {
    let input: Vec<&str> = include_str!("../../test.txt").lines().collect();

    assert_eq!(part_2(input), 12);
}
