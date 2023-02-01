use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
};

fn is_line_nice(line: &String) -> bool {
    let mut vowel_count = 0;
    let mut double_count = 0;
    let mut previous_letter = None;

    for c in line.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
            }
            _ => {}
        }

        if let Some(previous) = previous_letter {
            if previous == c {
                double_count += 1;
            }

            if match previous {
                'a' => c == 'b',
                'c' => c == 'd',
                'p' => c == 'q',
                'x' => c == 'y',
                _ => false,
            } {
                return false;
            }
        }

        previous_letter = Some(c);
    }

    vowel_count >= 3 && double_count > 0
}

fn part_1(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .filter(|line| is_line_nice(line))
        .count()
        .try_into()
        .unwrap()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn main() {
    {

        assert!(is_line_nice(&String::from("ugknbfddgicrmopn")));
        assert!(is_line_nice(&String::from("aaa")));
        assert!(!is_line_nice(&String::from("jchzalrnumimnmhp")));
        assert!(!is_line_nice(&String::from("haegwjzuvuyypxyu")));
        assert!(!is_line_nice(&String::from("dvszwmarrgswjxmb")));


        let lines = read_file("input.txt");
        let result = part_1(lines);
        println!("Part 1: {}", result);
    }
}
