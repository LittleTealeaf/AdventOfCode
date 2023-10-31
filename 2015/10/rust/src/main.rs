fn main() {
    let input = include_str!("../../input.txt")
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    println!("Part 1: {}", calculate(&input, 40));
    println!("Part 2: {}", calculate(&input, 50));
}

fn calculate(input: &Vec<u32>, iterations: usize) -> usize {
    let mut value = input.clone();
    for _ in 0..iterations {
        let mut new = Vec::new();

        let mut iter = value.into_iter();

        let mut current = iter.next().unwrap();
        let mut count = 1;

        for digit in iter {
            if current == digit {
                count += 1;
            } else {
                new.push(count);
                new.push(current);
                current = digit;
                count = 1;
            }
        }
        new.push(count);
        new.push(current);

        value = new;
    }
    value.len()
}
