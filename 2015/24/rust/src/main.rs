fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
}

fn part_1(input: &Vec<u64>) -> u64 {
    let input = input.clone();
    let goal = input.iter().sum::<u64>() / 3;

    #[derive(Clone, Debug)]
    struct State {
        g1_sum: u64,
        g1_prod: Option<u64>,
        g1_len: usize,
        g2_sum: u64,
        stack: Vec<u64>,
        goal_sum: u64,
    }

    impl State {
        fn recusive(self, mut min_length: usize) -> Option<Self> {
            if self.g1_len > min_length {
                None
            } else if self.g1_sum < self.goal_sum {
                let mut min: Option<State> = None;

                let mut children = (0..self.stack.len())
                    .map(|i| {
                        let mut state = self.clone();
                        let value = state.stack.swap_remove(i);
                        state.g1_sum += value;
                        state.g1_len += 1;

                        if let Some(val) = state.g1_prod {
                            state.g1_prod = val.checked_mul(value)
                        }

                        state
                    })
                    .collect::<Vec<_>>();

                children.sort_by_key(|i| i.g1_sum);
                children.reverse();

                for state in children {
                    if let Some(result) = state.recusive(min_length) {
                        min_length = result.g1_len;
                        if let Some(m) = &min {
                            if let (Some(a), Some(b)) = (m.g1_prod, result.g1_prod) {
                                if a > b {
                                    min = Some(result)
                                }
                            }
                        } else {
                            min = Some(result);
                        }
                    }
                }
                min
            } else if self.g1_sum > self.goal_sum {
                None
            } else if self.g2_sum < self.goal_sum {
                (0..self.stack.len()).find_map(|i| {
                    let mut state = self.clone();
                    let value = state.stack.swap_remove(i);
                    state.g2_sum += value;
                    state.recusive(min_length)
                })
            } else if self.g2_sum > self.goal_sum {
                None
            } else {
                println!("{:?}", self);
                Some(self)
            }
        }
    }

    let state = State {
        g1_sum: 0,
        g1_len: 0,
        g1_prod: Some(1),
        g2_sum: 0,
        goal_sum: goal,
        stack: input,
    };

    let result = state.recusive(usize::MAX).unwrap();
    println!("{:?}", result);

    0
}
