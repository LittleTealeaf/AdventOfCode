fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));

    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[derive(Clone, Copy)]
struct Race {
    time: i64,
    distance: i64,
}

struct Solution {
    races: Vec<Race>,
    race: Race,
}

impl Solution {
    fn new(input: &str) -> Self {
        let mut input = input.to_string();
        while input.contains("  ") {
            input = input.replace("  ", " ");
        }
        let mut lines = input.lines();
        let mut lines_2 = lines.clone();
        let mut time_iter = lines.next().unwrap().split(' ');
        let mut dist_iter = lines.next().unwrap().split(' ');
        let (_, _) = (time_iter.next(), dist_iter.next());

        let mut races = Vec::new();

        while let (Some(time), Some(dist)) = (time_iter.next(), dist_iter.next()) {
            races.push(Race {
                time: time.parse().unwrap(),
                distance: dist.parse().unwrap(),
            });
        }

        let mut time_iter = lines_2.next().unwrap().split(' ');
        let mut dist_iter = lines_2.next().unwrap().split(' ');
        let (_, _) = (time_iter.next(), dist_iter.next());
        let time_str = time_iter.collect::<String>();
        let dist_str = dist_iter.collect::<String>();

        let race = Race {
            time: time_str.parse().unwrap(),
            distance: dist_str.parse().unwrap(),
        };

        Self { races, race }
    }

    fn part_1(&self) -> usize {
        self.races
            .iter()
            .map(|race| {
                (0..=race.time)
                    .filter(|time| (race.time - time) * (time) > race.distance)
                    .count()
            })
            .product()
    }

    fn part_2(&self) -> usize {
        (0..=self.race.time)
            .filter(|time| (self.race.time - time) * (time) > self.race.distance)
            .count()
    }
}

#[test]
fn test_part_1() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_1(), 288);
}

#[test]
fn test_part_2() {
    let solution = Solution::new(include_str!("../../sample.txt"));
    assert_eq!(solution.part_2(), 71503);
}
