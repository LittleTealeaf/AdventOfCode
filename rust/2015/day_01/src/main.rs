use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    {
        let file = File::open("input.txt").unwrap();
        let result = part_1(file);
        println!("Part 1: {} calories", result);
    }

    {
        let file = File::open("input.txt").unwrap();
        let result = part_2(file);
        println!("Part 2: {} calories", result);
    }
}

fn part_1(file: File) -> u32 {
    let lines = BufReader::new(file).lines();

    let mut highest = 0;
    let mut current = 0;

    for line in lines {
        if let Ok(line) = line {
            match line.parse::<u32>() {
                Ok(value) => {
                    current += value;
                    if current > highest {
                        highest = current;
                    }
                }
                Err(_) => {
                    current = 0;
                }
            }
        }
    }

    highest
}

fn part_2(file: File) -> u32 {
    let lines = BufReader::new(file).lines();

    let mut heap = BinaryHeap::new();
    let mut current = 0;

    for line in lines {
        if let Ok(line) = line {
            match line.parse::<u32>() {
                Ok(value) => {
                    current += value;
                }
                Err(_) => {
                    heap.push(current);
                    current = 0;
                }
            }
        }
    }
    heap.push(current);

    let mut sum = 0;
    for _ in 0..3 {
        sum += heap.pop().unwrap();
    }

    sum
}

#[test]
fn test_part_1() {
    let file = File::open("sample.txt").unwrap();
    let value = part_1(file);
    assert_eq!(value, 24000);
}

#[test]
fn test_part_2() {
    let file = File::open("sample.txt").unwrap();
    let value = part_2(file);
    assert_eq!(value, 45000);
}
