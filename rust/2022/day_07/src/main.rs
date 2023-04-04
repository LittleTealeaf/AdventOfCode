use std::collections::HashMap;

enum CdDest {
    Dir(String),
    Back,
    Root,
}

enum PromptLine {
    Cd(CdDest),
    File(String, usize),
    Directory(String),
    Ls,
}

impl From<&str> for PromptLine {
    fn from(value: &str) -> Self {
        let tokens = value.split(" ").collect::<Vec<_>>();

        match tokens[0] {
            "$" => match tokens[1] {
                "ls" => PromptLine::Ls,
                "cd" => PromptLine::Cd(match tokens[2] {
                    "/" => CdDest::Root,
                    ".." => CdDest::Back,
                    dir => CdDest::Dir(dir.to_string()),
                }),
                _ => panic!(),
            },
            "dir" => PromptLine::Directory(tokens[1].to_string()),
            size_str => PromptLine::File(tokens[1].to_string(), size_str.parse().unwrap()),
        }
    }
}

fn main() {
    {
        let input = include_str!("../input.txt")
            .lines()
            .map(PromptLine::from)
            .collect();
        let result = part_1(input);
        println!("Part 1: {}", result);
    }
    {
        let input = include_str!("../input.txt")
            .lines()
            .map(PromptLine::from)
            .collect();
        let result = part_2(input);
        println!("Part 2: {}", result);
    }
}

fn part_1(input: Vec<PromptLine>) -> usize {
    let mut path = Vec::new();
    let mut directories: HashMap<String, usize> = HashMap::new();

    for step in input {
        match step {
            PromptLine::Cd(destination) => match destination {
                CdDest::Back => {
                    path.pop();
                }
                CdDest::Dir(name) => {
                    path.push(name);
                }
                CdDest::Root => {
                    path.clear();
                }
            },
            PromptLine::File(_, size) => {
                let path = path.clone();
                for i in 1..=path.len() {
                    let sub_path = &path[0..i];
                    let sub_path_str = sub_path.join("/");
                    let value = match directories.get(&sub_path_str) {
                        Some(value) => *value,
                        None => 0,
                    } + size;
                    directories.insert(sub_path_str, value);
                }
            }
            _ => {}
        }
    }

    directories
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| size <= &100000)
        .sum()
}

fn part_2(input: Vec<PromptLine>) -> usize {
    let mut path = Vec::new();
    let mut directories: HashMap<Vec<String>, usize> = HashMap::new();
    const TOTAL_SIZE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;

    for step in input {
        match step {
            PromptLine::Cd(destination) => match destination {
                CdDest::Back => {
                    path.pop();
                }
                CdDest::Dir(name) => {
                    path.push(name);
                }
                CdDest::Root => {
                    path.clear();
                }
            },
            PromptLine::File(_, size) => {
                let path = path.clone();
                for i in 0..=path.len() {
                    let sub_path = Vec::from(&path[0..i]);
                    let value = match directories.get(&sub_path) {
                        Some(value) => *value,
                        None => 0,
                    } + size;
                    directories.insert(sub_path, value);
                }
            }
            _ => {}
        }
    }

    let total_used_space = directories.get(&Vec::new()).unwrap();
    let total_usable_space = TOTAL_SIZE - NEEDED_SPACE;
    let min_space_to_free = total_used_space - total_usable_space;

    let mut large_enough_dirs = directories
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| size >= &min_space_to_free)
        .collect::<Vec<_>>();
    large_enough_dirs.sort();

    large_enough_dirs[0]
}

#[test]
fn test_part_1() {
    let input = include_str!("../example.txt")
        .lines()
        .map(PromptLine::from)
        .collect::<Vec<_>>();

    let result = part_1(input);
    assert_eq!(result, 95437);
}

#[test]
fn test_part_2() {
    let input = include_str!("../example.txt")
        .lines()
        .map(PromptLine::from)
        .collect::<Vec<_>>();

    let result = part_2(input);
    assert_eq!(result, 24933642);
}
