use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

#[cfg(test)]
use crate::utils;

use anyhow::Context;
use regex::Regex;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 26);
    //assert_eq!(part2(&input), 56000011);
}

pub fn part2(lines: &str) -> u64 {
    0
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
/// CC$ - DD$ - EE$ - FF# - GG# - HH$
///  |     |
/// BB$ - AA# - II# - JJ$
///
/// interesting:
/// B, C, D, E, H, J
///
/// possible:
/// A, B, C, D, E, H, J
/// A, B, C, D, E, J, H
/// A, B, C, D,#E, H, J -- repeat
///
pub fn part1(lines: &str) -> i32 {
    let nodes = parse_nodes(lines);
    let paths: Vec<Vec<Vec<usize>>> = (0..nodes.len()).map(|s| paths_from(s, &nodes)).collect();
    let interesting_nodes: HashSet<_> = nodes
        .iter()
        .filter_map(|n| (n.flow > 0).then_some(n.id))
        .collect();
    let valid_paths = valid_paths(vec![0], interesting_nodes, &paths);
    0
}

fn valid_paths(
    current_path: Vec<usize>,
    interesting_nodes: HashSet<usize>,
    paths: &Vec<Vec<Vec<usize>>>,
) {
    let current_node = *current_path.last().unwrap();
    for next_node in interesting_nodes.iter().cloned() {
        let next_path = paths[current_node][next_node].clone();
        let mut interesting_nodes = interesting_nodes.clone();
        for n in next_path.iter() {
            interesting_nodes.remove(n);
        }
        if interesting_nodes.is_empty() {
            // already visited all of the interesting nodes
            //println!("{:?}:{:?}", current_path, next_path);
        } else if current_path.len() + next_path.len() - 1 <= 30 {
            let mut current_path = current_path.clone();
            assert!(
                next_path.len() > 1,
                "next_path: {next_path:?} current_path: {current_path:?} next_node: {next_node} paths: {paths:?}"
            );
            current_path.extend_from_slice(&next_path.as_slice()[1..]);
            valid_paths(current_path, interesting_nodes, paths)
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

fn paths_from(start: usize, nodes: &[Node]) -> Vec<Vec<usize>> {
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
