use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_wire_id(s: &str) -> u32 {
    let mut id = 0;

    for c in s.chars() {
        let v = c as usize - 'a' as usize;

        id += v as u32;
        id = id << 5;
    }

    id
}

type Wire = u32;

#[derive(Clone, Copy)]
enum Input {
    Wire(Wire),
    Value(u16),
}

impl Input {
    fn get_wire(&self) -> Option<Wire> {
        match self {
            Self::Wire(wire) => Some(*wire),
            Self::Value(_) => None,
        }
    }
}

impl Input {
    fn to_value(&self, map: &HashMap<Wire, u16>) -> u16 {
        match self {
            Self::Wire(wire) => *map.get(wire).unwrap(),
            Self::Value(value) => *value,
        }
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let parse_result = value.parse();
        match parse_result {
            Ok(x) => Self::Value(x),
            Err(_) => Self::Wire(get_wire_id(value)),
        }
    }
}

#[derive(Clone, Copy)]
enum Action {
    And(Input, Input),
    Or(Input, Input),
    LeftShift(Input, u16),
    RightShift(Input, u16),
    Not(Input),
    Set(Input),
}

impl Action {
    fn get_wire_inputs(&self) -> Vec<Wire> {
        let inputs = match self {
            Self::And(a, b) | Self::Or(a, b) => vec![a, b],
            Self::LeftShift(i, _) | Self::RightShift(i, _) | Self::Not(i) | Self::Set(i) => vec![i],
        };

        inputs
            .iter()
            .filter_map(|item| -> Option<Wire> { item.get_wire() })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    action: Action,
    output: Wire,
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let tokens: Vec<&str> = value.split(" ").collect();

        let action = match tokens.len() {
            3 => Action::Set(Input::from(tokens[0])),
            4 => Action::Not(Input::from(tokens[1])),
            5 => match tokens[1] {
                "AND" => Action::And(Input::from(tokens[0]), Input::from(tokens[2])),
                "OR" => Action::Or(Input::from(tokens[0]), Input::from(tokens[2])),
                "LSHIFT" => Action::LeftShift(Input::from(tokens[0]), tokens[2].parse().unwrap()),
                "RSHIFT" => Action::RightShift(Input::from(tokens[0]), tokens[2].parse().unwrap()),
                _ => panic!(),
            },
            _ => panic!(),
        };

        let output = get_wire_id(tokens[tokens.len() - 1]);

        Self { action, output }
    }
}

fn run_instructions(instructions: &Vec<Instruction>) -> HashMap<Wire, u16> {
    let mut state = HashMap::new();

    let mut wait_list = HashMap::new();

    let mut stack = Vec::new();

    for instruction in instructions {
        stack.push(instruction);
    }

    while stack.len() > 0 {
        let instruction = stack.pop().unwrap();

        let mut runnable = true;
        for input in instruction.action.get_wire_inputs() {
            if !state.contains_key(&input) {
                if !wait_list.contains_key(&input) {
                    wait_list.insert(input.clone(), Vec::<&Instruction>::new());
                }

                wait_list.get_mut(&input).unwrap().push(&instruction);
                runnable = false;
            }
        }

        if runnable {
            let value = match instruction.action {
                Action::Set(input) => input.to_value(&state),
                Action::And(a, b) => a.to_value(&state) & b.to_value(&state),
                Action::Or(a, b) => a.to_value(&state) | b.to_value(&state),
                Action::Not(a) => !a.to_value(&state),
                Action::LeftShift(input, steps) => input.to_value(&state) << steps,
                Action::RightShift(input, steps) => input.to_value(&state) >> steps,
            };

            state.insert(instruction.output, value);

            // check wait list
            if let Some(instructions) = wait_list.remove(&instruction.output) {
                for instruction in instructions {
                    stack.push(&instruction);
                }
            }
        }
    }

    state
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let instructions: Vec<Instruction> = lines
        .map(|line| -> Instruction { Instruction::from(line.unwrap()) })
        .collect();

    let map = run_instructions(&instructions);

    println!("{}", get_wire_id("a"));

    println!("{:?}", map);
}
