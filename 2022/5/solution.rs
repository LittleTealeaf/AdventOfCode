use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part_1();
}


fn part_1() {
    let file = File::open("input.txt").expect("");
    let lines = io::BufReader::new(file).lines();
    let mut line_iter = lines.into_iter().peekable();

    let first_line = line_iter.next();
    let cols = (first_line.unwrap().unwrap().len() + 1) / 4;

    let mut state: Vec<Vec<char>> = Vec::new();

    for _ in [..cols] {
        (&mut state).append(&mut Vec::new());
    }
    

    loop {
        if let Some(Ok(line)) = line_iter.next() {
            if line == "" {
                break;
            }

            if !line.starts_with("[") {
                continue;
            }

            println!("{}", line);

        } else {
           panic!("Did not find instructions"); 
        }
    }

    loop {
        if let Some(Ok(line)) = line_iter.next() {

        } else {
            break;
        }
    }
}
