pub fn part_2(input: &str) -> usize {
    let mut floor = 0;
    for (index, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            return index + 1;
        }
    }

    0
}


#[test]
fn test_part_2() {
    assert_eq!(part_2(")"), 1);
    assert_eq!(part_2("()())"), 5);
}
