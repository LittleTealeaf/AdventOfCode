fn main() {
    let input = parse_input(include_str!("../../input.txt"));

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn parse_input(input: &str) -> Vec<[i32; 3]> {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split('x');
            let mut arr = [
                tokens.next().unwrap().parse::<i32>().unwrap(),
                tokens.next().unwrap().parse::<i32>().unwrap(),
                tokens.next().unwrap().parse::<i32>().unwrap(),
            ];
            arr.sort();
            arr
        })
        .collect()
}

fn part_1(input: &Vec<[i32; 3]>) -> i32 {
    input
        .iter()
        .map(|[x, y, z]| x * y + 2 * (x * y + y * z + x * z))
        .sum()
}

fn part_2(input: &Vec<[i32; 3]>) -> i32 {
    input.iter().map(|[x, y, z]| 2 * (x + y) + x * y * z).sum()
}
