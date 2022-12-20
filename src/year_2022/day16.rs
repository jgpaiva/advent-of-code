use std::{
    collections::{hash_map::Entry, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[cfg(test)]
use crate::utils;

use anyhow::Context;
use regex::Regex;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 1651);
    assert_eq!(part2(&input), 1707);
}

pub fn part2(lines: &str) -> u32 {
    let nodes = parse_nodes(lines);
    let paths: Vec<Vec<Path>> = (0..nodes.len()).map(|s| paths_from(s, &nodes)).collect();
    let interesting_nodes: BTreeSet<_> = nodes
        .iter()
        .filter_map(|n| (n.flow > 0).then_some(UnopenedNode(n.flow, n.id)))
        .collect();
    let aa_id = nodes.iter().find(|n| n.name == "AA").unwrap().id;
    let PathState {
        paths: _paths,
        value,
        opened_nodes: _opened_nodes,
        nodes_to_open: _nodes_to_open,
    } = best_path(aa_id, interesting_nodes, &paths, 26_u16, 2);
    #[cfg(test)]
    {
        println!(
            "path1: {}",
            _paths[0]
                .0
                .iter()
                .map(|n| nodes[*n].name.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        println!(
            "path2: {}",
            _paths[1]
                .0
                .iter()
                .map(|n| nodes[*n].name.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        println!("value: {value}, rounds_left1: {}, rounds_left2: {}, opened_nodes: {:?}, nodes_to_open: {_nodes_to_open:?}", 
        _paths[0].1, _paths[1].1,_opened_nodes.into_iter().map(|(n,r,v)| (nodes[n].name.clone(),r,v)).collect::<Vec<_>>());
    }
    value
}

#[derive(Debug)]
struct Node {
    name: String,
    id: usize,
    flow: u32,
    neighbours: Vec<usize>,
}

/// AA - rate=0;  -> DD, II, BB
/// BB - rate=13; -> CC, AA
/// CC - rate=2;  -> DD, BB
/// DD - rate=20; -> CC, AA, EE
/// EE - rate=3;  -> FF, DD
/// FF - rate=0;  -> EE, GG
/// GG - rate=0;  -> FF, HH
/// HH - rate=22; -> GG
/// II - rate=0;  -> AA, JJ
/// JJ - rate=21; -> II
///
/// CC 02 - DD 20 - EE 03 - FF 00 - GG 00 - HH 22
///  |     |
/// BB 13 - AA 00 - II 00 - JJ 21
///
/// interesting:
/// B, C, D, E, H, J
///
/// possible:
/// A, B, C, D, E, H, J
/// A, B, C, D, E, J, H
/// A, B, C, D,#E, H, J -- repeat
///
pub fn part1(lines: &str) -> u32 {
    let nodes = parse_nodes(lines);
    let paths: Vec<Vec<Path>> = (0..nodes.len()).map(|s| paths_from(s, &nodes)).collect();
    let interesting_nodes: BTreeSet<_> = nodes
        .iter()
        .filter_map(|n| (n.flow > 0).then_some(UnopenedNode(n.flow, n.id)))
        .collect();
    let aa_id = nodes.iter().find(|n| n.name == "AA").unwrap().id;
    let PathState {
        paths: _paths,
        value,
        opened_nodes: _opened_nodes,
        nodes_to_open: _nodes_to_open,
    } = best_path(aa_id, interesting_nodes, &paths, 30_u16, 1);
    #[cfg(test)]
    {
        println!(
            "path: {}",
            _paths
                .first()
                .unwrap()
                .0
                .iter()
                .map(|n| nodes[*n].name.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        println!("value: {value}, rounds_left: {}, opened_nodes: {_opened_nodes:?}, nodes_to_open: {_nodes_to_open:?}", _paths
        .first()
        .unwrap()
        .1);
    }
    value
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct UnopenedNode(u32, usize);
impl PartialOrd for UnopenedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnopenedNode {
    /// reverse comparison order, so that larger ones come first
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

type Path = Vec<usize>;

#[derive(Eq, PartialEq)]
struct PathState {
    paths: Vec<(Path, u16)>,
    value: u32,
    opened_nodes: Vec<(usize, u16, u32)>,
    nodes_to_open: BTreeSet<UnopenedNode>,
}

impl PathState {
    fn potential_value(&self) -> u32 {
        // 5 rounds
        // at round 1
        // 2 valves left
        // 1: 1 round to go A,
        // 2: 1 round to enable A,
        // 3: 1 round to go to B, (A flows)
        // 4: 1 round to enable B, (A flows)
        // 5: 1 round of A, B flowing
        // 3*A + B
        // (5 - 2) * A + (5 - 2 - 2)*B
        self.value
            + self
                .nodes_to_open
                .iter()
                .enumerate()
                .map(|(i, UnopenedNode(flow, _n))| {
                    (self
                        .paths
                        .iter()
                        .map(|(_p, rounds_left)| rounds_left)
                        .sum::<u16>() as i32
                        - (2 * i as i32))
                        .max(0) as u32
                        * flow
                })
                .sum::<u32>()
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.potential_value().cmp(&other.potential_value())
    }
}

fn best_path(
    start: usize,
    interesting_nodes: BTreeSet<UnopenedNode>,
    paths: &[Vec<Path>],
    starting_round: u16,
    num_explorers: u8,
) -> PathState {
    let mut to_explore = BinaryHeap::from([PathState {
        paths: (0..num_explorers)
            .map(|_| (vec![start], starting_round))
            .collect(),
        value: 0_u32,
        opened_nodes: Default::default(),
        nodes_to_open: interesting_nodes,
    }]);
    loop {
        let Some(PathState{ paths: past_paths, value, opened_nodes, nodes_to_open }) = to_explore.pop() else {
                unreachable!("we've ran out of paths, and none of them got to close all nodes?");
            };
        if nodes_to_open.is_empty() {
            // done, found the cheapest one
            return PathState {
                paths: past_paths,
                value,
                opened_nodes,
                nodes_to_open,
            };
        }
        for (current_path_index, (path, rounds_left)) in past_paths.iter().enumerate() {
            let current_node = *path.last().unwrap();
            if current_node == 0 && current_path_index == 1 && past_paths[0].0.len() == 1 {
                // don't repeat the paths with 2 diff orders
                continue;
            }
            for next_node in nodes_to_open.iter().cloned() {
                let mut next_nodes_to_open = nodes_to_open.clone();
                next_nodes_to_open.remove(&next_node);
                let next_path = &paths[current_node][next_node.1];
                let current_round_cost = 1 + (next_path.len() as u16 - 1);

                let rounds_left = (*rounds_left as i32 - current_round_cost as i32).max(0) as u16;
                let node_value = rounds_left as u32 * next_node.0;
                let value = value + node_value;

                let mut next_opened_nodes = opened_nodes.clone();
                next_opened_nodes.push((next_node.1, starting_round - rounds_left, node_value));

                let past_paths = past_paths
                    .iter()
                    .enumerate()
                    .map(|(i, past_path)| {
                        if i == current_path_index {
                            let mut path = path.clone();
                            path.extend_from_slice(&next_path.as_slice()[1..]);
                            (path, rounds_left)
                        } else {
                            past_path.clone()
                        }
                    })
                    .collect();

                to_explore.push(PathState {
                    paths: past_paths,
                    value,
                    opened_nodes: next_opened_nodes,
                    nodes_to_open: next_nodes_to_open,
                });
            }
        }
    }
}

#[test]
fn test_paths_from() {
    assert_eq!(
        paths_from(
            0,
            &[
                Node {
                    name: "".into(),
                    id: 0,
                    flow: 0,
                    neighbours: vec![1, 2]
                },
                Node {
                    name: "".into(),
                    id: 1,
                    flow: 0,
                    neighbours: vec![0, 3]
                },
                Node {
                    name: "".into(),
                    id: 2,
                    flow: 0,
                    neighbours: vec![0, 3]
                },
                Node {
                    name: "".into(),
                    id: 3,
                    flow: 0,
                    neighbours: vec![1, 2]
                }
            ],
        ),
        vec![vec![], vec![0, 1], vec![0, 2], vec![0, 1, 3]]
    );
}

fn paths_from(start: usize, nodes: &[Node]) -> Vec<Path> {
    let mut to_explore = VecDeque::from([vec![start]]);
    let mut paths: Vec<_> = (0..nodes.len()).map(|_| Vec::with_capacity(0)).collect();
    // path to one-self is trivial
    let mut found_paths = 1;
    loop {
        if found_paths == nodes.len() {
            return paths;
        }
        let Some(current_path) = to_explore.pop_front() else {
            unreachable!("explored everything and didn't find all uninteresting nodes?");
        };
        let current = *current_path.last().unwrap();
        for neighbour in nodes[current].neighbours.iter().copied() {
            if paths[neighbour].is_empty() && neighbour != start {
                let mut new_path = current_path.clone();
                new_path.push(neighbour);
                paths[neighbour] = new_path.clone();
                to_explore.push_back(new_path);
                found_paths += 1;
            }
        }
    }
}

fn parse_nodes(lines: &str) -> Vec<Node> {
    let re =
        Regex::new(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let captures_iter = re.captures_iter(lines);
    let mut mapping = HashMap::new();
    let nodes = captures_iter
        .into_iter()
        .enumerate()
        .map(|(i, line_captures)| {
            assert_eq!(line_captures.len(), 4);
            let name = line_captures[1].to_owned();
            let flow: u32 = line_captures[2].parse().context(format!(
                "error while parsing flow rate {}",
                &line_captures[1]
            ))?;
            mapping.insert(name.to_owned(), i);
            let neighbours: Vec<String> = line_captures[3].split(", ").map(String::from).collect();
            Ok((i, name, flow, neighbours))
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("should always parse");
    nodes
        .into_iter()
        .map(|(id, name, flow, neighbours)| Node {
            id,
            name,
            flow,
            neighbours: neighbours
                .into_iter()
                .map(|n| {
                    *mapping
                        .get(&n)
                        .unwrap_or_else(|| panic!("all nodes should be mapped, but {n} wasn't"))
                })
                .collect(),
        })
        .collect::<Vec<_>>()
}
