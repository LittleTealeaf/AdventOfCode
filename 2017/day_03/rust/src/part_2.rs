struct Layer {
    sides: [Vec<i64>; 4],
    corners: [i64; 4],
}

impl Layer {
    fn second_layer() -> Self {
        Self {
            sides: [vec![1], vec![4], vec![10], vec![23]],
            corners: [2, 5, 11, 25],
        }
    }

    fn new(layer: usize) -> Self {
        Self {
            sides: [
                vec![0; layer],
                vec![0; layer],
                vec![0; layer],
                vec![0; layer],
            ],
            corners: [0, 0, 0, 0],
        }
    }
}

const FIRST_NINE_VALUES: [i64; 9] = [1, 1, 2, 4, 5, 10, 11, 23, 25];

pub fn part_2(number: i64) -> i64 {
    for value in FIRST_NINE_VALUES {
        if value > number {
            return value;
        }
    }

    let mut layer_prev = Layer::second_layer();
    let mut index = 3;

    loop {
        // -3, -1, 1, 3, 5, 7,
        let side_length = index * 2 - 3;
        let mut layer = Layer::new(side_length);

        for side in 0..4 {
            let mut value =
                *layer.sides[(side + 3) % 4].last().unwrap() + layer.corners[(side + 3) % 4];
            for i in 0..side_length {
                if i < 2 {
                    value += layer_prev.corners[(side + 3) % 4];
                }

                if i < side_length - 2 {
                    value += layer_prev.sides[side][i];
                }

                if i < side_length - 1 && i > 0 {
                    value += layer_prev.sides[side][i - 1];
                }

                if i < side_length && i > 1 {
                    value += layer_prev.sides[side][i - 2];
                }

                if i > side_length - 3 {
                    value += layer_prev.corners[side];
                }

                if i > side_length - 2 {
                    value += layer.sides[(side + 1) % 4][0];
                }

                if value > number {
                    return value;
                }

                layer.sides[side][i] = value;
            }

            value += layer_prev.corners[side];
            value += layer.sides[(side + 1) % 4][0];

            if value > number {
                return value;
            }

            layer.corners[side] = value;
            
            
        }

        index += 1;
        layer_prev = layer;
    }
}

fn calculate_layer_size(layer: i64) -> i64 {
    let prev_length = (layer - 1) * 2 - 1;
    let prev_area = prev_length * prev_length;
    let length = layer * 2 - 1;
    let area = length * length;
    area - prev_area
}

#[test]
fn test_part_2() {
    let actual_values = [
        (1, 2),
        (2, 4),
        (3, 4),
        (4, 5),
        (5, 10),
        (6, 10),
        (7, 10),
        (8, 10),
        (25, 26),
        (59, 122),
        (100, 122),
        (200, 304),
        (300, 304),
        (400, 747),
        (800, 806),
    ];

    for (input, output) in actual_values {
        assert_eq!(part_2(input), output);
    }
}
