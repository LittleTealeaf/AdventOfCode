fn main() {
    let coords = parse_input(include_str!("../../input.txt"));
    println!("Part 1: {}", part_1(&coords));
    println!("Part 2: {}", part_2(&coords));
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split(",");
            Coord {
                x: tokens.next().unwrap().parse().unwrap(),
                y: tokens.next().unwrap().parse().unwrap(),
                z: tokens.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: &Vec<Coord>) -> usize {
    let max_x = input.iter().map(|c| c.x).max().unwrap();
    let max_y = input.iter().map(|c| c.y).max().unwrap();
    let max_z = input.iter().map(|c| c.z).max().unwrap();

    let mut grid = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];

    for coord in input {
        grid[coord.x][coord.y][coord.z] = true;
    }

    let mut count = 0;

    for coord in input {
        if coord.x > 0 && grid[coord.x - 1][coord.y][coord.z] {
            count += 1;
        }
        if coord.x < max_x && grid[coord.x + 1][coord.y][coord.z] {
            count += 1;
        }
        if coord.y > 0 && grid[coord.x][coord.y - 1][coord.z] {
            count += 1;
        }
        if coord.y < max_y && grid[coord.x][coord.y + 1][coord.z] {
            count += 1;
        }
        if coord.z > 0 && grid[coord.x][coord.y][coord.z - 1] {
            count += 1;
        }
        if coord.z < max_z && grid[coord.x][coord.y][coord.z + 1] {
            count += 1;
        }
    }

    input.len() * 6 - count
}

#[test]
fn test_part_1() {
    let coords = parse_input(include_str!("../../sample.txt"));
    let solution = part_1(&coords);

    assert_eq!(solution, 64);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cube {
    Empty,
    Filled,
    Visited,
}

impl Coord {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

fn part_2(input: &Vec<Coord>) -> usize {
    let input = input
        .iter()
        .map(|coord| Coord {
            x: coord.x + 1,
            y: coord.y + 1,
            z: coord.z + 1,
        })
        .clone()
        .collect::<Vec<_>>();

    let max_x = input.iter().map(|c| c.x).max().unwrap();
    let max_y = input.iter().map(|c| c.y).max().unwrap();
    let max_z = input.iter().map(|c| c.z).max().unwrap();

    let mut grid = vec![vec![vec![Cube::Empty; max_z + 2]; max_y + 2]; max_x + 2];

    for coord in input {
        grid[coord.x][coord.y][coord.z] = Cube::Filled;
    }

    let mut frontier = vec![Coord::new(0, 0, 0)];

    let mut surface_area = 0;

    while let Some(Coord { x, y, z }) = frontier.pop() {
        if matches!(grid[x][y][z], Cube::Visited) {
            continue;
        }
        grid[x][y][z] = Cube::Visited;
        let directions = [
            (x > 0).then(|| Coord::new(x - 1, y, z)),
            (x < max_x + 1).then(|| Coord::new(x + 1, y, z)),
            (y > 0).then(|| Coord::new(x, y - 1, z)),
            (y < max_y + 1).then(|| Coord::new(x, y + 1, z)),
            (z > 0).then(|| Coord::new(x, y, z - 1)),
            (z < max_z + 1).then(|| Coord::new(x, y, z + 1)),
        ]
        .into_iter()
        .flatten();

        for coord in directions {
            match grid[coord.x][coord.y][coord.z] {
                Cube::Empty => {
                    frontier.push(coord);
                }
                Cube::Filled => {
                    surface_area += 1;
                }
                Cube::Visited => {}
            }
        }
    }

    surface_area
}

#[test]
fn test_part_2() {
    let coords = parse_input(include_str!("../../sample.txt"));
    let solution = part_2(&coords);
    assert_eq!(solution, 58);
}
