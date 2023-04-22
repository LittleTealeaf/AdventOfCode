pub fn part_1(input: &str) -> i32 {
    input.lines().map(calculate_square_feet).sum()
}

fn calculate_square_feet(line: &str) -> i32 {
    let mut numbers = line
        .split("x")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    numbers.sort();

    3 * numbers[0] * numbers[1] + 2 * numbers[1] * numbers[2] + 2 * numbers[2] * numbers[0]
}

#[test]
fn test_calculate_square_feet() {
    assert_eq!(calculate_square_feet("2x3x4"), 58);
    assert_eq!(calculate_square_feet("1x1x10"), 43);
}
