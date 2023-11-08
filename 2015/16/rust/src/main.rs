use std::collections::HashMap;

fn constraints() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert(String::from("children"), 3);
    map.insert(String::from("cats"), 7);
    map.insert(String::from("samoyeds"), 2);
    map.insert(String::from("pomeranians"), 3);
    map.insert(String::from("akitas"), 0);
    map.insert(String::from("vizslas"), 0);
    map.insert(String::from("goldfish"), 5);
    map.insert(String::from("trees"), 3);
    map.insert(String::from("cars"), 2);
    map.insert(String::from("perfumes"), 1);
    map
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../../input.txt")));
}

fn part_1(input: &str) -> i32 {
    let constraints = constraints();

    input
        .lines()
        .find_map(|line| {
            let (_, line) = line.split_once(" ").unwrap();
            let (num, line) = line.split_once(": ").unwrap();

            for token in line.split(", ") {
                let (label, num) = token.split_once(": ").unwrap();
                let num = num.parse::<i32>().unwrap();
                if let Some(constr) = constraints.get(&label.to_string()) {
                    if num != *constr {
                        return None;
                    }
                }
            }
            Some(num.parse::<i32>().unwrap())
        })
        .unwrap()
}

fn part_2(input: &str) -> i32 {
    let constraints = constraints();

    input
        .lines()
        .find_map(|line| {
            let (_, line) = line.split_once(" ").unwrap();
            let (num, line) = line.split_once(": ").unwrap();

            for token in line.split(", ") {
                let (label, num) = token.split_once(": ").unwrap();
                let num = num.parse::<i32>().unwrap();
                if let Some(constr) = constraints.get(&label.to_string()) {
                    match label {
                        "cats" | "trees" => {
                            if num <= *constr {
                                return None;
                            }
                        }
                        "pomeranians" | "goldfish" => {
                            if num >= *constr {
                                return None;
                            }
                        }
                        _ => {
                            if num != *constr {
                                return None;
                            }
                        }
                    }
                }
            }
            Some(num.parse::<i32>().unwrap())
        })
        .unwrap()
}
