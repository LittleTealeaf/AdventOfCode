fn main() {
    let input = include_str!("../../input.txt")
        .trim()
        .split("\t")
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input.clone()));
}

fn part_1(mut banks: Vec<u64>) -> usize {
    let mut visited = Vec::new();

    let mut count = 0;

    loop {
        if visited.contains(&banks) {
            break;
        }

        visited.push(banks.clone());

        count += 1;

        let (base_index, blocks) = banks
            .iter()
            .map(|i| *i)
            .enumerate()
            .reduce(|acc, e| if acc.1 >= e.1 { acc } else { e })
            .unwrap();

        banks[base_index] = 0;

        for i in 0..blocks {
            let index = (base_index + (i as usize) + 1) % banks.len();
            banks[index] += 1;
        }
    }

    count
}

fn part_2(mut banks: Vec<u64>) -> usize {
    let mut visited: Vec<Vec<u64>> = Vec::new();

    let mut count = 0;

    loop {
        if let Some((index, _)) = visited
            .iter()
            .enumerate()
            .filter(|(_, state)| banks.eq(*state))
            .next()
        {
            return count - index;
        }

        visited.push(banks.clone());

        count += 1;

        let (base_index, blocks) = banks
            .iter()
            .map(|i| *i)
            .enumerate()
            .reduce(|acc, e| if acc.1 >= e.1 { acc } else { e })
            .unwrap();

        banks[base_index] = 0;

        for i in 0..blocks {
            let index = (base_index + (i as usize) + 1) % banks.len();
            banks[index] += 1;
        }
    }
}
