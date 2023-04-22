use part_1::part_1;


mod part_1;
mod part_2;

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
}
