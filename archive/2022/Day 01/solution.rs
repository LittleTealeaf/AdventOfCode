use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut max = 0;
        let mut calories = 0;

        for line in lines {
            if let Ok(line) = line {
                if line == String::from("") {
                    if max < calories {
                        max = calories;
                    }
                    calories = 0;
                } else {
                    let val: i32 = line.parse().expect("Invalid number");
                    calories += val;
                }
            }
        }

        if max < calories {
            max = calories;
        }

        println!("Part 1: Max Calories is {}", max);
    }
}

fn part_2() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut leaderboard: Vec<i32> = Vec::new();
        let mut calories = 0;

        for line in lines {
            if let Ok(line) = line {
                if line == String::from("") {
                    leaderboard.push(calories);
                    calories = 0;
                } else {
                    let val: i32 = line.parse().expect("Invalid number");
                    calories += val;
                }
            }
        }

        leaderboard.push(calories);

        leaderboard.sort();
        leaderboard.reverse();
        let sum: i32 = leaderboard[..3].iter().sum();

        println!("Part 2: Sum of top 3 is {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
