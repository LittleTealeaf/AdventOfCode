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
    let mut top_elves = [0; 3];

    let mut current = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(value) => current += value,
            Err(_) => {
                if current >= top_elves[0] {
                    top_elves[0] = current;
                    top_elves.sort();
                }
                current = 0;
            }
        }
    }
    if current >= top_elves[0] {
        top_elves[0] = current;
        top_elves.sort();
    }
    top_elves.into_iter().sum()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(include_str!("../../test.txt")), 24000);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(include_str!("../../test.txt")), 45000);
}
