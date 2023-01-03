use std::cmp::{min, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";

#[derive(Debug)]
enum Signal {
    Array(Vec<Signal>),
    Number(usize),
}

impl Signal {
    fn to_string() -> String {
        match self {
            Signal::Array(vec) => {
                unimplemented!("Formats");
            },
            Signal::Number(num) => usize::to_string(&num)
        }
    }
}

impl Clone for Signal {
    fn clone(&self) -> Self {
        match self {
            Signal::Array(a) => Signal::Array(a.clone()),
            Signal::Number(a) => Signal::Number(a.clone()),
        }
    }
}

#[derive(Debug)]
enum Compare {
    Ordered,
    Unordered,
    Undetermined,
}

fn main() {
    part_1();
    part_2();
}

fn parse_signal(signal: &str) -> Signal {
    if !signal.starts_with("[") {
        Signal::Number(signal.parse().unwrap())
    } else {
        let mut items: Vec<Signal> = Vec::new();
        let mut bfr: Vec<char> = Vec::new();
        let mut depth = 0;
        for chr in signal.chars() {
            if (depth == 1 && chr == ',') || (depth == 1 && chr == ']') {
                if bfr.len() > 0 {
                    items.push(parse_signal(&bfr.iter().collect::<String>()));
                    bfr.clear();
                } else {
                    items.push(Signal::Array(Vec::new()));
                }
            } else if depth != 0 {
                bfr.push(chr);
            }

            if chr == '[' {
                depth += 1;
            } else if chr == ']' {
                depth -= 1;
            }
        }
        Signal::Array(items)
    }
}

fn compare_signals(a: &Signal, b: &Signal) -> Compare {
    match a {
        Signal::Array(arr_a) => match b {
            Signal::Array(arr_b) => {
                let l = min(arr_a.len(), arr_b.len());
                for i in 0..l {
                    let comp = compare_signals(&arr_a[i], &arr_b[i]);
                    if match comp {
                        Compare::Undetermined => false,
                        _ => true,
                    } {
                        return comp;
                    }
                }
                if arr_a.len() == arr_b.len() {
                    Compare::Undetermined
                } else if arr_a.len() == l {
                    Compare::Ordered
                } else {
                    Compare::Unordered
                }
            }
            Signal::Number(_) => {
                if arr_a.len() == 0 {
                    Compare::Ordered
                } else {
                    match compare_signals(&arr_a[0], b) {
                        Compare::Undetermined | Compare::Unordered => Compare::Unordered,
                        Compare::Ordered => Compare::Ordered,
                    }
                }
            }
        },
        Signal::Number(num_a) => match b {
            Signal::Array(arr_b) => {
                if arr_b.len() == 0 {
                    Compare::Unordered
                } else {
                    match compare_signals(a, &arr_b[0]) {
                        Compare::Undetermined | Compare::Ordered => Compare::Ordered,
                        Compare::Unordered => Compare::Unordered,
                    }
                }
            }
            Signal::Number(num_b) => {
                if num_a == num_b {
                    Compare::Undetermined
                } else if num_a < num_b {
                    Compare::Ordered
                } else {
                    Compare::Unordered
                }
            }
        },
    }
}

fn part_1() {
    let file = File::open(FILENAME).expect("");
    let mut lines = BufReader::new(file).lines();
    let mut index = 1;
    let mut sum = 0;

    loop {
        if let Some(Ok(line_a)) = lines.next() {
            if let Some(Ok(line_b)) = lines.next() {
                let a = parse_signal(&line_a);
                let b = parse_signal(&line_b);

                if let Compare::Ordered = compare_signals(&a, &b) {
                    sum += index;
                }
                index += 1;

                if let Some(_) = lines.next() {
                    continue;
                }
            }
        }
        break;
    }

    println!("Part 1: {}", sum);
}

fn part_2() {
    let file = File::open(FILENAME).expect("");

    let mut signals: Vec<Signal> = Vec::new();
    signals.push(parse_signal("[[2]]"));
    signals.push(parse_signal("[[6]]"));

    for line_result in BufReader::new(file).lines() {
        if let Ok(line) = line_result {
            if line.len() > 0 {
                signals.push(parse_signal(&line));
            }
        }
    }

    
    signals.sort_by(|a, b| -> Ordering {
        match compare_signals(&a, &b) {
            Compare::Ordered => Ordering::Less,
            Compare::Unordered => Ordering::Greater,
            Compare::Undetermined => Ordering::Equal
        }
    });

    let mut index_2 = -1;
    let mut index_6 = -1;
    
    for i in 0..signals.len() {
        let sig = signals[i];
    }
}
