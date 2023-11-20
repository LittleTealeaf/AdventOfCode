fn main() {
    let tree = parse_input(include_str!("../../input.txt"));

    println!("Part 1: {}", tree.name);
    println!("Part 2: {}", tree.get_correct_weight().unwrap());
}

#[derive(Debug)]
struct Program {
    name: String,
    children: Vec<Program>,
    weight: i32,
}

impl Program {
    fn find_child_mut(&mut self, name: String) -> Option<&mut Self> {
        if self.name.eq(&name) {
            Some(self)
        } else {
            self.children
                .iter_mut()
                .find_map(|child| child.find_child_mut(name.clone()))
        }
    }

    fn get_weight(&self) -> i32 {
        self.children.iter().map(Program::get_weight).sum::<i32>() + self.weight
    }

    // Part 2
    fn get_correct_weight(&self) -> Option<i32> {
        match self.children.len() {
            0 => None,
            1 => self.children[0].get_correct_weight(),
            2 => self.children[0]
                .get_correct_weight()
                .or_else(|| self.children[1].get_correct_weight()),
            len => {
                let weights = self
                    .children
                    .iter()
                    .map(Program::get_weight)
                    .collect::<Vec<_>>();
                let mut index = len;

                let mut correct = weights[0];

                if weights[0] != weights[1] {
                    if weights[0] != weights[2] {
                        correct = weights[2];
                        index = 0;
                    } else {
                        index = 1;
                    }
                } else {
                    for i in 2..len {
                        if weights[i] != weights[0] {
                            index = i;
                        }
                    }
                }

                (index != len).then(|| {
                    let program = &self.children[index];
                    program.get_correct_weight().unwrap_or_else(|| {
                        correct
                            - program
                                .children
                                .iter()
                                .map(Program::get_weight)
                                .sum::<i32>()
                    })
                })
            }
        }
    }
}

// Part 1
fn parse_input(input: &str) -> Program {
    let mut buffer = input
        .lines()
        .map(|line| {
            let line = line.replace(" (", " ").replace(")", "");
            let mut tokens = line.split(" ");
            let name = tokens.next().unwrap().to_string();
            let weight = tokens.next().unwrap().parse::<i32>().unwrap();
            Program {
                name,
                weight,
                children: Vec::new(),
            }
        })
        .collect::<Vec<_>>();

    for line in input.lines().filter(|line| line.contains("->")) {
        let line = line.replace(" -> ", " ").replace(", ", " ");
        let mut tokens = line.split(" ");
        let name = tokens.next().unwrap().to_string();
        let _ = tokens.next();
        let children = tokens.map(|i| i.to_string()).collect::<Vec<_>>();

        let child_programs = buffer
            .iter()
            .enumerate()
            .filter_map(|(i, p)| children.contains(&p.name).then_some(i))
            .rev()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|i| buffer.swap_remove(i))
            .collect::<Vec<_>>();

        // Find the buffer

        if let Some(program) = buffer
            .iter_mut()
            .find_map(|i| i.find_child_mut(name.clone()))
        {
            program.children = child_programs;
        }
    }

    buffer.pop().unwrap()
}
