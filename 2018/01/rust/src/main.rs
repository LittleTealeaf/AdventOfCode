fn main() {
    let input: Vec<i32> = include_str!("../../input.txt")
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Vec<i32>) -> i32 {
    input.iter().sum()
}

fn part_2(input: &Vec<i32>) -> i32 {
    let mut frequencies = vec![0];
    let mut freq = 0;

    for change in input {
        freq += *change;
        if frequencies.contains(&freq) {
            return freq;
        }
        frequencies.push(freq);
    }

    let offset = freq;

    // Find the two frequencies visited with a difference of 474

    let mut iter = 0;

    loop {
        iter += 1;
        for i in 1..frequencies.len() {
            for j in i + 1..frequencies.len() {
                if frequencies[j] - frequencies[i] == offset * iter {
                    return frequencies[j];
                }
            }
        }
    }
}
