type InputType = Vec<(u32, u32, u32, u32)>;

fn get_input() -> InputType {
    include_str!("../input.txt")
        .lines()
        .map(|entry| entry.split_once(",").unwrap())
        .map(|(l, r)| {
            let ((l1, l2), (r1, r2)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
            (
                l1.parse().unwrap(),
                l2.parse().unwrap(),
                r1.parse().unwrap(),
                r2.parse().unwrap(),
            )
        })
        .collect()
}

fn main() {
    println!("Part 1: {}", part_1(get_input()));
    println!("Part 2: {}", part_2(get_input()));
}

fn part_1(input: InputType) -> usize {
    input
        .into_iter()
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count()
}

fn part_2(input: InputType) -> usize {
    input
        .into_iter()
        .filter(|(a, b, c, d)| a <= d && c <= b)
        .count()
}
