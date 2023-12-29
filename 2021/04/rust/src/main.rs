fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Debug)]
struct Board(Vec<Vec<u32>>);

impl Board {
    fn bingo(&self, numbers: &[u32]) -> bool {
        let board = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| numbers.contains(item))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // horizontal and vertical
        (0..5).any(|i| {
            [
                (0..5).map(|j| board[i][j]).collect::<Vec<_>>(),
                (0..5).map(|j| board[j][i]).collect::<Vec<_>>(),
            ]
            .into_iter()
            .any(|i| i.into_iter().all(|i| i))
        })
    }

    fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().flat_map(|i| i.iter().copied())
    }
}

#[derive(Debug)]
struct Solution {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut sections = input.split("\n\n");

        let numbers = sections
            .next()
            .unwrap()
            .split(',')
            .map(|token| token.parse::<u32>().unwrap())
            .collect();

        let boards = sections
            .map(|section| {
                Board(
                    section
                        .lines()
                        .map(|line| {
                            line.split(' ')
                                .filter_map(|token| token.parse::<u32>().ok())
                                .collect()
                        })
                        .collect(),
                )
            })
            .collect();

        Self { numbers, boards }
    }

    fn part_1(&self) -> u32 {
        for i in 0..self.numbers.len() {
            let numbers = &self.numbers[..=i];
            if let Some(board) = self.boards.iter().find(|board| board.bingo(numbers)) {
                let unmarked_sum = board.iter().filter(|i| !numbers.contains(i)).sum::<u32>();
                let winning_number = numbers[i];
                return unmarked_sum * winning_number;
            }
        }
        todo!()
    }

    fn part_2(&self) -> u32 {
        let mut boards = self.boards.iter().collect::<Vec<_>>();
        for i in 0..self.numbers.len() {
            let numbers = &self.numbers[..=i];
            if boards.len() > 1 {
                boards.retain(|board| !board.bingo(numbers));
            } else if boards[0].bingo(numbers) {
                let unmarked_sum = boards[0]
                    .iter()
                    .filter(|i| !numbers.contains(i))
                    .sum::<u32>();
                let winning_number = numbers[i];
                return unmarked_sum * winning_number;
            }
        }
        panic!()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 4512);
}
#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 1924);
}
