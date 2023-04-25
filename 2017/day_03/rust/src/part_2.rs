pub fn part_2(number: i64) -> i64 {
    let mut previous_layer = vec![1];
    let mut layer = 2;

    // loop {
    //     previous_layer.reverse();
    //     let mut layer_values: Vec<i64> = Vec::new();
    //
    //     let mut prev_layer_buffer: Vec<i64> = Vec::new();
    //     prev_layer_buffer.push(*previous_layer.last().unwrap());
    //
    //     let layer_size = calculate_layer_size(layer);
    //     let mut dist = layer - 2;
    //     let mut change = -1;
    //
    //     for i in 0..layer_size {
    //         let mut value = layer_values.last().unwrap_or(&0).clone();
    //
    //         if dist == 0 {
    //             change = 1;
    //         } else if dist == layer - 1 {
    //             change = -1;
    //         }
    //         dist += change;
    //     }
    // }

    0
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
