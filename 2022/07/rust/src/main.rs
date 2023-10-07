use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    let mut sizes = HashMap::<String, u32>::new();

    let mut directory = vec![""];

    for line in input.lines() {
        let mut tokens = line.split(" ");

        match tokens.next() {
            Some("$") => match tokens.next() {
                Some("cd") => match tokens.next() {
                    Some("..") => {
                        directory.pop();
                    }
                    Some("/") => {
                        directory.clear();
                        directory.push("");
                    }
                    Some(path) => {
                        directory.push(path);
                    }
                    None => panic!(),
                },
                _ => {}
            },
            Some(value) => {
                if let Ok(num) = value.parse::<u32>() {
                    directory.push(tokens.next().unwrap());

                    let final_path = directory
                        .iter()
                        .map(|i| String::from(*i))
                        .collect::<Vec<_>>()
                        .join("/");

                    if !sizes.contains_key(&final_path) {
                        for i in 0..directory.len() {
                            let path = directory[..i]
                                .iter()
                                .map(|i| String::from(*i))
                                .collect::<Vec<_>>()
                                .join("/");

                            sizes.insert(path.clone(), *sizes.get(&path).unwrap_or(&0) + num);
                        }
                    }

                    directory.pop();
                }
            }
            None => {}
        }
    }

    sizes
        .values()
        .into_iter()
        .filter(|value| *value <= &100000)
        .sum()
}

fn part_2(input: &str) -> u32 {
    let mut system = HashMap::<String, u32>::new();

    let mut dir = vec!["".to_string()];

    for line in input.lines() {
        let mut tokens = line.split(" ");

        match tokens.next() {
            Some("$") => match tokens.next() {
                Some("cd") => match tokens.next() {
                    Some("..") => {
                        dir.pop();
                    }
                    Some("/") => {
                        dir.clear();
                        dir.push("".to_string());
                    }
                    Some(path) => {
                        dir.push(path.to_string());
                    }
                    None => panic!(),
                },
                _ => {}
            },
            Some(value) => {
                if let Ok(num) = value.parse::<u32>() {
                    let filename = String::from(tokens.next().unwrap()) + "?";
                    dir.push(filename);

                    let final_path = dir.iter().cloned().collect::<Vec<_>>().join("/");

                    if !system.contains_key(&final_path) {
                        for i in 0..dir.len() {
                            let path = dir[..i + 1].iter().cloned().collect::<Vec<_>>().join("/");

                            system.insert(path.clone(), *system.get(&path).unwrap_or(&0) + num);
                        }
                    }

                    dir.pop();
                }
            }
            None => {}
        }
    }

    let max_size = 70000000;
    let used_space = *system.get(&String::from("")).unwrap();
    let free_space = max_size - used_space;
    let needed_space = 30000000;
    let min_space_to_free = if needed_space > free_space {
        needed_space - free_space
    } else {
        0
    };

    system
        .into_iter()
        .filter_map(|(key, value)| {
            (!key.contains("?") && value >= min_space_to_free).then_some(value)
        })
        .min()
        .unwrap()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(include_str!("../../test.txt")), 95437);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(include_str!("../../test.txt")), 24933642);
}
