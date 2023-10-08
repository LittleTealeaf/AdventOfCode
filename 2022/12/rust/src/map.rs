pub struct Map {
    pub board: Vec<Vec<usize>>,
    pub start: Point,
    pub end: Point,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };
        let board = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        'S' => {
                            start.x = x;
                            start.y = y;
                            0
                        }
                        'E' => {
                            end.y = y;
                            end.x = x;
                            'z' as usize - 'a' as usize
                        }
                        ch => ch as usize - 'a' as usize,
                    })
                    .collect()
            })
            .collect();

        Self { board, start, end }
    }

    pub fn load_input() -> Self {
        Self::parse_input(include_str!("../../input.txt"))
    }

    #[cfg(test)]
    pub fn load_test() -> Self {
        Self::parse_input(include_str!("../../test.txt"))
    }

    pub fn dist_to_end(&self, point: Point) -> usize {
        self.end.l1_dist(&point)
    }

    pub fn get(&self, point: Point) -> Option<usize> {
        self.board.get(point.y)?.get(point.x).copied()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn l1_dist(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[test]
fn test_map() {
    assert_eq!(Map::load_test().get(Point {x: 2, y: 0}), Some(1));
}
