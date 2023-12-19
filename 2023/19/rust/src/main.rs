use std::collections::HashMap;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Condition {
    category: char,
    operand: char,
    value: i64,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Target {
    Accepted,
    Rejected,
    Workflow(String),
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Rule {
    condition: Option<Condition>,
    target: Target,
}

struct Solution {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<HashMap<char, i64>>,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut workflows = HashMap::new();
        let mut lines = input.lines();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (name, contents) = line.split_once('{').unwrap();
            let contents = contents.replace('}', "");

            let rules = contents
                .split(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let (condition, target) = rule.split_once(':').unwrap();
                        Rule {
                            condition: {
                                let mut chars = condition.chars();
                                let category = chars.next().unwrap();
                                let operand = chars.next().unwrap();
                                let value = chars.collect::<String>().parse::<i64>().unwrap();
                                Some(Condition {
                                    category,
                                    operand,
                                    value,
                                })
                            },
                            target: match target {
                                "R" => Target::Rejected,
                                "A" => Target::Accepted,
                                name => Target::Workflow(name.to_string()),
                            },
                        }
                    } else {
                        Rule {
                            condition: None,
                            target: match rule {
                                "R" => Target::Rejected,
                                "A" => Target::Accepted,
                                name => Target::Workflow(name.to_string()),
                            },
                        }
                    }
                })
                .collect();

            workflows.insert(name.to_string(), rules);
        }

        Self {
            workflows,
            parts: lines
                .map(|line| {
                    line[1..line.len() - 1]
                        .split(',')
                        .map(|sec| {
                            let mut chars = sec.chars();
                            let category = chars.next().unwrap();
                            let _ = chars.next().unwrap();
                            let value = chars.collect::<String>().parse::<i64>().unwrap();
                            (category, value)
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> i64 {
        self.parts
            .iter()
            .filter_map(|part| {
                let mut status = Target::Workflow("in".to_string());

                'workflow: while let Target::Workflow(workflow) = &status {
                    let workflow = self.workflows.get(workflow).unwrap();
                    for rule in workflow {
                        if let Some(condition) = &rule.condition {
                            let result = match condition.operand {
                                '>' => part.get(&condition.category).unwrap() > &condition.value,
                                '<' => part.get(&condition.category).unwrap() < &condition.value,
                                '=' => part.get(&condition.category).unwrap() == &condition.value,
                                _ => panic!(),
                            };
                            if result {
                                status = rule.target.clone();
                                continue 'workflow;
                            }
                        } else {
                            status = rule.target.clone();
                            continue 'workflow;
                        }
                    }
                }

                matches!(status, Target::Accepted).then(|| part.values())
            })
            .flatten()
            .sum::<i64>()
    }

    fn part_2(&self) -> i64 {
        #[derive(Clone)]
        struct Range {
            min: i64,
            max: i64,
        }

        let mut accepted: Vec<HashMap<char, Range>> = Vec::new();

        let mut frontier = vec![(
            HashMap::from([
                ('x', Range { min: 1, max: 4000 }),
                ('m', Range { min: 1, max: 4000 }),
                ('a', Range { min: 1, max: 4000 }),
                ('s', Range { min: 1, max: 4000 }),
            ]),
            "in".to_string(),
        )];

        'frontier: while let Some((mut ranges, workflow_name)) = frontier.pop() {
            let workflow = self.workflows.get(&workflow_name).unwrap();

            for rule in workflow {
                if let Some(condition) = &rule.condition {
                    let range = ranges.get(&condition.category).unwrap();
                    match condition.operand {
                        '>' => {
                            if condition.value < range.max {
                                if condition.value < range.min {
                                    match &rule.target {
                                        Target::Accepted => {
                                            accepted.push(ranges.clone());
                                        }
                                        Target::Workflow(workflow) => {
                                            frontier.push((ranges.clone(), workflow.clone()))
                                        }
                                        _ => {}
                                    }
                                    continue 'frontier;
                                } else {
                                    let mut new_ranges = ranges.clone();
                                    new_ranges.get_mut(&condition.category).unwrap().min =
                                        condition.value + 1;
                                    ranges.get_mut(&condition.category).unwrap().max =
                                        condition.value;
                                    match &rule.target {
                                        Target::Accepted => {
                                            accepted.push(new_ranges);
                                        }
                                        Target::Workflow(workflow) => {
                                            frontier.push((new_ranges, workflow.clone()))
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        '<' => {
                            if condition.value > range.min {
                                if condition.value > range.max {
                                    match &rule.target {
                                        Target::Accepted => {
                                            accepted.push(ranges.clone());
                                        }
                                        Target::Workflow(workflow) => {
                                            frontier.push((ranges.clone(), workflow.clone()))
                                        }
                                        _ => {}
                                    }
                                    continue 'frontier;
                                } else {
                                    let mut new_ranges = ranges.clone();
                                    new_ranges.get_mut(&condition.category).unwrap().max =
                                        condition.value - 1;
                                    ranges.get_mut(&condition.category).unwrap().min =
                                        condition.value;
                                    match &rule.target {
                                        Target::Accepted => {
                                            accepted.push(new_ranges);
                                        }
                                        Target::Workflow(workflow) => {
                                            frontier.push((new_ranges, workflow.clone()))
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        '=' => {
                            if condition.value == range.min {
                                let mut new_ranges = ranges.clone();
                                new_ranges.get_mut(&condition.category).unwrap().min =
                                    condition.value + 1;
                                ranges.get_mut(&condition.category).unwrap().max = condition.value;
                                match &rule.target {
                                    Target::Accepted => {
                                        accepted.push(new_ranges);
                                    }
                                    Target::Workflow(workflow) => {
                                        frontier.push((new_ranges, workflow.clone()))
                                    }
                                    _ => {}
                                }
                            } else if condition.value == range.max {
                                let mut new_ranges = ranges.clone();
                                new_ranges.get_mut(&condition.category).unwrap().max =
                                    condition.value - 1;
                                ranges.get_mut(&condition.category).unwrap().min = condition.value;
                                match &rule.target {
                                    Target::Accepted => {
                                        accepted.push(new_ranges);
                                    }
                                    Target::Workflow(workflow) => {
                                        frontier.push((new_ranges, workflow.clone()))
                                    }
                                    _ => {}
                                }
                            } else if condition.value > range.min && condition.value < range.max {
                                let mut greater_range = ranges.clone();
                                greater_range.get_mut(&condition.category).unwrap().min =
                                    condition.value + 1;
                                frontier.push((greater_range, workflow_name.clone()));
                                ranges.get_mut(&condition.category).unwrap().max =
                                    condition.value - 1;
                                let mut equal_range = ranges.clone();
                                {
                                    let range = equal_range.get_mut(&condition.category).unwrap();
                                    range.min = condition.value;
                                    range.max = condition.value;
                                }
                                match &rule.target {
                                    Target::Accepted => {
                                        accepted.push(equal_range);
                                    }
                                    Target::Workflow(workflow) => {
                                        frontier.push((equal_range, workflow.clone()));
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => panic!(),
                    }
                } else {
                    match &rule.target {
                        Target::Accepted => accepted.push(ranges),
                        Target::Rejected => continue 'frontier,
                        Target::Workflow(new_workflow) => {
                            frontier.push((ranges, new_workflow.clone()))
                        }
                    }
                    break;
                }
            }
        }

        accepted
            .into_iter()
            .map(|ranges| {
                ranges
                    .into_values()
                    .map(|range| range.max - range.min + 1)
                    .product::<i64>()
            })
            .sum::<i64>()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 19114);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 167409079868000);
}
