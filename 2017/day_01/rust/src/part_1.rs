pub fn part_1(input: &str) -> i32 {
    let chars = input.chars().collect::<Vec<_>>();
    let mut sum = 0;

    for (i, c) in chars.iter().enumerate() {
        if chars[(i + 1) % chars.len()] == *c {
            sum += c.to_string().parse::<i32>().unwrap();
        }
    }

    sum
}

#[test]
fn part_1_test_1() {
    assert_eq!(part_1("1122"), 3);
}

#[test]
fn part_1_test_2() {
    assert_eq!(part_1("1111"), 4);
}

#[test]
fn part_1_test_3() {
    assert_eq!(part_1("1234"), 0);
}

#[test]
fn part_1_test_4() {
    assert_eq!(part_1("91212129"), 9);
}
