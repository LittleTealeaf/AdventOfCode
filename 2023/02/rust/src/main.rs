fn main() {
    let input = include_str!("../../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;

    input
        .lines()
        .filter_map(|line| {
            let line = line.replace(":", "").replace(",", "").replace(";", "");
            let mut tokens = line.split(" ");
            let _ = tokens.next().unwrap();
            let number = tokens.next().unwrap().parse::<i32>().unwrap();

            while let (Some(count), Some(color)) = (tokens.next(), tokens.next()) {
                let count = count.parse::<i32>().unwrap();

                let valid = match color {
                    "red" => count <= RED,
                    "blue" => count <= BLUE,
                    "green" => count <= GREEN,
                    _ => panic!(),
                };

                if !valid {
                    return None;
                }
            }

            Some(number)
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let line = line.replace(":", "").replace(",", "").replace(";", "");
            let mut tokens = line.split(" ");
            let _ = tokens.next();
            let _ = tokens.next();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            while let (Some(count), Some(color)) = (tokens.next(), tokens.next()) {
                let count = count.parse::<i32>().unwrap();

                match color {
                    "red" => red = red.max(count),
                    "blue" => blue = blue.max(count),
                    "green" => green = green.max(count),
                    _ => panic!(),
                };
            }

            red * blue * green
        })
        .sum()
}

#[test]
fn tests() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part_1(input), 8);
    assert_eq!(part_2(input), 2286);
}
