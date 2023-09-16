fn read_input() -> Vec<usize> {
    include_str!("../../../../inputs/2019/02/input.txt")
        .split(",")
        .filter_map(|c| c.parse().ok())
        .collect()
}

fn main() {
    println!("Part 1: {}", part_1(read_input()));
    println!("Part 2: {}", part_2(read_input()));
}

fn execute_program(mut input: Vec<usize>) -> Vec<usize> {
    let mut pointer = 0;
    loop {
        match input[pointer] {
            1 => {
                let a_ptr = input[pointer + 1];
                let b_ptr = input[pointer + 2];
                let out_ptr = input[pointer + 3];
                input[out_ptr] = input[a_ptr] + input[b_ptr];
            }
            2 => {
                let a_ptr = input[pointer + 1];
                let b_ptr = input[pointer + 2];
                let out_ptr = input[pointer + 3];
                input[out_ptr] = input[a_ptr] * input[b_ptr];
            }
            99 => break,
            _ => panic!("Invalid Op Code"),
        }
        pointer += 4;
    }

    input
}

fn part_1(mut input: Vec<usize>) -> usize {
    input[1] = 12;
    input[2] = 2;

    input = execute_program(input);

    input[0]
}

fn part_2(mut input: Vec<usize>) -> usize {
    0
}
