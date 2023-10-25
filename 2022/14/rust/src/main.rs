use map::{Map, Tile};

mod map;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../../input.txt")));
}

fn part_1(input: &str) -> i32 {
    let mut map = Map::from_input(input);
    // let mut grid = build_grid(&input);
    let mut sand_count = 0;

    let mut x = 500;
    let mut y = 0;

    while y < map.get_height() + 1 {
        if matches!(map.get(x, y + 1), Tile::Empty) {
            y += 1;
        } else if matches!(map.get(x - 1, y + 1), Tile::Empty) {
            y += 1;
            x -= 1;
        } else if matches!(map.get(x + 1, y + 1), Tile::Empty) {
            y += 1;
            x += 1;
        } else {
            sand_count += 1;
            map.set(x, y, Tile::Sand);
            x = 500;
            y = 0;
        }
    }

    sand_count
}

fn part_2(input: &str) -> i32 {
    let mut map = Map::from_input(input);
    let mut sand_count = 0;

    let mut x = 500;
    let mut y = 0;

    while matches!(map.get(500, 0), Tile::Empty) {
        if y == map.get_height() + 1 {
            sand_count += 1;
            map.set(x, y, Tile::Sand);
            x = 500;
            y = 0;
        } else if matches!(map.get(x, y + 1), Tile::Empty) {
            y += 1;
        } else if x > 0 && matches!(map.get(x - 1, y + 1), Tile::Empty) {
            y += 1;
            x -= 1;
        } else if x < 999 && matches!(map.get(x + 1, y + 1), Tile::Empty) {
            y += 1;
            x += 1;
        } else {
            sand_count += 1;
            map.set(x, y, Tile::Sand);
            x = 500;
            y = 0;
        }
    }

    sand_count
}
