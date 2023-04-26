use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(parse_input(input)));
    println!("Part 2: {}", part_2(parse_input(input)));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split("\t")
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part_1(values: Vec<i32>) -> usize {
    let mut set = HashSet::new();
    let mut values = values.clone();

    while set.insert(values.clone()) {
        let largest_memory = values.iter().max().unwrap();
        let mut index = 0;
        for i in 0..values.len() {
            if values[i] == *largest_memory {
                index = i;
                break;
            }
        }

        let mut count = values[index];
        values[index] = 0;
        while count > 0 {
            index = (index + 1) % values.len();
            values[index] += 1;
            count -= 1;
        }
    }

    set.len()
}

fn part_2(values: Vec<i32>) -> usize {
    let mut hashes = Vec::new();
    let mut values = values.clone();

    while {
        let hash = {
            let mut hasher = DefaultHasher::new();
            values.hash(&mut hasher);
            hasher.finish()
        };
        let contains = hashes.contains(&hash);
        hashes.push(hash);
        !contains
    } {
        let largest_memory = values.iter().max().unwrap();
        let mut index = 0;
        for i in 0..values.len() {
            if values[i] == *largest_memory {
                index = i;
                break;
            }
        }

        let mut count = values[index];
        values[index] = 0;
        while count > 0 {
            index = (index + 1) % values.len();
            values[index] += 1;
            count -= 1;
        }
    }

    let repeated = hashes.pop().unwrap();
    let mut count = 0;
    while let Some(hash) = hashes.pop() {
        if hash == repeated {
            break;
        }

        count += 1;
    }

    count + 1
}

#[test]
fn test_part_1() {
    let values = vec![0, 2, 7, 0];
    assert_eq!(part_1(values), 5)
}

#[test]
fn test_part_2() {
    let values = vec![0, 2, 7, 0];
    assert_eq!(part_2(values), 4)
}
