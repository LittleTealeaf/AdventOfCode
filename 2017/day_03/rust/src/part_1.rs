pub fn part_1(number: i64) -> i64 {
    let layer = {
        let sqrt = next_perfect_square_root(number) + 1;
        if sqrt % 2 == 0 {
            sqrt / 2
        } else {
            sqrt / 2 + 1
        }
    };

    if layer == 1 {
        return 0;
    }

    let mut position = get_layer_start(layer);
    let mut distance = 2 * layer - 3;
    let mut change = -1;

    while position < number {
        position += 1;
        distance += change;
        if distance == layer - 1 {
            change = 1;
        }
        if distance == 2 * layer - 2 {
            change = -1;
        }
    }

    distance
}

fn next_perfect_square_root(number: i64) -> i64 {
    let mut left = 1;
    let mut right = number - 1;

    while left < right {
        let center = (right - left) / 2 + left;

        if let Some(square) = center.checked_mul(center) {
            if square < number {
                left = center + 1;
            } else if square > number {
                right = center;
            } else {
                return center;
            }
        } else {
            right = center;
        }

    }

    left
}

fn get_layer_start(layer: i64) -> i64 {
    if layer == 1 {
        return 1;
    }
    let value = layer - 1;
    let value = value * 2 - 1;
    value * value + 1
}

#[test]
fn test_get_layer_start() {
    assert_eq!(get_layer_start(1), 1);
    assert_eq!(get_layer_start(2), 2);
    assert_eq!(get_layer_start(3), 10);
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(1), 0);
    assert_eq!(part_1(12), 3);
    assert_eq!(part_1(23), 2);
    assert_eq!(part_1(1024), 31);
}
