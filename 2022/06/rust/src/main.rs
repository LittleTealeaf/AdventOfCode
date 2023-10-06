fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let binding = input.chars().map(|i| i as usize).collect::<Vec<_>>();
    let windows = &binding[..].windows(4);

    windows
        .clone()
        .enumerate()
        .find_map(|(i, slice)| {
            for i in 0..slice.len() {
                for j in (i + 1)..(slice.len()) {
                    if slice[i] == slice[j] {
                        return None;
                    }
                }
            }

            Some(i)
        })
        .unwrap()
        + 4
}

fn part_2(input: &str) -> usize {
    let binding = input.chars().map(|i| i as usize).collect::<Vec<_>>();
    let windows = &binding[..].windows(14);

    windows
        .clone()
        .enumerate()
        .find_map(|(i, slice)| {
            for i in 0..slice.len() {
                for j in (i + 1)..(slice.len()) {
                    if slice[i] == slice[j] {
                        return None;
                    }
                }
            }

            Some(i)
        })
        .unwrap()
        + 14
}
