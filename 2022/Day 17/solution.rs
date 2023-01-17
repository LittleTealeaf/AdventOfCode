use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";

fn main() {
    let part_1_answer = part_1(INPUT);
    let part_2_answer = part_2(INPUT);

    println!("Part 1: {} units tall", part_1_answer);
    println!("Part 2: {} units tall", part_2_answer);
}

fn part_1(filename: &str) -> usize {
    let input = {
        let file = File::open(filename).unwrap();
        let mut lines = BufReader::new(file).lines();
        lines.next().unwrap().unwrap()
    };

    let instructions = parse_instructions(input);

    let height = run_simulation(instructions, 2022);

    height
}

fn part_2(filename: &str) -> usize {
    let input = {
        let file = File::open(filename).unwrap();
        let mut lines = BufReader::new(file).lines();
        lines.next().unwrap().unwrap()
    };

    let instructions = parse_instructions(input);

    let height = run_simulation(instructions, 1000000000000);

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

#[repr(u8)]
#[derive(Debug)]
enum Rock {
    Minus,
    Plus,
    BackL,
    Vertical,
    Square,
}

impl Rock {
    fn height(&self) -> usize {
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

    fn can_fall(&self, sim: &Vec<[bool; 7]>, x: usize, y: usize) -> bool {
        if y == 0 {
            return false;
        }
        match self {
            Rock::Minus => {
                let row = sim.get(y - 1).unwrap();

                if row[x] || row[x + 1] || row[x + 2] || row[x + 3] {
                    return false;
                }
                return true;
            }
            Rock::Plus => {
                let row = sim.get(y - 1).unwrap();
                if row[x + 1] {
                    return false;
                }

                let row = sim.get(y).unwrap();
                if row[x] || row[x + 2] {
                    return false;
                }

                true
            }
            Rock::BackL => {
                let row = sim.get(y - 1).unwrap();
                if row[x] || row[x + 1] || row[x + 2] {
                    return false;
                }

                true
            }
            Rock::Vertical => {
                let row = sim.get(y - 1).unwrap();
                if row[x] {
                    return false;
                }
                true
            }
            Rock::Square => {
                let row = sim.get(y - 1).unwrap();
                if row[x] || row[x + 1] {
                    return false;
                }
                true
            }
        }
    }

    fn can_move(&self, direction: &Direction, sim: &Vec<[bool; 7]>, x: usize, y: usize) -> bool {
        match direction {
            Direction::Left => {
                if x == 0 {
                    false
                } else {
                    match self {
                        Rock::Minus => !sim.get(y).unwrap()[x - 1],
                        Rock::Plus => {
                            !(sim.get(y).unwrap()[x]
                                || sim.get(y + 1).unwrap()[x - 1]
                                || sim.get(y + 2).unwrap()[x])
                        }
                        Rock::BackL => {
                            !(sim.get(y).unwrap()[x - 1]
                                || sim.get(y + 1).unwrap()[x + 1]
                                || sim.get(y + 2).unwrap()[x + 1])
                        }
                        Rock::Vertical => {
                            !(sim.get(y).unwrap()[x - 1]
                                || sim.get(y + 1).unwrap()[x - 1]
                                || sim.get(y + 2).unwrap()[x - 1]
                                || sim.get(y + 3).unwrap()[x - 1])
                        }
                        Rock::Square => {
                            !(sim.get(y).unwrap()[x - 1] || sim.get(y + 1).unwrap()[x - 1])
                        }
                    }
                }
            }
            Direction::Right => {
                if x == 7 - self.width() {
                    false
                } else {
                    match self {
                        Rock::Minus => !sim.get(y).unwrap()[x + 4],
                        Rock::Plus => {
                            !(sim.get(y).unwrap()[x + 2]
                                || sim.get(y + 1).unwrap()[x + 3]
                                || sim.get(y + 2).unwrap()[x + 2])
                        }
                        Rock::BackL => {
                            !(sim.get(y).unwrap()[x + 3]
                                || sim.get(y + 1).unwrap()[x + 3]
                                || sim.get(y + 2).unwrap()[x + 3])
                        }
                        Rock::Vertical => {
                            !(sim.get(y).unwrap()[x + 1]
                                || sim.get(y + 1).unwrap()[x + 1]
                                || sim.get(y + 2).unwrap()[x + 1]
                                || sim.get(y + 3).unwrap()[x + 1])
                        }
                        Rock::Square => {
                            !(sim.get(y).unwrap()[x + 2] || sim.get(y + 1).unwrap()[x + 2])
                        }
                    }
                }
            }
        }
    }

    fn place(&self, sim: &mut Vec<[bool; 7]>, x: usize, y: usize) -> usize {
        match self {
            Rock::Minus => {
                let row = sim.get_mut(y).unwrap();
                row[x] = true;
                row[x + 1] = true;
                row[x + 2] = true;
                row[x + 3] = true;
            }
            Rock::Plus => {
                let row = sim.get_mut(y).unwrap();
                row[x + 1] = true;
                let row = sim.get_mut(y + 1).unwrap();
                row[x] = true;
                row[x + 1] = true;
                row[x + 2] = true;
                let row = sim.get_mut(y + 2).unwrap();
                row[x + 1] = true;
            }
            Rock::BackL => {
                let row = sim.get_mut(y).unwrap();
                row[x] = true;
                row[x + 1] = true;
                row[x + 2] = true;
                let row = sim.get_mut(y + 1).unwrap();
                row[x + 2] = true;
                let row = sim.get_mut(y + 2).unwrap();
                row[x + 2] = true;
            }
            Rock::Vertical => {
                for i in 0..4 {
                    let row = sim.get_mut(y + i).unwrap();
                    row[x] = true;
                }
            }
            Rock::Square => {
                for i in 0..2 {
                    let row = sim.get_mut(y + i).unwrap();
                    row[x] = true;
                    row[x + 1] = true;
                }
            }
        }

        y + self.height()
    }
}

fn run_simulation(instructions: Vec<Direction>, rock_count: u64) -> usize {
    let mut sim = Vec::new();
    let instr_len = instructions.len();

    let mut rock = Rock::Minus;
    let mut height = 0;
    let mut instr_index = 0;

    for _ in 0..(rock_count) {
        while sim.len() < 3 + height + rock.height() {
            sim.push([false; 7]);
        }

        let mut y = 3 + height;
        let mut x = 2;

        let width = rock.width();

        let mut move_down = false;

        while rock.can_fall(&sim, x, y) {
            if move_down {
                y -= 1;
            } else {
                move_down = true;
            }

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
