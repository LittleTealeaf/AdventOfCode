use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "sample.txt";

fn main() {
    let part_1_answer = part_1(INPUT);
    println!("Part 1: {} units tall", part_1_answer);

    //let part_2_answer = part_2(INPUT);
    //println!("Part 2: {} units tall", part_2_answer);
    //
}

fn part_1(filename: &str) -> i64 {
    let input = {
        let file = File::open(filename).unwrap();
        let mut lines = BufReader::new(file).lines();
        lines.next().unwrap().unwrap()
    };

    let instructions = parse_instructions(input);

    let height = run_simulation(instructions, 2022);

    height
}

fn part_2(filename: &str) -> i64 {
    let input = {
        let file = File::open(filename).unwrap();
        let mut lines = BufReader::new(file).lines();
        lines.next().unwrap().unwrap()
    };

    let instructions = parse_instructions(input);

    //let height = run_simulation(instructions, 1000000000000);
    let height = run_simulation(instructions, 1000000);

    height
}

#[repr(u8)]
enum Direction {
    Left,
    Right,
}

fn parse_instructions(input: String) -> Vec<Direction> {
    let mut instructions = Vec::new();

    let mut chars = input.chars();
    for c in chars {
        instructions.push(match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid Character Found"),
        })
    }

    instructions
}

struct Simulation {
    columns: [Vec<i64>; 7],
}

impl Simulation {
    fn new() -> Simulation {
        Simulation {
            columns: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        }
    }

    fn put_rock(&mut self, x: usize, y: i64) {
        //dbg!(&self.columns[x]);
        let len = self.columns[x].len();

        if len == 0 {
            self.columns[x].push(y);
        } else {
            for i in 0..len {
                if let Some(&v) = self.columns[x].get(i) {
                    if v == y {
                        return;
                    }
                    if v > y {
                        self.columns[x].insert(i, y);
                        return;
                    }
                }
            }
            self.columns[x].push(y);
        }
    }

    fn get(&self, x: usize, y: i64) -> bool {
        match self.columns[x].binary_search(&y) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn clean(&mut self, threshold: i64) {
        for x in 0..7 {
            for (i, val) in self.columns[x].iter().enumerate() {
                if val > &threshold {
                    self.columns[x].reverse();
                    self.columns[x].truncate(self.columns[x].len() - i);
                    self.columns[x].reverse();
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Rock {
    Minus,
    Plus,
    BackL,
    Vertical,
    Square,
}

impl Rock {
    fn height(&self) -> i64 {
        match self {
            Rock::Minus => 1,
            Rock::Plus | Rock::BackL => 3,
            Rock::Vertical => 4,
            Rock::Square => 2,
        }
    }

    fn width(&self) -> usize {
        match self {
            Rock::Minus => 4,
            Rock::Plus | Rock::BackL => 3,
            Rock::Vertical => 1,
            Rock::Square => 2,
        }
    }

    fn can_fall(&self, sim: &Simulation, x: usize, y: i64) -> bool {
        if y == 0 {
            return false;
        }
        match self {
            Rock::Minus => {
                !(sim.get(x, y - 1)
                    || sim.get(x + 1, y - 1)
                    || sim.get(x + 2, y - 1)
                    || sim.get(x + 3, y - 1))
            }
            Rock::Plus => !(sim.get(x, y) || sim.get(x + 2, y) || sim.get(x + 1, y - 1)),
            Rock::BackL => !(sim.get(x, y - 1) || sim.get(x + 1, y - 1) || sim.get(x + 2, y - 1)),
            Rock::Vertical => !sim.get(x, y - 1),
            Rock::Square => !(sim.get(x, y - 1) || sim.get(x + 1, y - 1)),
        }
    }

    fn can_move(&self, direction: &Direction, sim: &Simulation, x: usize, y: i64) -> bool {
        match direction {
            Direction::Left => {
                if x == 0 {
                    false
                } else {
                    match self {
                        Rock::Minus => !sim.get(x + 1, y),
                        Rock::Plus => {
                            !(sim.get(x, y) || sim.get(x, y + 2) || sim.get(x - 1, y + 1))
                        }
                        Rock::BackL => {
                            !(sim.get(x - 1, y) || sim.get(x + 1, y + 1) || sim.get(x + 1, y + 2))
                        }
                        Rock::Vertical => {
                            !(sim.get(x - 1, y)
                                || sim.get(x - 1, y + 1)
                                || sim.get(x - 1, y + 2)
                                || sim.get(x - 1, y + 3))
                        }
                        Rock::Square => !(sim.get(x - 1, y) || sim.get(x - 1, y + 1)),
                    }
                }
            }
            Direction::Right => {
                if x == 7 - self.width() {
                    false
                } else {
                    match self {
                        Rock::Minus => !sim.get(x + 4, y),
                        Rock::Plus => {
                            !(sim.get(x + 2, y) || sim.get(x + 3, y + 1) || sim.get(x + 2, y + 2))
                        }
                        Rock::BackL => {
                            !(sim.get(x + 3, y) || sim.get(x + 3, y + 1) || sim.get(x + 3, y + 2))
                        }
                        Rock::Vertical => {
                            !(sim.get(x + 1, y)
                                || sim.get(x + 1, y + 1)
                                || sim.get(x + 1, y + 2)
                                || sim.get(x + 1, y + 3))
                        }
                        Rock::Square => !(sim.get(x + 2, y) || sim.get(x + 2, y + 1)),
                    }
                }
            }
        }
    }

    fn place(&self, simulation: &mut Simulation, x: usize, y: i64) -> i64 {
        let points = match self {
            Rock::Minus => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Rock::Plus => vec![(x, y + 1), (x + 2, y + 1), (x + 1, y + 2)],
            Rock::BackL => vec![(x, y), (x + 1, y), (x + 2, y + 1), (x + 2, y + 2)],
            Rock::Vertical => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Rock::Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        };
        for (x, y) in points {
            simulation.put_rock(x, y);
        }

        y + self.height()
    }
}

fn run_simulation(instructions: Vec<Direction>, rock_count: u64) -> i64 {
    let mut sim = Simulation::new();
    let instr_len = instructions.len();

    let mut rock = Rock::Minus;
    let mut height = 0;
    let mut instr_index = 0;

    for i in 0..(rock_count) {
        let mut y = 3 + height;
        let mut x = 2;
        let width = rock.width();

        loop {
            let jet = instructions.get(instr_index).unwrap();
            instr_index = (instr_index + 1) % instr_len;

            if rock.can_move(jet, &sim, x, y) {
                match jet {
                    Direction::Left => {
                        x -= 1;
                    }
                    Direction::Right => {
                        x += 1;
                    }
                }
            }

            if !rock.can_fall(&sim, x, y) {
                break;
            }

            y -= 1;
        }

        let h = rock.place(&mut sim, x, y);
        if h > height {
            height = h;
        }

        rock = match rock {
            Rock::Minus => Rock::Plus,
            Rock::Plus => Rock::BackL,
            Rock::BackL => Rock::Vertical,
            Rock::Vertical => Rock::Square,
            Rock::Square => Rock::Minus,
        };
    }

    height
}
