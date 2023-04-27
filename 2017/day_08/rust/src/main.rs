use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i64 {
    let mut registers: HashMap<&str, i64> = HashMap::new();

    let instructions = input.lines().map(str::trim);

    for instruction in instructions {
        let mut tokens = instruction.split(" ");
        let target_register = tokens.next().unwrap();
        let increase = tokens.next().unwrap().eq("inc");
        let amount: i64 = tokens.next().unwrap().parse().unwrap();
        tokens.next();
        let test_register = tokens.next().unwrap();
        let test_type = tokens.next().unwrap();
        let test_target = &tokens.next().unwrap().parse().unwrap();

        let test_value = registers.get(test_register).unwrap_or(&0);

        let test_result = match test_type {
            ">" => test_value > test_target,
            "<" => test_value < test_target,
            ">=" => test_value >= test_target,
            "<=" => test_value <= test_target,
            "==" => test_value == test_target,
            "!=" => test_value != test_target,
            _ => panic!(),
        };

        if test_result {
            let mut value = *registers.get(target_register).unwrap_or(&0);
            if increase {
                value += amount;
            } else {
                value -= amount;
            }
            registers.insert(target_register, value);
        }
    }

    registers
        .into_iter()
        .map(|(_, value)| value)
        .max()
        .unwrap_or(0)
}

fn part_2(input: &str) -> i64 {
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let mut highest_value = 0;

    let instructions = input.lines().map(str::trim);

    for instruction in instructions {
        let mut tokens = instruction.split(" ");
        let target_register = tokens.next().unwrap();
        let increase = tokens.next().unwrap().eq("inc");
        let amount: i64 = tokens.next().unwrap().parse().unwrap();
        tokens.next();
        let test_register = tokens.next().unwrap();
        let test_type = tokens.next().unwrap();
        let test_target = &tokens.next().unwrap().parse().unwrap();

        let test_value = registers.get(test_register).unwrap_or(&0);

        let test_result = match test_type {
            ">" => test_value > test_target,
            "<" => test_value < test_target,
            ">=" => test_value >= test_target,
            "<=" => test_value <= test_target,
            "==" => test_value == test_target,
            "!=" => test_value != test_target,
            _ => panic!(),
        };

        if test_result {
            let mut value = *registers.get(target_register).unwrap_or(&0);
            if increase {
                value += amount;
            } else {
                value -= amount;
            }
            if value > highest_value {
                highest_value = value;
            }
            registers.insert(target_register, value);
        }
    }

    highest_value
}

#[test]
fn test_part_1() {
    let input = include_str!("../../test.txt");
    assert_eq!(part_1(input), 1);
}

#[test]
fn test_part_2() {
    let input = include_str!("../../test.txt");
    assert_eq!(part_2(input), 10);
}
