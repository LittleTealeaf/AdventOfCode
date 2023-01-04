use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Tile {
    Rock,
    Air,
    Sand,
}

fn main() {
    part_1();
    part_2();
}

fn print_map(map: &Vec<Vec<Tile>>) {
    for row in map {
        let mut string: Vec<char> = Vec::new();
        for tile in row {
            string.push(match tile {
                Tile::Rock => '#',
                Tile::Air => '.',
                Tile::Sand => 'o',
            });
        }
        println!("{:?}", string);
    }
}

fn parse_scan(file: File, map: &mut Vec<Vec<Tile>>) -> usize {
    let mut lines = BufReader::new(file).lines();

    let mut scans: Vec<Vec<Point>> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let mut points: Vec<Point> = line
                .split(" -> ")
                .map(|string| -> Point {
                    let coords: Vec<&str> = string.split(",").collect();
                    Point {
                        x: coords[0].parse().unwrap(),
                        y: coords[1].parse().unwrap(),
                    }
                })
                .collect();

            (&mut scans).push(points);
        }
    }

    let mut x_min = std::usize::MAX;
    let mut x_max = std::usize::MIN;
    let mut y_max = std::usize::MIN;

    for scan in &scans {
        for point in scan {
            if x_min > point.x {
                x_min = point.x;
            }
            if x_max < point.x {
                x_max = point.x;
            }
            if y_max < point.y {
                y_max = point.y;
            }
        }
    }

    x_min -= 2;
    x_max += 2;
    y_max += 2;

    //let mut map: Vec<Vec<Tile>> = Vec::new();

    for _ in 0..y_max {
        let mut row: Vec<Tile> = Vec::new();
        for _ in 0..(x_max - x_min) {
            row.push(Tile::Air);
        }
        map.push(row);
    }

    for scan in &scans {
        let mut scan_iter = scan.iter();
        let tile = scan_iter.next().unwrap();
        let mut x = tile.x;
        let mut y = tile.y;
        map[y][x - x_min] = Tile::Rock;

        for point in scan_iter {
            while x != point.x || y != point.y {
                map[y][x - x_min] = Tile::Rock;

                if x < point.x {
                    x += 1;
                } else if x > point.x {
                    x -= 1;
                } else if y < point.y {
                    y += 1;
                } else if y > point.y {
                    y -= 1;
                }
            }
            map[y][x - x_min] = Tile::Rock;
        }
    }

    x_min
}

fn part_1() {
    let file = File::open(FILENAME).expect("");
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let x_min = parse_scan(file, &mut map);
    let y_max = map.len();

    let drop_x = 500 - x_min;
    let mut count = 0;

    'sand: loop {
        let mut x = drop_x;
        let mut y = 0;


        'fall: while let Some(row_below) = (&map).get(y + 1) {
            match row_below[x] {
                Tile::Air => {
                    y += 1;
                    continue 'fall;
                },
                _ => {
                    match row_below[(x as isize - 1) as usize] {
                        Tile::Air => {
                            y += 1;
                            x -= 1;
                            continue 'fall;
                        },
                        _ => {
                            match row_below[x + 1] {
                                Tile::Air => {
                                    x += 1;
                                    y += 1;
                                    continue 'fall;
                                },
                                _ => {
                                    break 'fall;
                                }
                            }
                        }
                    }
                }
            }
        }


        if y + 1 == y_max  {
            break 'sand;
        }

        (&mut map)[y][x] = Tile::Sand;


        count += 1;
    }

    println!("Part 1: {} units", count);
}

fn part_2() {}
