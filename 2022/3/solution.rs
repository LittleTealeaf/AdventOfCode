use std::fs::File;
use std::io::{self, BufRead};

/*
fn read_input() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines());
}
*/

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input.txt").expect("");
    let lines = io::BufReader::new(file).lines();

    let mut total = 0;

    for line in lines {
        match line {
            Err(_) => panic!("Error"),
            Ok(line) => {
                let a = &line[..line.len()/2].as_bytes();
                let b = &line[line.len()/2..].as_bytes();

                let mut letter: u8 = 0; 

                for ch in b.iter() {
                    if a.contains(ch) {
                        letter = *ch;
                    }
                }

                if letter < 'a' as u8 {
                    total += letter as i32 - 'A' as i32 + 27;
                } else {
                    total += letter as i32 -'a' as i32 + 1;
                }
            } 
        }
    }

    println!("Part 1: Total is {total}");
}

fn part_2() {}
