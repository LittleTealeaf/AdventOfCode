use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    b_x: isize,
    b_y: isize,
}

impl Sensor {
    fn dist(&self) -> isize {
        (self.x - self.b_x).abs() + (self.y - self.b_y).abs()
    }

    fn contains(&self, x: isize, y: isize) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.dist()
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open(FILENAME).unwrap();
    let lines = BufReader::new(file).lines();

    let mut sensors: Vec<Sensor> = Vec::new();
    let mut x_min: isize = std::isize::MAX;
    let mut x_max: isize = std::isize::MIN;

    for line in lines {
        if let Ok(line) = line {
            let line = line.replace(":", "").replace(",", "");
            let words: Vec<&str> = line.split(" ").map(|c| -> &str { &c[2..] }).collect();
            let sensor = Sensor {
                x: words[2].parse().unwrap(),
                y: words[3].parse().unwrap(),
                b_x: words[8].parse().unwrap(),
                b_y: words[9].parse().unwrap(),
            };
            let dist = sensor.dist();
            let x_left = sensor.x - dist;
            let x_right = sensor.x + dist;

            if x_left < x_min {
                x_min = x_left;
            }
            if x_right > x_max {
                x_max = x_right;
            }

            sensors.push(sensor);
        }
    }

    let y_target: isize = 2000000;

    let mut row: Vec<bool> = Vec::new();
    for _ in 0..=(x_max - x_min) {
        row.push(false);
    }

    let mut count: isize = 0;

    for sensor in sensors {
        let dist = sensor.dist();
        let y_diff = sensor.y.abs_diff(y_target) as isize;
        if y_diff <= dist {
            for x in (sensor.x - (dist - y_diff))..(sensor.x + (dist - y_diff)) {
                let i = x - x_min;
                if !row[i as usize] {
                    count += 1;
                }
                row[i as usize] = true;
            }
        }
    }
    println!("Part 1: {} positions", count);
}

fn part_2() {
    let file = File::open(FILENAME).unwrap();
    let lines = BufReader::new(file).lines();

    let mut sensors: Vec<Sensor> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let line = line.replace(":", "").replace(",", "");
            let words: Vec<&str> = line.split(" ").map(|c| -> &str { &c[2..] }).collect();
            sensors.push(Sensor {
                x: words[2].parse().unwrap(),
                y: words[3].parse().unwrap(),
                b_x: words[8].parse().unwrap(),
                b_y: words[9].parse().unwrap(),
            })
        }
    }

    let max: isize = 4000000;
    let mut x: isize = 0;
    let mut y: isize = 0;

    'y: loop {
        if y > max {
            break 'y;
        }

        'x: loop {
            if x > max {
                x = 0;
                break 'x;
            }
            for sensor in &sensors {
                if sensor.contains(x, y) {
                    let y_diff = (y - sensor.y).abs();

                    x = sensor.x + (sensor.dist() - y_diff) + 1;

                    continue 'x;
                }
            }
            break 'y;
        }
        y += 1;
    }

    let freq = x * 4000000 + y;
    println!("Part 2: {}", freq);
}
