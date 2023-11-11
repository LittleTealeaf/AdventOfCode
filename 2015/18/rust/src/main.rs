fn main() {
    println!(
        "Part 1: {}",
        part_1(parse_input(include_str!("../../input.txt")))
    );
    println!(
        "Part 2: {}",
        part_2(parse_input(include_str!("../../input.txt")))
    );
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|char| char == '#').collect())
        .collect()
}

fn part_1(input: Vec<Vec<bool>>) -> usize {
    let mut state = input.clone();

    for _ in 0..100 {
        let mut new_state = state.clone();
        for i in 0..100 {
            for j in 0..100 {
                let count = [
                    (i > 0).then(|| state[i - 1][j]),
                    (j > 0).then(|| state[i][j - 1]),
                    (i > 0 && j > 0).then(|| state[i - 1][j - 1]),
                    (i < 99).then(|| state[i + 1][j]),
                    (j < 99).then(|| state[i][j + 1]),
                    (i > 0 && j < 99).then(|| state[i - 1][j + 1]),
                    (j > 0 && i < 99).then(|| state[i + 1][j - 1]),
                    (i < 99 && j < 99).then(|| state[i + 1][j + 1]),
                ]
                .into_iter()
                .flatten()
                .filter(|i| *i)
                .count();

                if state[i][j] {
                    new_state[i][j] = count == 2 || count == 3;
                } else {
                    new_state[i][j] = count == 3;
                }
            }
        }
        state = new_state;
    }

    state
        .into_iter()
        .map(|row| row.into_iter().filter(|i| *i).count())
        .sum()
}

fn part_2(input: Vec<Vec<bool>>) -> usize {
    let mut state = input.clone();

    for _ in 0..100 {
        let mut new_state = state.clone();
        for i in 0..100 {
            for j in 0..100 {
                let count = [
                    (i > 0).then(|| state[i - 1][j]),
                    (j > 0).then(|| state[i][j - 1]),
                    (i > 0 && j > 0).then(|| state[i - 1][j - 1]),
                    (i < 99).then(|| state[i + 1][j]),
                    (j < 99).then(|| state[i][j + 1]),
                    (i > 0 && j < 99).then(|| state[i - 1][j + 1]),
                    (j > 0 && i < 99).then(|| state[i + 1][j - 1]),
                    (i < 99 && j < 99).then(|| state[i + 1][j + 1]),
                ]
                .into_iter()
                .flatten()
                .filter(|i| *i)
                .count();

                if state[i][j] {
                    new_state[i][j] = count == 2 || count == 3;
                } else {
                    new_state[i][j] = count == 3;
                }
            }
        }
        new_state[0][0] = true;
        new_state[0][99] = true;
        new_state[99][0] = true;
        new_state[99][99] = true;
        state = new_state;
    }

    state
        .into_iter()
        .map(|row| row.into_iter().filter(|i| *i).count())
        .sum()
}
