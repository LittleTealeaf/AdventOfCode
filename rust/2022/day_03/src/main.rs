fn get_input() -> Vec<String> {
    include_str!("../input.txt")
        .split("\n")
        .map(String::from)
        .filter(|a| a.len() > 0)
        .collect()
}

fn main() {
    println!("Part 1: {}", part_1(get_input()));
    println!("Part 2: {}", part_2(get_input()));
}

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn part_1(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|rucksack| -> char {
            let len = rucksack.len();
            let first_half = &rucksack[0..len / 2];
            let second_half = &rucksack[len / 2..];

            for c1 in first_half.chars() {
                for c2 in second_half.chars() {
                    if c1 == c2 {
                        return c1;
                    }
                }
            }
            panic!(
                "There are no common values in the rucksack \"{}\"",
                rucksack
            );
        })
        .map(get_priority)
        .sum()
}

fn part_2(input: Vec<String>) -> u32 {
    let mut iterator = input.into_iter();
    let mut groups = Vec::new();
    while let (Some(a), Some(b), Some(c)) = (iterator.next(), iterator.next(), iterator.next()) {
        groups.push((a, b, c));
    }

    groups
        .into_iter()
        .map(|(a, b, c)| {
            for ca in a.chars() {
                for cb in b.chars() {
                    if ca == cb {
                        for cc in c.chars() {
                            if ca == cc {
                                return ca;
                            }
                        }
                    }
                }
            }
            panic!("There is no group badge");
        })
        .map(get_priority)
        .sum()
}
