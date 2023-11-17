use std::char;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

fn part_1(instructions: &Vec<Direction>) -> usize {
    let mut houses = vec![Coord { x: 0, y: 0 }];
    let mut x = 0;
    let mut y = 0;

    for dir in instructions {
        match dir {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x += 1,
            Direction::Right => x -= 1,
        }
        let house = Coord { x, y };
        if !houses.contains(&house) {
            houses.push(house);
        }
    }

    houses.len()
}

fn part_2(instructions: &Vec<Direction>) -> usize {
    let mut houses = vec![Coord { x: 0, y: 0 }];
    let mut santa = Coord { x: 0, y: 0 };
    let mut robot = Coord { x: 0, y: 0 };

    let mut iter = instructions.iter();

    while let (Some(santa_dir), Some(robot_dir)) = (iter.next(), iter.next()) {
        for (coord, dir) in [(&mut santa, santa_dir), (&mut robot, robot_dir)] {
            match dir {
                Direction::Up => coord.y += 1,
                Direction::Down => coord.y -= 1,
                Direction::Left => coord.x += 1,
                Direction::Right => coord.x -= 1,
            }
            let house = Coord {
                x: coord.x,
                y: coord.y,
            };
            if !houses.contains(&house) {
                houses.push(house);
            }
        }
    }

    houses.len()
}

fn main() {
    let instructions = include_str!("../../input.txt")
        .chars()
        .flat_map(Direction::try_from)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
}
