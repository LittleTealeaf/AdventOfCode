pub fn part_2(input: &str) -> i32 {
    let chars = input.chars().collect::<Vec<_>>();
    let mut sum = 0;

    let len = chars.len();

    for (i, c) in chars.iter().enumerate() {
        if chars[(i + len / 2) % len] == *c {
            sum += c.to_string().parse::<i32>().unwrap();
        }
    }

    sum
}

#[test]
fn part_2_test_1() {
    assert_eq!(part_2("1212"), 6);
}

#[test]
fn part_2_test_2() {
    assert_eq!(part_2("1221"), 0);
}

#[test]
fn part_2_test_3() {
    assert_eq!(part_2("123425"), 4);
}

#[test]
fn part_2_test_4() {
    assert_eq!(part_2("123123"), 12);
}

#[test]
fn part_2_test_5() {
    assert_eq!(part_2("12131415"), 4);
}
