use std::collections::HashMap;

fn part_1(input: i32) -> i32 {
    let mut ring = 1;
    let mut i = 1;

    if input == 1 {
        return 0;
    }

    while i < input {
        ring += 1;
        let side = ring * 2 - 1;
        let side_prev = side - 2;
        let ring_size = side * side - side_prev * side_prev;
        i += ring_size;
    }

    {
        let side = ring * 2 - 1;
        let side_prev = side - 2;
        i -= side * side - side_prev * side_prev;
        ring -= 1;
    };

    i += 1;

    let min = ring;
    let max = ring * 2;
    let mut dist = ring + (ring - 1);
    let mut scale = -1;

    for _ in i..input {
        if dist == min || dist == max {
            scale *= -1;
        }
        dist += scale;
    }

    dist
}

fn part_2(input: i32) -> i32 {
    let mut cache = HashMap::<(i32, i32), i32>::new();

    cache.insert((0, 0), 1);

    let mut ring = 1;
    let mut x = 1;
    let mut y = 0;

    loop {
        let offsets = [
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y + 1),
            (x + 1, y),
            (x + 1, y - 1),
        ];

        let value = offsets
            .into_iter()
            .filter_map(|val| cache.get(&val))
            .sum::<i32>();

        if value > input {
            return value;
        }

        cache.insert((x, y), value);

        if x == ring && y == -1 * ring {
            x += 1;
            ring += 1;
        } else if y == -1 * ring {
            x += 1;
        } else if x == -1 * ring {
            y -= 1;
        } else if y == ring {
            x -= 1;
        } else if x == ring {
            y += 1;
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt")
        .trim()
        .parse::<i32>()
        .unwrap();

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}
