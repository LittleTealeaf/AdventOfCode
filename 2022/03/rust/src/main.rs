fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let length = line.len();
            let first_half = line.get(0..length / 2).unwrap().chars();
            let second_half = line
                .get(length / 2..length)
                .unwrap()
                .chars()
                .collect::<Vec<_>>();

            for c in first_half {
                if second_half.contains(&c) {
                    let n = c as u32;
                    if n >= 'a' as u32 {
                        return Some(n - 'a' as u32 + 1);
                    } else {
                        return Some(n - 'A' as u32 + 27);
                    }
                }
            }
            None
        })
        .flatten()
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    let mut iter = input.lines();
    let mut sum = 0;

    while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
        let valid = b
            .chars()
            .filter(|ch| a.contains(&ch.to_string()))
            .collect::<Vec<_>>();
        let ch = c
            .chars()
            .find(|ch| valid.contains(ch))
            .expect("Could not find similar item");

        let n = ch as u32;
        if n >= 'a' as u32 {
            sum += n - 'a' as u32 + 1;
        } else {
            sum += n - 'A' as u32 + 27;
        }
    }

    sum
}

#[test]
fn test_part_1() {
    let input = include_str!("../../test.txt");
    assert_eq!(part_1(input), 157);
}
