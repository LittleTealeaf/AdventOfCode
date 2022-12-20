use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
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

        println!("Max Calories is {}", max);

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
