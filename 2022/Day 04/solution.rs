use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("input.txt").expect("");
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;

    for line in lines {
        if let Ok(line) = line {
            let line_segments: Vec<Vec<u64>> = line
                .split(",")
                .map(|val: &str| -> Vec<u64> {
                    val.split("-")
                        .map(|n| -> u64 { n.parse().expect("Invalid") })
                        .collect()
                })
                .collect();
            let (inner, outer) = {
                if line_segments[0][1] - line_segments[0][0]
                    < line_segments[1][1] - line_segments[1][0]
                {
                    (&line_segments[0], &line_segments[1])
                } else {
                    (&line_segments[1], &line_segments[0])
                }
            };

            if outer[0] <= inner[0] && inner[1] <= outer[1] {
                count += 1;
            }
        }
    }

    println!("Part 1: Count is {}", count);
}

fn part_2() {
    let file = File::open("input.txt").expect("");
    let lines = io::BufReader::new(file).lines();

    let mut count = 0;

    for line in lines {
        if let Ok(line) = line {
            let line_segments: Vec<Vec<u64>> = line
                .split(",")
                .map(|val: &str| -> Vec<u64> {
                    val.split("-")
                        .map(|n| -> u64 { n.parse().expect("Invalid") })
                        .collect()
                })
                .collect();
            let (first, second) = {
                if line_segments[0][0] < line_segments[1][0] {
                    (&line_segments[0], &line_segments[1])
                } else {
                    (&line_segments[1], &line_segments[0])
                }
            };

            if first[1] >= second[0] {
                count += 1;
            }
        }
    }

    println!("Part 2: Count is {}", count);
}
