use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let content = read_to_string("input.txt").expect("Could not read input file");
    let mut chariter = content.chars();
    let mut buffer: VecDeque<char> = VecDeque::from([' '; 4]);
    let mut index = 1;

    'chars: loop {
        match (&mut chariter).next() {
            None => {
                index = -1;
                break;
            }
            Some(c) => {
                buffer.pop_front();
                buffer.push_back(c);

                if index < 4 {
                    index += 1;
                    continue;
                }

                for i in 0..4 {
                    for j in (i + 1)..4 {
                        if buffer.get(i).unwrap() == buffer.get(j).unwrap() {
                            index += 1;
                            continue 'chars;
                        }
                    }
                }
                break;
            }
        }
    }

    println!("Part 1: {}", index);
}

fn part_2() {
    let content = read_to_string("input.txt").expect("Could not read input file");
    let mut chariter = content.chars();
    let mut buffer: VecDeque<char> = VecDeque::from([' '; 14]);
    let mut index = 1;

    'chars: loop {
        match (&mut chariter).next() {
            None => {
                index = -1;
                break;
            }
            Some(c) => {
                buffer.pop_front();
                buffer.push_back(c);

                if index < 14 {
                    index += 1;
                    continue;
                }

                for i in 0..14 {
                    for j in (i + 1)..14 {
                        if buffer.get(i).unwrap() == buffer.get(j).unwrap() {
                            index += 1;
                            continue 'chars;
                        }
                    }
                }
                break;
            }
        }
    }

    println!("Part 2: {}", index);
}
