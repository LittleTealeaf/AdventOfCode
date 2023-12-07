use std::cmp::Ordering;

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Clone)]
struct Hand {
    hand: Vec<usize>,
    bid: i64,
}

struct Solution {
    hands: Vec<Hand>,
}

impl Solution {
    pub fn new(input: &str) -> Self {
        Self {
            hands: input
                .lines()
                .filter_map(|line| {
                    let (hand, bid) = line.split_once(' ')?;
                    Some(Hand {
                        hand: hand
                            .chars()
                            .map(|char| match char {
                                'A' => 0,
                                'K' => 1,
                                'Q' => 2,
                                'J' => 3,
                                'T' => 4,
                                '9' => 5,
                                '8' => 6,
                                '7' => 7,
                                '6' => 8,
                                '5' => 9,
                                '4' => 10,
                                '3' => 11,
                                '2' => 12,
                                _ => panic!(),
                            })
                            .collect(),
                        bid: bid.parse().ok()?,
                    })
                })
                .collect(),
        }
    }

    pub fn part_1(&self) -> i64 {
        struct Entry {
            h: Hand,
            t: usize,
        }
        let mut hands = self
            .hands
            .iter()
            .map(|hand| {
                let mut count = [0; 13];
                for c in &hand.hand {
                    count[*c] += 1;
                }
                count.sort();
                count.reverse();
                let hand_type = if count[0] == 5 {
                    0
                } else if count[0] == 4 {
                    1
                } else if count[0] == 3 && count[1] == 2 {
                    2
                } else if count[0] == 3 {
                    3
                } else if count[0] == 2 && count[1] == 2 {
                    4
                } else if count[0] == 2 {
                    5
                } else {
                    6
                };

                Entry {
                    h: hand.clone(),
                    t: hand_type,
                }
            })
            .collect::<Vec<_>>();

        hands.sort_by(|a, b| match a.t.cmp(&b.t) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for i in 0..5 {
                    match a.h.hand[i].cmp(&b.h.hand[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        });

        hands
            .into_iter()
            .rev()
            .enumerate()
            .map(|(index, hand)| hand.h.bid * (index + 1) as i64)
            .sum::<i64>()
    }

    fn part_2(&self) -> i64 {
        struct Entry {
            h: Hand,
            t: usize,
        }
        let mut hands = self
            .hands
            .iter()
            .map(|hand| {
                let mut hand = hand.clone();
                hand.hand = hand
                    .hand
                    .iter()
                    .map(|i| match *i {
                        4.. => *i - 1,
                        3 => 12,
                        i => i,
                    })
                    .collect();

                let mut count = [0; 13];
                for c in &hand.hand {
                    count[*c] += 1;
                }
                let jokers = count[12];
                count[12] = 0;
                count.sort();
                count.reverse();
                count[0] += jokers;
                let hand_type = if count[0] == 5 {
                    0
                } else if count[0] == 4 {
                    1
                } else if count[0] == 3 && count[1] == 2 {
                    2
                } else if count[0] == 3 {
                    3
                } else if count[0] == 2 && count[1] == 2 {
                    4
                } else if count[0] == 2 {
                    5
                } else {
                    6
                };

                Entry {
                    h: hand.clone(),
                    t: hand_type,
                }
            })
            .collect::<Vec<_>>();

        hands.sort_by(|a, b| match a.t.cmp(&b.t) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for i in 0..5 {
                    match a.h.hand[i].cmp(&b.h.hand[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        });

        hands
            .into_iter()
            .rev()
            .enumerate()
            .map(|(index, hand)| hand.h.bid * (index + 1) as i64)
            .sum::<i64>()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 6440);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 5905);
}
