mod part_1;
mod part_2;
mod utils;

use part_1::part_1;
use part_2::part_2;

fn main() {
    let number = include_str!("../../input.txt")
        .trim()
        .parse::<i64>()
        .unwrap();
    println!("Part 1: {}", part_1(number));
    println!("Part 2: {}", part_2(number));
}
