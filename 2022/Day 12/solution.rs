use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Tile {
    start: bool,
    height: usize,
    moves: usize,
}

#[derive(Debug)]
struct Coords {
    x: isize,
    y: isize,
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}

fn part_1(filename: &str) {
    let file = File::open(filename).expect("");
    let lines = BufReader::new(file).lines();

    let mut map: Vec<Vec<Tile>> = Vec::new();

    let mut buffer: Vec<Coords> = Vec::new();
    let mut start: Option<Coords> = None;

    {
        let mut y = 0;
        for line in lines {
            if let Ok(line) = line {
                let mut row: Vec<Tile> = Vec::new();
                let mut x = 0;
                for c in line.chars() {
                    row.push(match c {
                        'E' => {
                            buffer.push(Coords { x, y });
                            Tile {
                                height: 'z' as usize,
                                moves: 0,
                                start: false,
                            }
                        }
                        'S' => {
                            start = Some(Coords { x, y });
                            Tile {
                                height: 'a' as usize,
                                moves: std::usize::MAX,
                                start: true,
                            }
                        }
                        _ => Tile {
                            height: c as usize,
                            moves: std::usize::MAX,
                            start: false,
                        },
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
            if let Some(center) = buffer.pop() {
                let ref_center = (&map)
                    .get(center.y as usize)
                    .unwrap()
                    .get(center.x as usize)
                    .unwrap();
                let height = ref_center.height;
                let moves = ref_center.moves;

                for (x, y) in [
                    (center.x + 1, center.y),
                    (center.x - 1, center.y),
                    (center.x, center.y + 1),
                    (center.x, center.y - 1),
                ] {
                    if x < 0 || y < 0 {
                        continue;
                    }
                    if let Some(ref_row) = (&mut map).get_mut(y as usize) {
                        if let Some(ref_pt) = ref_row.get_mut(x as usize) {
                            if ref_pt.height >= height - 1 {
                                if ref_pt.moves > moves + 1 {
                                    ref_pt.moves = moves + 1;
                                    buffer.push(Coords { x, y });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    {
        if let Some(st) = start {
            let start_tile = (&map)
                .get(st.y as usize)
                .unwrap()
                .get(st.x as usize)
                .unwrap();
            println!("Part 1: {} moves", start_tile.moves);
        }
    }
}

fn part_2(filename: &str) {
    let file = File::open(filename).expect("");
    let lines = BufReader::new(file).lines();

    let mut map: Vec<Vec<Tile>> = Vec::new();

    let mut buffer: Vec<Coords> = Vec::new();
    let mut starts: Vec<Coords> = Vec::new();

    {
        let mut y = 0;
        for line in lines {
            if let Ok(line) = line {
                let mut row: Vec<Tile> = Vec::new();
                let mut x = 0;
                for c in line.chars() {
                    row.push(match c {
                        'E' => {
                            buffer.push(Coords { x, y });
                            Tile {
                                height: 'z' as usize,
                                moves: 0,
                                start: false,
                            }
                        }
                        'S' | 'a' => {
                            starts.push(Coords { x, y });
                            Tile {
                                height: 'a' as usize,
                                moves: std::usize::MAX,
                                start: true,
                            }
                        }
                        _ => Tile {
                            height: c as usize,
                            moves: std::usize::MAX,
                            start: false,
                        },
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
            if let Some(center) = buffer.pop() {
                let ref_center = (&map)
                    .get(center.y as usize)
                    .unwrap()
                    .get(center.x as usize)
                    .unwrap();
                let height = ref_center.height;
                let moves = ref_center.moves;

                for (x, y) in [
                    (center.x + 1, center.y),
                    (center.x - 1, center.y),
                    (center.x, center.y + 1),
                    (center.x, center.y - 1),
                ] {
                    if x < 0 || y < 0 {
                        continue;
                    }
                    if let Some(ref_row) = (&mut map).get_mut(y as usize) {
                        if let Some(ref_pt) = ref_row.get_mut(x as usize) {
                            if ref_pt.height >= height - 1 {
                                if ref_pt.moves > moves + 1 {
                                    ref_pt.moves = moves + 1;
                                    buffer.push(Coords { x, y });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut shortest_dist = std::usize::MAX;

    for start in starts {
        let tile = (&map).get(start.y as usize).unwrap().get(start.x as usize).unwrap();
        if tile.moves < shortest_dist {
            shortest_dist = tile.moves;
        }
    }

    println!("Part 2: {} moves", shortest_dist);
}
