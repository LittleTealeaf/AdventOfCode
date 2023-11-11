fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &Vec<i32>) -> i32 {
    fn recursive(input: &Vec<i32>, capacity: i32, index: usize) -> i32 {
        if capacity == 150 {
            1
        } else if capacity > 150 || index >= input.len() {
            0
        } else {
            recursive(input, capacity + input[index], index + 1)
                + recursive(input, capacity, index + 1)
        }
    }

    recursive(input, 0, 0)
}

fn part_2(input: &Vec<i32>) -> usize {
    fn recursive(
        input: &Vec<i32>,
        capacity: i32,
        index: usize,
        containers: usize,
    ) -> Option<(usize, usize)> {
        if capacity == 150 {
            Some((containers, 1))
        } else if capacity > 150 || index >= input.len() {
            None
        } else {
            let values = [
                recursive(input, capacity + input[index], index + 1, containers + 1),
                recursive(input, capacity, index + 1, containers),
            ]
            .into_iter()
            .flatten();

            let mut min_containers = usize::MAX;
            let mut total_count = 0;

            for (count, instances) in values {
                if count < min_containers {
                    min_containers = count;
                    total_count = 0;
                }
                if count == min_containers {
                    total_count += instances;
                }
            }
            Some((min_containers, total_count))
        }
    }

    let (_, size) = recursive(input, 0, 0, 0).unwrap();
    return size;
}
