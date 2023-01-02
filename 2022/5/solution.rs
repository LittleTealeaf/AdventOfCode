use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input.txt").expect("");
    let lines = io::BufReader::new(file).lines();
    let mut line_iter = lines.into_iter().peekable();

    let first_line = line_iter.next().unwrap().unwrap();
    let cols = (first_line.len() + 1) / 4;

    let mut state: Vec<Vec<char>> = vec![Vec::new(); cols];

    {
        let mut chars = first_line.chars();
        chars.next();
        for i in 0..cols {
            if let Some(c) = (&mut chars).next() {
                if c != ' ' {
                    (&mut state)[i].push(c);
                }
            }
            chars.nth(2);
        }
    }

    loop {
        if let Some(Ok(line)) = line_iter.next() {
            if line == "" {
                break;
            }

            if !line.starts_with("[") {
                continue;
            }

            let mut chars = line.chars();

            chars.next();
            for i in 0..cols {
                if let Some(c) = (&mut chars).next() {
                    if c != ' ' {
                        (&mut state)[i].insert(0, c);
                    }
                }
                chars.nth(2);
            }
        } else {
            panic!("Did not find instructions");
        }
    }

    loop {
        if let Some(Ok(line)) = line_iter.next() {
            let words: Vec<&str> = line.split(" ").collect();
            let count: usize = words[1].parse().unwrap();
            let start: usize = words[3].parse().unwrap();
            let end: usize = words[5].parse().unwrap();

            let state = &mut state;

            for _ in 0..count {
                if let Some(val) = state[start - 1].pop() {
                    (&mut state[end - 1]).push(val);
                }
            }
        } else {
            break;
        }
    }

    let mut solution: Vec<char> = Vec::new();

    for i in 0..cols {
        solution.push(*(&mut state)[i].last().unwrap());
    }
    let s: String = solution.iter().collect();

    println!("Part 1 Solution: {}", s);
}

fn part_2() {
    let file = File::open("input.txt").expect("");
    let lines = io::BufReader::new(file).lines();
    let mut line_iter = lines.into_iter().peekable();

    let first_line = line_iter.next().unwrap().unwrap();
    let cols = (first_line.len() + 1) / 4;

    let mut state: Vec<Vec<char>> = vec![Vec::new(); cols];

    {
        let mut chars = first_line.chars();
        chars.next();
        for i in 0..cols {
            if let Some(c) = (&mut chars).next() {
                if c != ' ' {
                    (&mut state)[i].push(c);
                }
            }
            chars.nth(2);
        }
    }

    loop {
        if let Some(Ok(line)) = line_iter.next() {
            if line == "" {
                break;
            }

            if !line.starts_with("[") {
                continue;
            }

            let mut chars = line.chars();

            chars.next();
            for i in 0..cols {
                if let Some(c) = (&mut chars).next() {
                    if c != ' ' {
                        (&mut state)[i].insert(0, c);
                    }
                }
                chars.nth(2);
            }
        } else {
            panic!("Did not find instructions");
        }
    }

    loop {
        if let Some(Ok(line)) = line_iter.next() {
            let words: Vec<&str> = line.split(" ").collect();
            let count: usize = words[1].parse().unwrap();
            let start: usize = words[3].parse().unwrap();
            let end: usize = words[5].parse().unwrap();

            let state = &mut state;

            let mut vals: Vec<char>  = Vec::new(); 

            for _ in 0..count {
                if let Some(c) = state[start - 1].pop() {
                    (&mut vals).push(c);
                }
            }

            for _ in 0..count {
                if let Some(c) = vals.pop() {
                    (&mut state[end - 1]).push(c);
                }
            }


        } else {
            break;
        }
    }

    let mut solution: Vec<char> = Vec::new();

    for i in 0..cols {
        solution.push(*(&mut state)[i].last().unwrap());
    }
    let s: String = solution.iter().collect();

    println!("Part 2 Solution: {}", s);
}
