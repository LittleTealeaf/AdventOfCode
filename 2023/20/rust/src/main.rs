use std::collections::{HashMap, VecDeque};

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction { inputs: HashMap<String, bool> },
    Broadcaster,
    SandMachine(bool),
}

#[derive(Clone)]
struct Node {
    module: Module,
    targets: Vec<String>,
}

#[derive(Clone)]
struct Solution {
    modules: HashMap<String, Node>,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut modules = HashMap::from_iter(input.lines().map(|line| {
            let (identifier, targets) = line.split_once(" -> ").unwrap();

            let (name, module) = if identifier == "broadcaster" {
                (identifier, Module::Broadcaster)
            } else if &identifier[..1] == "%" {
                (&identifier[1..], Module::FlipFlop(false))
            } else {
                (
                    &identifier[1..],
                    Module::Conjunction {
                        inputs: HashMap::new(),
                    },
                )
            };

            let targets = targets
                .split(", ")
                .map(|target| target.to_string())
                .collect();

            (name.to_string(), Node { module, targets })
        }));

        for name in modules.keys().cloned().collect::<Vec<_>>() {
            let node = modules.get(&name).unwrap().clone();
            for target in &node.targets {
                if let Some(Module::Conjunction { inputs }) =
                    &mut modules.get_mut(&target.clone()).map(|i| &mut i.module)
                {
                    inputs.insert(name.clone(), false);
                }
            }
        }

        Self { modules }
    }

    fn part_1(&self) -> usize {
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        let mut modules = self.modules.clone();

        for _ in 0..1000 {
            let mut queue =
                VecDeque::from([(String::from("broadcaster"), String::from("button"), false)]);

            while let Some((name, source, pulse)) = queue.pop_front() {
                if pulse {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }

                if let Some(node) = modules.get_mut(&name) {
                    let pulse = match &mut node.module {
                        Module::FlipFlop(value) => {
                            if !pulse {
                                *value = !*value;
                                *value
                            } else {
                                continue;
                            }
                        }
                        Module::Conjunction { inputs } => {
                            inputs.insert(source, pulse);
                            !inputs.values().cloned().all(|i| i)
                        }
                        Module::Broadcaster => pulse,
                        Module::SandMachine(value) => {
                            if !pulse {
                                *value = true;
                            }
                            continue;
                        }
                    };

                    for target in &node.targets {
                        queue.push_back((target.clone(), name.clone(), pulse));
                    }
                }
            }
        }

        high_pulses * low_pulses
    }

    // TODO: either run this fully, or implement better method
    fn part_2(&self) -> usize {
        let mut modules = self.modules.clone();
        modules.insert(
            "rx".to_string(),
            Node {
                targets: Vec::new(),
                module: Module::SandMachine(false),
            },
        );

        fn machine_on(modules: &HashMap<String, Node>) -> bool {
            if let Some(node) = modules.get(&String::from("rx")) {
                if let Module::SandMachine(value) = node.module {
                    return value;
                }
            }
            false
        }

        let mut presses = 0;

        while !machine_on(&modules) {
            presses += 1;
            let mut queue =
                VecDeque::from([(String::from("broadcaster"), String::from("button"), false)]);

            while let Some((name, source, pulse)) = queue.pop_front() {
                if let Some(node) = modules.get_mut(&name) {
                    let pulse = match &mut node.module {
                        Module::FlipFlop(value) => {
                            if !pulse {
                                *value = !*value;
                                *value
                            } else {
                                continue;
                            }
                        }
                        Module::Conjunction { inputs } => {
                            inputs.insert(source, pulse);
                            !inputs.values().cloned().all(|i| i)
                        }
                        Module::Broadcaster => pulse,
                        Module::SandMachine(value) => {
                            if !pulse {
                                *value = true;
                            }
                            continue;
                        }
                    };

                    for target in &node.targets {
                        queue.push_back((target.clone(), name.clone(), pulse));
                    }
                }
            }
        }

        presses
    }
}

#[test]
fn test_part_1_example_1() {
    let solution = Solution::new(include_str!("../../example_1.txt"));
    assert_eq!(solution.part_1(), 32000000);
}
