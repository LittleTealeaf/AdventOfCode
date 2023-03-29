use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    {
        let file = File::open("input.txt").unwrap();
        let value = part_1(file);
        println!("Part 1: {} square feet", value);
    }

    {
        let file = File::open("input.txt").unwrap();
        let value = part_2(file);
        println!("Part 2: {} feet", value);
    }
}

fn part_1(file: File) -> u32 {
    BufReader::new(file)
        .lines()
        .into_iter()
        .map(Result::unwrap)
        .map(|line| -> u32 {
            let mut vals = line
                .split("x")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            vals.sort();
            3 * vals[0] * vals[1] + 2 * vals[1] * vals[2] + 2 * vals[2] * vals[0]
        })
        .sum::<u32>()
}

fn part_2(file: File) -> u32 {
    BufReader::new(file)
        .lines()
        .into_iter()
        .map(Result::unwrap)
        .map(|line| -> u32 {
            let mut vals = line
                .split("x")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            vals.sort();
            (vals[0] + vals[1]) * 2 + vals[0] * vals[1] * vals[2]
        })
        .sum::<u32>()
}
