fn main() {
    {
        let input = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];
        let value = part_1(input, 40);
        println!("Part 1: {}", value);
    }
}

fn number_to_array(value: usize) -> Vec<u8> {
    let mut array = Vec::new();
    let mut value = value;

    while value > 0 {
        array.push((value % 10) as u8);
        value /= 10;
    }

    array.reverse();

    array
}

fn part_1(state: Vec<u8>, steps: usize) -> usize {
    let mut state = state;

    for _ in 0..steps {
        let mut buffer = Vec::new();

        let mut value = 0;
        let mut count = 1;
        let mut add_to_buffer = false;

        for item in state {
            if item == value {
                count += 1;
            } else {
                if add_to_buffer {
                    buffer.append(&mut number_to_array(count));
                    buffer.push(value);
                } else {
                    add_to_buffer = true;
                }

                value = item;
                count = 1;
            }
        }
        buffer.append(&mut number_to_array(count));
        buffer.push(value);

        state = buffer;
    }

    state.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1];
        let value = part_1(input, 5);
        assert_eq!(value, 6);
    }
}
