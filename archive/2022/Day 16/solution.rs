use std::io::{BufReader, BufRead};
use std::fs::File;

const FILENAME: &str = "sample.txt";

fn main() {
    part_1();
    part_2();
}


struct Valve {
    flow_rate: usize,
    name: &str,
    exits: Vec<&str>,
    index_exits: Vec<usize>
}


fn part_1() {
    let file = File::open(FILENAME).unwrap();
    let lines = BufReader::new(file).lines();
}

