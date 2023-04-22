use part_1::part_1;

use crate::part_2::part_2;

mod part_1;
mod part_2;

fn main() {
    let input = {
        let mut chars = include_str!("../../input.txt").chars();
        chars.next_back();
        chars.as_str()
    };
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}
