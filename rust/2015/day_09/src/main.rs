use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
    distance: i32,
}

impl From<String> for Edge {
    fn from(value: String) -> Self {
        let tokens = value.split(" ").collect::<Vec<_>>();

        let from = String::from(tokens[0]);
        let to = String::from(tokens[2]);
        let distance = tokens[4].parse().unwrap();

        Self { from, to, distance }
    }
}

fn get_nodes(edges: &Vec<Edge>) -> Vec<String> {
    let mut nodes = Vec::new();

    for edge in edges {
        if !nodes.contains(&edge.from) {
            nodes.push(edge.from.clone());
        }
    }

    nodes
}

fn part_1(edges: Vec<Edge>) -> i32 {
    let distance = 0;

    distance
}

fn main() {
    {
        let file = File::open("input.txt").unwrap();
        let edges = BufReader::new(file)
            .lines()
            .into_iter()
            .map(Result::unwrap)
            .map(Edge::from)
            .collect::<Vec<_>>();
        part_1(edges);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let strings = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];

        let edges = strings
            .into_iter()
            .map(String::from)
            .map(Edge::from)
            .collect::<Vec<_>>();

        let value = part_1(edges);

        assert_eq!(value, 605);
    }
}
