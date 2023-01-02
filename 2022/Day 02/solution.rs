use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut score: i32 = 0;

        for line in lines {
            if let Ok(line) = line {
                let mut iter = line.as_bytes().iter();
                let a = *(iter.next().expect("Expected Value")) as i32 - 'A' as i32;
                iter.next();
                let b = *(iter.next().expect("Expected Value")) as i32 - 'X' as i32;

                score += b + 1;

                if (b - a) % 3 == 1 || (b - a) % 3 == -2 {
                    score += 6;
                } else if b == a {
                    score += 3;
                }
            }
        }

        println!("Part 1: Total score is {score}");
    }
}

fn part_2() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut score: i32 = 0;

        for line in lines {
            if let Ok(line) = line {
                let mut iter = line.as_bytes().iter();
                let a = *(iter.next().expect("")) as i32 - 'A' as i32;
                iter.next();
                let b = *(iter.next().expect("")) as i32 - 'X' as i32;

                score += b * 3
                    + if b == 0 {
                        (a + 2) % 3 + 1
                    } else if b == 1 {
                        a + 1
                    } else {
                        (a + 1) % 3 + 1
                    };
            }
        }

        println!("Part 2: Total score is {score}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
