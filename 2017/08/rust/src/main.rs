use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../../input.txt")));
}

fn part_1(input: &str) -> i32 {
    let mut registers = HashMap::<String, i32>::new();

    for line in input.lines() {
        let mut tokens = line.split(" ");
        let reg = tokens.next().unwrap().to_string();
        let operation = tokens.next().unwrap();
        let value = tokens.next().unwrap().parse::<i32>().unwrap();
        let _ = tokens.next().unwrap();
        let cond_reg = tokens.next().unwrap().to_string();
        let cond = tokens.next().unwrap();
        let cond_val = tokens.next().unwrap().parse::<i32>().unwrap();

        // Check condition
        let cond_reg_val = registers.get(&cond_reg).unwrap_or(&0);
        let cond_result = match cond {
            "==" => cond_reg_val == &cond_val,
            ">" => cond_reg_val > &cond_val,
            "<" => cond_reg_val < &cond_val,
            ">=" => cond_reg_val >= &cond_val,
            "<=" => cond_reg_val <= &cond_val,
            "!=" => cond_reg_val != &cond_val,
            _ => panic!(),
        };

        if cond_result {
            let reg_val = registers.remove(&reg).unwrap_or(0);
            match operation {
                "dec" => registers.insert(reg, reg_val - value),
                "inc" => registers.insert(reg, reg_val + value),
                _ => panic!(),
            };
        }
    }

    registers.into_values().max().unwrap()
}

fn part_2(input: &str) -> i32 {
    let mut registers = HashMap::<String, i32>::new();
    let mut max = 0;

    for line in input.lines() {
        let mut tokens = line.split(" ");
        let reg = tokens.next().unwrap().to_string();
        let operation = tokens.next().unwrap();
        let value = tokens.next().unwrap().parse::<i32>().unwrap()
            * if operation == "inc" { 1 } else { -1 };
        let _ = tokens.next().unwrap();
        let cond_reg = tokens.next().unwrap().to_string();
        let cond = tokens.next().unwrap();
        let cond_val = tokens.next().unwrap().parse::<i32>().unwrap();

        // Check condition
        let cond_reg_val = registers.get(&cond_reg).unwrap_or(&0);
        let cond_result = match cond {
            "==" => cond_reg_val == &cond_val,
            ">" => cond_reg_val > &cond_val,
            "<" => cond_reg_val < &cond_val,
            ">=" => cond_reg_val >= &cond_val,
            "<=" => cond_reg_val <= &cond_val,
            "!=" => cond_reg_val != &cond_val,
            _ => panic!(),
        };

        if cond_result {
            let reg_val = registers.remove(&reg).unwrap_or(0) + value;
            if reg_val > max {
                max = reg_val;
            }
            registers.insert(reg, reg_val);
        }
    }

    max
}
