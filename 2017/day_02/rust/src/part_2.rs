pub fn part_2(input: &str) -> i32 {
    input.lines().map(calc_row).sum()
}

fn calc_row(input: &str) -> i32 {
    let mut tokens = input.split("\t").map(str::parse::<i32>).filter_map(Result::ok).collect::<Vec<_>>();
    tokens.sort();
    tokens.reverse();

    for (i, v) in tokens.iter().enumerate() {
        for j in (i+1)..(tokens.len()) {
            if v % tokens[j] == 0 {
                return v / tokens[j];
            }
        }
    }


    return -1;
}

#[test]
fn test_part_2() {
    let values = ["5\t9\t2\t8", "9\t4\t7\t3", "3\t8\t6\t5"].join("\n");

    assert_eq!(part_2(&values), 9);
}

#[test]
fn test_calc_row_1() {
    assert_eq!(calc_row("5\t9\t2\t8"), 4);
}

#[test]
fn test_calc_row_2() {
    assert_eq!(calc_row("9\t4\t7\t3"), 3);
}

#[test]
fn test_calc_row_3() {
    assert_eq!(calc_row("3\t8\t6\t5"), 2);
}
