use std::fs;

enum Direction {
    Up,
    Down,
}

struct InvalidChar;

impl TryFrom<char> for Direction {
    type Error = InvalidChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Up),
            ')' => Ok(Self::Down),
            _ => Err(InvalidChar),
        }
    }
}

fn part_1(instructions: Vec<Direction>) -> i32 {
    let mut floor = 0;
    for instruction in instructions {
        floor += match instruction {
            Direction::Up => 1,
            Direction::Down => -1,
        }
    }

    floor
}

fn part_2(instructions: Vec<Direction>) -> usize {
    let mut floor = 0;
    for (index, instruction) in instructions.into_iter().enumerate() {
        floor += match instruction {
            Direction::Up => 1,
            Direction::Down => -1,
        };

        if floor == -1 {
            return index;
        }
    }

    0
}

fn read_input() -> Vec<Direction> {
    fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .into_iter()
        .map(Direction::try_from)
        .filter_map(Result::ok)
        .collect()
}

fn main() {
    {
        let input = read_input();
        let result = part_1(input);
        println!("Part 1: {}", result);
    }
    {
        let input = read_input();
        let result = part_2(input);
        println!("Part 2: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_cases = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input_text, expected_value) in test_cases {
            let input = input_text
                .chars()
                .into_iter()
                .map(Direction::try_from)
                .filter_map(Result::ok)
                .collect();
            let value = part_1(input);
            assert_eq!(value, expected_value);
        }
    }
}
