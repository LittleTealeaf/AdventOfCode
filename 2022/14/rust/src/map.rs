use std::{cmp::Ordering, collections::HashMap};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tile {
    Empty,
    Sand,
    Rock,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

pub struct Map {
    map: HashMap<(i32, i32), Tile>,
    height: i32,
}

impl Map {
    pub fn from_input(input: &str) -> Self {
        let mut map = Map {
            map: HashMap::new(),
            height: 0,
        };

        for line in input.lines() {
            let mut coords = line.split(" -> ").map(|coord| {
                let (x, y) = coord.split_once(",").unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            });

            let (mut x, mut y) = coords.next().unwrap();
            map.set(x, y, Tile::Rock);
            map.height = map.height.max(y);

            while let Some((gx, gy)) = coords.next() {
                map.height = map.height.max(gy);
                while x != gx || y != gy {
                    match (x.cmp(&gx), y.cmp(&gy)) {
                        (Ordering::Greater, _) => x -= 1,
                        (Ordering::Less, _) => x += 1,
                        (_, Ordering::Greater) => y -= 1,
                        (_, Ordering::Less) => y += 1,
                        (_, _) => {}
                    }
                    map.set(x, y, Tile::Rock);
                }
            }
        }

        map
    }

    pub fn get(&self, x: i32, y: i32) -> Tile {
        self.map.get(&(x, y)).cloned().unwrap_or_default()
    }

    pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
        self.map.insert((x, y), tile);
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}
