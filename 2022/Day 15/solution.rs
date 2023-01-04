use std::fs::File;
use std::io::{BufReader, BufRead};

const FILENAME: &str = "input.txt";

fn main() {
    part_1();
    part_2();
}


fn part_1() {
    let file = File::open(FILENAME).unwrap();
    let mut lines = BufReader::new(file).lines();




}

fn part_2() {
}
