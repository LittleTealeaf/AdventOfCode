use std::collections::HashSet;

fn main() {
    {
        let value = part_1(include_str!("../input.txt").replace("\n", "").to_string());
        println!("Part 1: {}", value);
    }
    {
        let value = part_2(include_str!("../input.txt").replace("\n", "").to_string());
        println!("Part 2: {}", value);
    }
}

fn part_1(input: String) -> i32 {
    let tokens = input.split(", ");
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;

    for token in tokens {
        direction += match &token[0..=0] {
            "R" => 1,
            "L" => -1,
            _ => panic!(),
        };

        if direction == -1 {
            direction = 3;
        } else if direction == 4 {
            direction = 0;
        }

        if let Ok(value) = token[1..].parse::<i32>() {
            match direction {
                0 => y += value,
                1 => x += value,
                2 => y -= value,
                3 => x -= value,
                val => panic!("Invalid Direction: {}", val),
            }
        }
    }

    i32::abs(x) + i32::abs(y)
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(String::from("R2, L3")), 5);
    assert_eq!(part_1(String::from("R2, R2, R2")), 2);
    assert_eq!(part_1(String::from("R5, L5, R5, R3")), 12);
}

fn part_2(input: String) -> i32 {
    let tokens = input.split(", ");
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;

    let mut locations = HashSet::new();

    for token in tokens {
        direction += match &token[0..=0] {
            "R" => 1,
            "L" => -1,
            _ => panic!(),
        };

        if direction == -1 {
            direction = 3;
        } else if direction == 4 {
            direction = 0;
        }

        if let Ok(value) = token[1..].parse::<i32>() {
            for _ in 0..value {
                match direction {
                    0 => y += 1,
                    1 => x += 1,
                    2 => y -= 1,
                    3 => x -= 1,
                    val => panic!("Invalid Direction: {}", val),
                }

                let current_location = (x, y);

                if locations.contains(&current_location) {
                    return i32::abs(x) + i32::abs(y);
                } else {
                    locations.insert(current_location);
                }
            }
        }
    }

    -1
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(String::from("R8, R4, R4, R8")), 4);
}
