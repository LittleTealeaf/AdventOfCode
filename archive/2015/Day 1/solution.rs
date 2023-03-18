use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn main() {
    part_1();
    part_2();
}

fn part_1() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let mut lines = BufReader::new(file).lines();
    let data = lines.next().unwrap()?;
    let chars = data.chars();

    let mut floor = 0;
    for c in chars {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }
    }

    println!("Part 1: Floor {}", floor);

    Ok(())
}


fn part_2() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let mut lines = BufReader::new(file).lines();
    let data = lines.next().unwrap()?;
    let chars = data.chars();

    let mut floor = 0;
    let mut count = 0;
    for c in chars {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }

        count += 1;

        if floor == -1 {
            break;
        }

    }

    println!("Part 2: Index {}", count);

    Ok(())
}
