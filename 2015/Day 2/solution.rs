use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let solution_a = part_1().unwrap();
    println!("Part 1: {} square feet", solution_a);

    let solution_b = part_2().unwrap();
    println!("Part 2: {} feet of ribbon", solution_b);
}

fn part_1() -> Result<i64, Error> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut wrapping_paper_area: i64 = 0;
    for line in lines {
        if let Ok(line) = line {
            let dims: Vec<i64> = line
                .split("x")
                .map(|x| -> i64 { x.parse().unwrap() })
                .collect();
            let areas = [dims[0] * dims[1], dims[1] * dims[2], dims[2] * dims[0]];
            let min_area: i64 = min(min(areas[0],areas[1]),areas[2]);
            let total_area: i64 = areas.iter().sum();

            wrapping_paper_area += min_area + total_area * 2;
        }

    }

    Ok(wrapping_paper_area)
}


fn part_2() -> Result<i64, Error> {

    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut total_ribbon: i64 = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut dims: Vec<i64> = line
                .split("x")
                .map(|x| -> i64 { x.parse().unwrap() })
                .collect();
            dims.sort();

            total_ribbon += dims[0] * 2 + dims[1] * 2 + dims[0] * dims[1] * dims[2];
        }

    }

    Ok(total_ribbon)
}
