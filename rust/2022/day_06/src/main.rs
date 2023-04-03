fn get_input() -> String {
    include_str!("../input.txt").to_string()
}

fn main() {
    {
        let input = get_input();
        let result = part_1(input);
        println!("Part 1: {}", result);
    }

    {
        let input = get_input();
        let result = part_2(input);
        println!("Part 2: {}", result);
    }
}

fn part_1(string: String) -> usize {
    let chars = string.chars().collect::<Vec<_>>();

    'iter: for i in 0..(chars.len() - 4) {
        let buffer = &chars[i..(i + 4)];
        for i in 0..4 {
            for j in 1..(4 - i) {
                if buffer[i] == buffer[i + j] {
                    continue 'iter;
                }
            }
        }
        return i + 4;
    }

    0
}

fn part_2(string: String) -> usize {
    let chars = string.chars().collect::<Vec<_>>();

    'iter: for i in 0..(chars.len() - 14) {
        let buffer = &chars[i..(i + 14)];
        for i in 0..14 {
            for j in 1..(14 - i) {
                if buffer[i] == buffer[i + j] {
                    continue 'iter;
                }
            }
        }
        return i + 14;
    }

    0
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
    assert_eq!(part_1(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 6);
    assert_eq!(
        part_1(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
        10
    );
    assert_eq!(part_1(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
    assert_eq!(part_2(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
    assert_eq!(part_2(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 23);
    assert_eq!(part_2(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
    assert_eq!(part_2(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
}
