pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut values = line
                .split("\t")
                .map(str::parse::<i32>)
                .filter_map(Result::ok)
                .collect::<Vec<_>>();
            values.sort();

            values.last().unwrap() - values.first().unwrap()
        })
        .sum()
}

#[test]
fn part_1_test() {
    let lines = ["5\t1\t9\t5", "7\t5\t3", "2\t4\t6\t8"].join("\n");
    assert_eq!(part_1(&lines), 18);
}
