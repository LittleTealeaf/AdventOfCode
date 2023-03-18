use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_1_filter(line: &String) -> bool {
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


fn part_2_filter(line: &String) -> bool {
    let arr: Vec<char> = line.chars().collect();
    let mut has_sandwich = false;
    let mut has_pair = false;

    for i in 2..arr.len() {
        if arr[i-2] == arr[i] {
            has_sandwich = true;
        }

        if i > 2 {
            for j in 1..=(i-2) {
                if arr[j] == arr[i] && arr[j-1] == arr[i-1] {
                    has_pair = true;
                }
            }
        }

        if has_pair && has_sandwich {
            return true;
        }
    }


    false
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
        assert!(part_1_filter(&String::from("ugknbfddgicrmopn")));
        assert!(part_1_filter(&String::from("aaa")));
        assert!(!part_1_filter(&String::from("jchzalrnumimnmhp")));
        assert!(!part_1_filter(&String::from("haegwjzuvuyypxyu")));
        assert!(!part_1_filter(&String::from("dvszwmarrgswjxmb")));

        let lines = read_file("input.txt");
        let result = lines.iter().filter(|line| part_1_filter(line)).count();
        println!("Part 1: {}", result);
    }

    {
        assert!(part_2_filter(&String::from("qjhvhtzxzqqjkmpb")));
        assert!(part_2_filter(&String::from("xxyxx")));
        assert!(!part_2_filter(&String::from("uurcxstgmygtbstg")));
        assert!(!part_2_filter(&String::from("ieodomkazucvgmuy")));

        let lines = read_file("input.txt");
        let result = lines.iter().filter(|line| part_2_filter(line)).count();
        println!("Part 2: {}", result);
    }
}
