fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> String {
    let lines = input.lines();
    let mut position = 5;
    let mut code = Vec::new();

    for line in lines.into_iter() {
        for c in line.chars() {
            match c {
                'D' => {
                    if position <= 6 {
                        position += 3;
                    }
                }
                'U' => {
                    if position > 3 {
                        position -= 3;
                    }
                }
                'L' => {
                    if position % 3 != 1 {
                        position -= 1;
                    }
                }
                'R' => {
                    if position % 3 != 0 {
                        position += 1;
                    }
                }
                _ => panic!(),
            }
        }

        code.push(position);
    }

    code.into_iter().map(|digit| digit.to_string()).collect()
}

#[test]
fn test_part_1() {
    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let value = part_1(input);
    assert_eq!(value, "1985");
}

fn part_2(input: &str) -> String {
    let lines = input.lines();
    let mut code = Vec::new();
    const KEYPAD: [[Option<char>; 5]; 5] = [
        [None, None, Some('1'), None, None],
        [None, Some('2'), Some('3'), Some('4'), None],
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        [None, Some('A'), Some('B'), Some('C'), None],
        [None, None, Some('D'), None, None],
    ];

    let mut x = 0;
    let mut y = 2;

    for line in lines {
        for c in line.chars() {
            let prev_x = x;
            let prev_y = y;
            match c {
                'R' => x += 1,
                'U' => {
                    if y > 0 {
                        y -= 1
                    }
                }
                'D' => y += 1,
                'L' => {
                    if x > 0 {
                        x -= 1
                    }
                }
                _ => panic!(),
            }

            let valid = match KEYPAD.get(y) {
                Some(row) => match row.get(x) {
                    Some(item) => match item {
                        Some(_) => true,
                        None => false,
                    },
                    None => false,
                },
                None => false,
            };

            if !valid {
                x = prev_x;
                y = prev_y;
            }
        }

        code.push(KEYPAD[y][x].unwrap())
    }

    code.into_iter().collect()
}

#[test]
fn test_part_2() {
    let input = "ULL\nRRDDD\nLURDL\nUUUUD";
    let value = part_2(input);
    assert_eq!(value, "5DB3");
}
