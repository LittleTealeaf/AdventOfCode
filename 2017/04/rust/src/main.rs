fn main() {
    let phrases = include_str!("../../input.txt")
        .lines()
        .map(|line| line.split(" ").map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&phrases));
    println!("Part 2: {}", part_2(&phrases));
}

fn part_1(phrases: &Vec<Vec<String>>) -> usize {
    phrases
        .into_iter()
        .filter(|phrase| {
            let mut tmp = Vec::new();

            for word in *phrase {
                if tmp.contains(&word) {
                    return false;
                } else {
                    tmp.push(word);
                }
            }
            return true;
        })
        .count()
}

fn part_2(phrases: &Vec<Vec<String>>) -> usize {
    let phrases = phrases
        .clone()
        .into_iter()
        .map(|phrase| {
            phrase
                .into_iter()
                .map(|word| {
                    let mut chars = word.chars().collect::<Vec<_>>();
                    chars.sort();
                    chars.into_iter().collect()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part_1(&phrases)
}
