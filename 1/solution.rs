use std::io::{self, BufRead};

fn main() {
    let mut empty: &[u8] = &[];
    let mut buffer = String::new();

    let bytes = empty.read_line(&mut buffer);

}
