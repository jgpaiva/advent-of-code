use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2018/test_day7");
    assert_eq!(part1(&input), "CABDFE");
    assert_eq!(part2(&input, 2, 0), 15);
}

lazy_static! {
    static ref MAPPING: HashMap<char, usize> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, v)| (v, i + 1))
        .collect::<HashMap<char, usize>>();
}

pub fn part2(lines: &Vec<String>, worker_n: usize, sleep_time: i32) -> i32 {
    let input = parse(lines);
    let mut deps = input.deps.iter().fold(HashMap::new(), |mut accum, d| {
        let mut v = accum.get_mut(&d.from);
        if v.is_none() {
            accum.insert(d.from, HashSet::new());
            v = accum.get_mut(&d.from);
        }
        v.unwrap().insert(&d.to);
        accum
    });
    let all: HashSet<char> = deps.values().flatten().map(|x| **x).collect();
    for i in all {
        if deps.get(&i).is_none() {
            deps.insert(i, HashSet::new());
        }
    }
    let mut done: HashSet<char> = HashSet::new();
    let mut workers: HashMap<usize, Option<(char, i32)>> =
        (0..worker_n).map(|x| (x, None)).collect();
    let mut time = 0;
    loop {
        for worker_index in 0..worker_n {
            process_work_for_worker(
                &mut deps,
                worker_index,
                &mut workers,
                time,
                sleep_time,
                &mut done,
            );
        }
        for worker_index in 0..worker_n {
            get_work_for_worker(&mut deps, worker_index, &mut workers, time);
        }
        if workers.values().flatten().next().is_none() {
            break;
        }
        time += 1;
        assert!(time < 1000);
    }
    time
}

fn get_work_for_worker(
    deps: &mut HashMap<char, HashSet<&char>>,
    worker_index: usize,
    workers: &mut HashMap<usize, Option<(char, i32)>>,
    time: i32,
) {
    let worker = workers.get_mut(&worker_index).unwrap();
    if worker.is_none() {
        for (item, deps) in &*deps {
            if deps.is_empty() {
                worker.replace((*item, time));
                break;
            }
        }
        if let Some((item, _)) = worker {
            deps.remove(item);
        }
    }
}

fn process_work_for_worker(
    deps: &mut HashMap<char, HashSet<&char>>,
    worker_index: usize,
    workers: &mut HashMap<usize, Option<(char, i32)>>,
    time: i32,
    sleep_time: i32,
    done: &mut HashSet<char>,
) {
    let worker = workers.get_mut(&worker_index).unwrap();
    if let Some((item, item_ts)) = worker {
        if is_done(time, *item, *item_ts, sleep_time) {
            let (item, _item_ts) = worker.take().unwrap();
            done.insert(item);
            for (_, deps) in deps.iter_mut() {
                deps.remove(&item);
            }
            deps.remove(&item);
        }
    }
}

fn is_done(time: i32, item: char, item_ts: i32, sleep_time: i32) -> bool {
    time >= (item_ts + *MAPPING.get(&item).unwrap() as i32 + sleep_time)
}

#[allow(dead_code)]
fn part1_v2(lines: &Vec<String>) -> String {
    let input = parse(lines);
    let mut edges = HashMap::new();
    for dep in &input.deps {
        edges.entry(dep.from).or_insert(vec![]).push(dep.to);
    }
    let mut output: Vec<char> = vec![];
    let mut open_nodes: HashSet<char> = input
        .deps
        .iter()
        .flat_map(|Dep { from, to }| vec![*from, *to])
        .collect();
    while !open_nodes.is_empty() {
        let mut open_nodes_sorted: Vec<char> = open_nodes.iter().copied().collect();
        open_nodes_sorted.sort_unstable();
        let current_node = open_nodes_sorted.into_iter().next().unwrap();
        let mut visiting: HashSet<char> = HashSet::new();
        visit(
            current_node,
            &mut visiting,
            &mut open_nodes,
            &mut output,
            &edges,
        );
    }
    output.into_iter().collect()
}

pub fn part1(lines: &Vec<String>) -> String {
    let input = parse(lines);
    let mut edges = HashMap::new();
    for dep in &input.deps {
        edges
            .entry(dep.from)
            .or_insert(HashSet::<char>::new())
            .insert(dep.to);
    }
    let mut output: Vec<char> = vec![];
    let mut remaining_nodes: HashSet<char> = input
        .deps
        .iter()
        .flat_map(|Dep { from, to }| vec![*from, *to])
        .collect();
    for node in &remaining_nodes {
        edges.entry(*node).or_insert(HashSet::<char>::new());
    }
    while !remaining_nodes.is_empty() {
        let mut candidates: Vec<char> = edges
            .iter()
            .filter(|(_k, v)| {
                let branches: HashSet<_> = v.intersection(&remaining_nodes).collect();
                branches.is_empty()
            })
            .map(|(k, _v)| *k)
            .collect();
        candidates.sort_unstable();
        let node = candidates[0];
        output.push(node);
        remaining_nodes.remove(&node);
        edges.remove(&node);
    }
    output.into_iter().collect()
}

fn visit(
    node: char,
    visiting: &mut HashSet<char>,
    open_nodes: &mut HashSet<char>,
    output: &mut Vec<char>,
    edges: &HashMap<char, Vec<char>>,
) {
    if !open_nodes.contains(&node) {
        return;
    }
    if visiting.contains(&node) {
        panic!(
            "loop found in input? node: {} visiting: {:?}",
            node, visiting
        );
    }

    visiting.insert(node);
    let mut node_s_edges: Vec<char> = edges
        .get(&node)
        .unwrap_or(&vec![])
        .iter().copied()
        .collect();
    node_s_edges.sort_unstable();
    for node in node_s_edges {
        visit(node, visiting, open_nodes, output, edges);
    }
    visiting.remove(&node);

    open_nodes.remove(&node);
    output.push(node);
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
struct Graph {
    deps: Vec<Dep>,
}

#[derive(Debug, Eq, PartialEq)]
struct Dep {
    from: char,
    to: char,
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(&utils::to_vec(&[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
        ])),
        Graph {
            deps: vec![
                Dep { to: 'C', from: 'A' },
                Dep { to: 'C', from: 'F' },
                Dep { to: 'A', from: 'B' },
            ]
        }
    );
}

fn parse(lines: &Vec<String>) -> Graph {
    Graph {
        deps: lines
            .iter()
            .map(|line| {
                let (to, from) = line
                    .split_once(" must be finished before step ")
                    .expect("line needs to be splitable");
                let to = to
                    .chars()
                    .nth(to.len() - 1)
                    .expect("line needs to be longer than 1 char");
                let from = from
                    .chars()
                    .next()
                    .expect("line needs to have at least one item");
                Dep { from, to }
            })
            .collect(),
    }
}
