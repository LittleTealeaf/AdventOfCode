use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

#[derive(Debug, Clone)]
struct Route {
    from: String,
    to: String,
    distance: i32,
}

impl From<String> for Route {
    fn from(value: String) -> Self {
        let tokens = value.split(" ").collect::<Vec<_>>();

        let from = String::from(tokens[0]);
        let to = String::from(tokens[2]);
        let distance = tokens[4].parse().unwrap();

        Self { from, to, distance }
    }
}

#[derive(Debug)]
struct Edge {
    endpoints: [usize; 2],
    distance: i32,
}

impl Edge {
    fn has_node(&self, node: usize) -> bool {
        self.endpoints[0] == node || self.endpoints[1] == node
    }
}

fn index_of<T: PartialEq>(array: &Vec<T>, item: &T) -> Option<usize> {
    for i in 0..(array.len()) {
        if array[i].eq(&item) {
            return Some(i);
        }
    }
    None
}

fn convert_routes_to_edges(routes: Vec<Route>) -> (Vec<Edge>, usize) {
    let mut edges = Vec::new();
    let mut nodes = Vec::new();

    for Route { from, to, distance } in routes {
        let endpoints = [
            match index_of(&nodes, &from) {
                Some(index) => index,
                None => {
                    nodes.push(from);
                    nodes.len() - 1
                }
            },
            match index_of(&nodes, &to) {
                Some(index) => index,
                None => {
                    nodes.push(to);
                    nodes.len() - 1
                }
            },
        ];

        edges.push(Edge {
            endpoints,
            distance,
        });
    }

    (edges, nodes.len())
}

struct Snapshot {
    current_node: usize,
    distance_traveled: i32,
    nodes_left: Vec<usize>,
}

fn part_1(routes: Vec<Route>) -> i32 {
    let (mut edges, node_count) = convert_routes_to_edges(routes);
    edges.sort_by_key(|edge| -> i32 { edge.distance });
    let [start, _] = edges.last().unwrap().endpoints;

    let mut distances: Vec<Vec<i32>> = Vec::new();

    for _ in 0..node_count {
        let mut vec = Vec::new();
        for _ in 0..node_count {
            vec.push(-1);
        }
        distances.push(vec);
    }

    for edge in edges {
        let [start, end] = edge.endpoints;
        distances[start][end] = edge.distance;
        distances[end][start] = edge.distance;
    }

    let mut snapshots = {
        let nodes_left = (0..node_count)
            .filter(|i| -> bool { i != &start })
            .collect::<Vec<_>>();

        vec![Snapshot {
            current_node: start,
            distance_traveled: 0,
            nodes_left,
        }]
    };

    let mut min_distance = -1;

    while let Some(snapshot) = snapshots.pop() {
        if snapshot.nodes_left.len() == 0 {
            if min_distance == -1 || min_distance > snapshot.distance_traveled {
                min_distance = snapshot.distance_traveled;
            }
            continue;
        }
        
        let mut nodes_clone = snapshot.nodes_left.clone();

        for next in snapshot.nodes_left {
            let index = index_of(&nodes_clone, &next).unwrap();
            nodes_clone.remove(index);
            snapshots.push(Snapshot {
                current_node: next,
                distance_traveled: (snapshot.distance_traveled + distances[next][snapshot.current_node]),
                nodes_left: nodes_clone.clone()
            });
            nodes_clone.push(next);
        }
    }

    min_distance
}

fn main() {
    {
        let file = File::open("input.txt").unwrap();
        let edges = BufReader::new(file)
            .lines()
            .into_iter()
            .map(Result::unwrap)
            .map(Route::from)
            .collect::<Vec<_>>();
        let result = part_1(edges);
        println!("Part 1: {}", result);
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
            .map(Route::from)
            .collect::<Vec<_>>();

        let value = part_1(edges);

        assert_eq!(value, 605);
    }
}
