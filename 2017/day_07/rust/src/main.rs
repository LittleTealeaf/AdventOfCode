use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> String {
    let mut parents = HashSet::new();
    let mut children = HashSet::new();
    let input = input.replace(",", "").replace("-> ", "");

    let lines = input.lines().map(str::trim).map(|l| l.split(" "));

    for line in lines {
        let mut line = line.clone();
        let disc_name = line.next().unwrap();
        if !children.contains(&disc_name) {
            parents.insert(disc_name);
        }

        let _ = line.next();

        while let Some(value) = line.next() {
            parents.remove(value);
            children.insert(value);
        }
    }

    parents.into_iter().next().unwrap().to_string()
}

#[test]
fn test_part_1() {
    let input = include_str!("../../test.txt");

    assert_eq!(part_1(input), String::from("tknk"));
}

fn part_2(input: &str) -> i32 {
    // let input = input.replace(",", "").replace("-> ", "").replace;
    let input = input
        .replace(",", "")
        .replace("-> ", "")
        .replace(")", "")
        .replace("(", "");

    let mut map: HashMap<&str, (i32, i32)> = HashMap::new();

    let mut lines = input.lines().map(str::trim).collect::<VecDeque<_>>();

    'next_line: while let Some(line) = lines.pop_front() {
        let mut tokens = line.split(" ");
        let disc_name = tokens.next().unwrap();
        let size = tokens.next().unwrap().parse::<i32>().unwrap();

        let mut children = Vec::new();

        while let Some(value) = tokens.next() {
            match map.get(value) {
                Some(value) => {
                    children.push(*value);
                }
                None => {
                    lines.push_back(line);
                    continue 'next_line;
                }
            }
        }

        if children.len() > 1 {
            let target_value = {
                let first = children[0].0 + children[0].1;
                let second = children[1].0 + children[1].1;
                if first == second || first == children[2].0 + children[2].1 {
                    first
                } else {
                    second
                }
            };

            for (a, b) in children.iter() {
                if a + b != target_value {
                    return target_value - b;
                }
            }
        }

        map.insert(
            disc_name,
            (size, children.into_iter().map(|(a, b)| a + b).sum()),
        );
    }

    todo!()
}

#[test]
fn test_part_2() {
    let input = include_str!("../../test.txt");

    assert_eq!(part_2(input), 60);
}
