use std::collections::HashSet;

fn main() {
    {
        let input = parse_input(include_str!("../../../../inputs/2015/03/input.txt").to_string());
        let result = part_1(input);
        println!("Part 1: {} houses", result);
    }
    {
        let input = parse_input(include_str!("../../../../inputs/2015/03/input.txt").to_string());
        let result = part_2(input);
        println!("Part 2: {} houses", result);
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_input(input: String) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|ch| -> Option<Direction> {
            match ch {
                '^' => Some(Direction::North),
                'v' => Some(Direction::South),
                '<' => Some(Direction::West),
                '>' => Some(Direction::East),
                _ => None,
            }
        })
        .collect()
}

#[derive(Hash, PartialEq, Eq)]
struct House {
    x: i32,
    y: i32,
}

fn part_1(input: Vec<Direction>) -> usize {
    let mut visited_houses = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    for direction in input {
        visited_houses.insert(House { x, y });
        match direction {
            Direction::North => y += 1,
            Direction::South => y -= 1,
            Direction::East => x += 1,
            Direction::West => x -= 1,
        }
    }

    visited_houses.len()
}

fn part_2(input: Vec<Direction>) -> usize {
    let mut visited_houses = HashSet::new();

    let mut x = [0; 2];
    let mut y = [0; 2];

    for (i, direction) in input.iter().enumerate() {
        visited_houses.insert(House {
            x: x[i % 2],
            y: y[i % 2],
        });
        match direction {
            Direction::North => y[i % 2] += 1,
            Direction::South => y[i % 2] -= 1,
            Direction::East => x[i % 2] += 1,
            Direction::West => x[i % 2] -= 1,
        }
    }

    visited_houses.len()
}
