use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";

#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
    Number(usize),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Array(a) => Value::Array(a.clone()),
            Value::Number(a) => Value::Number(a.clone()),
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
}
fn parse_signal(signal: &str) -> Value {
    if !signal.starts_with("[") {
        Value::Number(signal.parse().unwrap())
    } else {
        let mut items: Vec<Value> = Vec::new();
        let mut bfr: Vec<char> = Vec::new();
        let mut depth = 0;
        for chr in signal.chars() {
            if (depth == 1 && chr == ',') || (depth == 1 && chr == ']') {
                if bfr.len() > 0 {
                    items.push(parse_signal(&bfr.iter().collect::<String>()));
                    bfr.clear();
                } else {
                    items.push(Value::Array(Vec::new()));
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
        Value::Array(items)
    }
}

fn part_1_compare(a: &Value, b: &Value) -> Compare {
    match a {
        Value::Array(arr_a) => match b {
            Value::Array(arr_b) => {
                let l = min(arr_a.len(), arr_b.len());
                for i in 0..l {
                    let comp = part_1_compare(&arr_a[i], &arr_b[i]);
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
            Value::Number(num_b) => {
                if arr_a.len() == 0 {
                    Compare::Ordered
                } else {
                    match part_1_compare(&arr_a[0], b) {
                        Compare::Undetermined | Compare::Unordered => Compare::Unordered,
                        Compare::Ordered => Compare::Ordered,
                    }
                }
            }
        },
        Value::Number(num_a) => match b {
            Value::Array(arr_b) => {
                if arr_b.len() == 0 {
                    Compare::Unordered
                } else {
                    match part_1_compare(a, &arr_b[0]) {
                        Compare::Undetermined | Compare::Ordered => Compare::Ordered,
                        Compare::Unordered => Compare::Unordered,
                    }
                }
            }
            Value::Number(num_b) => {
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

                if let Compare::Ordered = part_1_compare(&a, &b) {
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
