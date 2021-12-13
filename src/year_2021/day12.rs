use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day12");
    assert_eq!(part1(input.clone()), 226);
    assert_eq!(part2(input.clone()), 3509);
}

#[allow(dead_code)]
pub fn part2(input: String) -> u32 {
    let edges = create_graph(input);
    let start = *edges.keys().find(|x| x.is_start).unwrap();
    find_routes(start, &edges, vec![start], part2_route_validator)
}

#[allow(dead_code)]
fn part2_route_validator(route: &[Node], n: Node) -> bool {
    let mut counter: HashMap<Node, u32> = HashMap::new();
    for node in route
        .iter()
        .cloned()
        .chain([n].into_iter())
        .filter(|x| !x.is_large)
    {
        *(counter.entry(node).or_insert(0)) += 1
    }
    counter.values().into_iter().filter(|x| **x > 2).count() == 0
        && counter.values().into_iter().filter(|x| **x == 2).count() <= 1
}

pub fn part1(input: String) -> u32 {
    let edges = create_graph(input);
    let start = *edges.keys().find(|x| x.is_start).unwrap();
    find_routes(start, &edges, vec![start], part1_route_validator)
}

fn part1_route_validator(route: &[Node], n: Node) -> bool {
    let mut counter: HashMap<Node, u32> = HashMap::new();
    for node in route
        .iter()
        .cloned()
        .chain([n].into_iter())
        .filter(|x| !x.is_large)
    {
        *(counter.entry(node).or_insert(0)) += 1
    }
    counter.values().into_iter().filter(|x| **x > 1).count() == 0
}

fn find_routes(
    current: Node,
    edges: &HashMap<Node, Vec<Node>>,
    route: Vec<Node>,
    route_validator: fn(&[Node], Node) -> bool,
) -> u32 {
    if current.is_end {
        return 1;
    }
    let empty_vec: Vec<Node> = vec![];
    let next = edges
        .get(&current)
        .unwrap_or(&empty_vec)
        .iter()
        .filter(|n| !n.is_start && route_validator(&route, **n));
    next.cloned()
        .map(|n| {
            find_routes(
                n,
                edges,
                route.iter().cloned().chain([n].into_iter()).collect(),
                route_validator,
            )
        })
        .sum()
}

fn create_graph(input: String) -> HashMap<Node, Vec<Node>> {
    let input: Vec<(&str, &str)> = input
        .split_terminator('\n')
        .map(|line| line.split_once('-').unwrap())
        .collect();
    let mut edges: HashMap<Node, Vec<Node>> = HashMap::new();
    let mut mapping: HashMap<&str, u32> = HashMap::new();
    let mut node_counter: u32 = 0;
    for (x1, x2) in input {
        let large = x1.chars().all(|c| c.is_uppercase());
        let x1 = Node {
            id: *mapping.entry(x1).or_insert_with(|| {
                node_counter += 1;
                node_counter
            }),
            is_large: large,
            is_start: x1 == "start",
            is_end: x1 == "end",
        };
        let large = x2.chars().all(|c| c.is_uppercase());
        let x2 = Node {
            id: *mapping.entry(x2).or_insert_with(|| {
                node_counter += 1;
                node_counter
            }),
            is_large: large,
            is_start: x2 == "start",
            is_end: x2 == "end",
        };
        edges.entry(x1).or_insert_with(Vec::new).push(x2);
        edges.entry(x2).or_insert_with(Vec::new).push(x1);
    }
    edges
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Node {
    id: u32,
    is_large: bool,
    is_start: bool,
    is_end: bool,
}
