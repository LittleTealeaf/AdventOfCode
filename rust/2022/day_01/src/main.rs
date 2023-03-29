type InputType = Vec<Vec<u32>>;

fn parse_input() -> InputType {
    // let a = include_str!("../input.txt").split("\n\n").collect::<Vec<_>>();
    // println!("{:?}", a);
    include_str!("../input.txt")
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .filter(|a| a.len() > 0)
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn main() {
    println!("Part 1: {}", part_1(parse_input()));
    println!("Part 2: {}", part_2(parse_input()));
}

fn part_1(input: InputType) -> u32 {
    input
        .into_iter()
        .map(|elf| elf.into_iter().sum())
        .max()
        .unwrap()
}

fn part_2(input: InputType) -> u32 {
    let mut elves = input
        .into_iter()
        .map(|elf| elf.into_iter().sum())
        .collect::<Vec<u32>>();
    elves.sort();

    elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap()
}
