use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Condition {
    category: char,
    operand: Ordering,
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
            let (name, contents) = line[..line.len() - 1].split_once('{').unwrap();

            workflows.insert(
                name.into(),
                contents
                    .split(',')
                    .map(|rule| {
                        if rule.contains(':') {
                            let (condition, target) = rule.split_once(':').unwrap();
                            Rule {
                                condition: {
                                    let mut chars = condition.chars();
                                    Some(Condition {
                                        category: chars.next().unwrap(),
                                        operand: match chars.next().unwrap() {
                                            '>' => Ordering::Greater,
                                            '<' => Ordering::Less,
                                            '=' => Ordering::Equal,
                                            _ => panic!(),
                                        },
                                        value: chars.collect::<String>().parse::<i64>().unwrap(),
                                    })
                                },
                                target: match target {
                                    "A" => Target::Accepted,
                                    "R" => Target::Rejected,
                                    s => Target::Workflow(s.to_string()),
                                },
                            }
                        } else {
                            Rule {
                                condition: None,
                                target: match rule {
                                    "A" => Target::Accepted,
                                    "R" => Target::Rejected,
                                    s => Target::Workflow(s.to_string()),
                                },
                            }
                        }
                    })
                    .collect(),
            );
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
                            if part.get(&condition.category).unwrap().cmp(&condition.value)
                                == condition.operand
                            {
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

        let mut accepted = Vec::new();
        let mut frontier = vec![(
            HashMap::from(['x', 'm', 'a', 's'].map(|c| (c, Range { min: 1, max: 4000 }))),
            "in".to_string(),
        )];

        'frontier: while let Some((ranges, workflow_name)) = frontier.pop() {
            let workflow = self.workflows.get(&workflow_name).unwrap();

            for rule in workflow {
                let mut result = true;
                if let Some(condition) = &rule.condition {
                    let range = ranges.get(&condition.category).unwrap();
                    if range.min < range.max
                        && condition.value >= range.min
                        && condition.value <= range.max
                    {
                        frontier.extend(
                            [
                                (condition.value > range.min)
                                    .then_some((range.min, condition.value - 1)),
                                (condition.value < range.max)
                                    .then_some((condition.value + 1, range.max)),
                                Some((condition.value, condition.value)),
                            ]
                            .into_iter()
                            .flatten()
                            .map(|(min, max)| {
                                (
                                    {
                                        let mut new_ranges = ranges.clone();
                                        new_ranges.insert(condition.category, Range { min, max });
                                        new_ranges
                                    },
                                    workflow_name.clone(),
                                )
                            }),
                        );
                        continue 'frontier;
                    }

                    result = range.max.cmp(&condition.value) == condition.operand;
                }

                if result {
                    match &rule.target {
                        Target::Accepted => accepted.push(ranges.clone()),
                        Target::Rejected => {}
                        Target::Workflow(wkflow) => frontier.push((ranges, wkflow.to_string())),
                    }
                    continue 'frontier;
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
