fn main() {
    let input = parse_input(include_str!("../../input.txt"));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

struct Reading {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

impl Reading {
    fn distance(&self) -> u64 {
        self.sx.abs_diff(self.bx) + self.sy.abs_diff(self.by)
    }
}

fn parse_input(input: &str) -> Vec<Reading> {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once("=").unwrap();
            let (str_sensor_x, line) = line.split_once(",").unwrap();
            let (_, line) = line.split_once("=").unwrap();
            let (str_sensor_y, line) = line.split_once(":").unwrap();

            let (_, line) = line.split_once("=").unwrap();
            let (str_beacon_x, line) = line.split_once(",").unwrap();
            let (_, str_beacon_y) = line.split_once("=").unwrap();

            Reading {
                sx: str_sensor_x.parse().unwrap(),
                sy: str_sensor_y.parse().unwrap(),
                bx: str_beacon_x.parse().unwrap(),
                by: str_beacon_y.parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: &Vec<Reading>) -> usize {
    const Y: i64 = 2000000;

    // Find the min and max
    let min = input
        .iter()
        .map(|input| (input.sx - input.distance() as i64))
        .min()
        .unwrap();

    let max = input
        .iter()
        .map(|input| (input.sx + input.distance() as i64))
        .max()
        .unwrap();

    let mut row = vec![false; (max - min) as usize];

    for Reading { sy, sx, by, bx } in input {
        let dist = sy.abs_diff(*by) + sx.abs_diff(*bx);
        let dist_to_y = sy.abs_diff(Y);

        if dist_to_y <= dist {
            let diff = dist - dist_to_y;
            let left = sx - diff as i64;
            let right = sx + diff as i64;
            for i in left..right {
                row[(i - min) as usize] = true;
            }
        }
    }

    for Reading {
        by,
        bx,
        sx: _,
        sy: _,
    } in input
    {
        if by == &Y {
            row[(*bx) as usize] = false;
        }
    }

    row.into_iter().filter(|i| *i).count()
}

fn part_2(input: &Vec<Reading>) -> i64 {
    for y in 0..=4000000 {
        let mut x = 0;
        'a: while x < 4000000 {
            for sensor in input {
                let dist = sensor.distance();
                if sensor.sx.abs_diff(x) + sensor.sy.abs_diff(y) <= dist {
                    let y_dist = sensor.sy.abs_diff(y);
                    let x_dist = dist - y_dist;

                    x = sensor.sx + x_dist as i64 + 1;

                    continue 'a;
                }
            }
            return x * 4000000 + y;
        }
    }
    0
}
