use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    println!("Part 1: {} houses", part_1().unwrap());
    println!("Part 2: {} houses", part_2().unwrap());
}

#[derive(Hash, Eq)]
struct House {
    x: i32,
    y: i32,
}

impl PartialEq for House {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn part_1() -> Result<usize, Error> {
    let file = File::open("input.txt")?;
    let line = BufReader::new(file).lines().next().unwrap()?;
    let chars = line.chars();

    let mut houses = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for c in chars {
        houses.insert(House { x, y });

        match c {
            '>' => {
                x += 1;
            }
            '<' => {
                x -= 1;
            }
            '^' => {
                y += 1;
            }
            'v' => {
                y -= 1;
            }
            _ => {}
        }
    }
    houses.insert(House { x, y });

    Ok(houses.len())
}

fn part_2() -> Result<usize, Error> {
    let file = File::open("input.txt")?;
    let line = BufReader::new(file).lines().next().unwrap()?;
    let chars = line.chars();

    let mut houses = HashSet::new();
    houses.insert(House { x: 0, y: 0 });
    let mut x = [0; 2];
    let mut y = [0; 2];
    let mut count = 0;

    for c in chars {
        match c {
            '>' => {
                x[count % 2] += 1;
            }
            '<' => {
                x[count % 2] -= 1;
            }
            '^' => {
                y[count % 2] += 1;
            }
            'v' => {
                y[count % 2] -= 1;
            }
            _ => {}
        }
        houses.insert(House {
            x: x[count % 2],
            y: y[count % 2],
        });
        count += 1;
    }

    Ok(houses.len())
}
