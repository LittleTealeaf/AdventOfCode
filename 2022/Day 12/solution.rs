use std::fs::File;
use std::io::{BufRead, BufReader};

struct Tile {
    height: u32,
    moves: Option<u64>,
}

enum Point {
    Start(Tile),
    Path(Tile),
}

struct Coords {
    x: usize,
    y: usize,
}

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("sample.txt").expect("");
    let lines = BufReader::new(file).lines();

    let mut map: Vec<Vec<Point>> = Vec::new();

    let mut buffer: Vec<Coords> = Vec::new();

    {
        let mut y = 0;
        for line in lines {
            if let Ok(line) = line {
                let mut row: Vec<Point> = Vec::new();
                let mut x = 0;
                for c in line.chars() {
                    row.push(match c {
                        'E' => {
                            buffer.push(Coords { x, y });
                            Point::Path(Tile {
                                height: 'z' as u32,
                                moves: Some(0),
                            })
                        }
                        'S' => Point::Start(Tile {
                            height: 'a' as u32,
                            moves: None,
                        }),
                        _ => Point::Path(Tile {
                            height: c as u32,
                            moves: None,
                        }),
                    });
                    x += 1;
                }
                (&mut map).push(row);
            }

            y += 1;
        }
    }

    {
        while buffer.len() > 0 {
            if let Some(pos) = buffer.pop() {
                for i in [-1,1] {
                    for t in [true,false] {
                        let (x,y) = if t {
                            (pos.x + i, pos.y)
                        } else {
                            (pos.x, pos.y + i)
                        };

                        if let Some(row) = (&mut map).get(y) {
                            if let Some(&mut point) = row.get(x) {

                            }
                        }
                    }
                }
            }
        }
    }
}
