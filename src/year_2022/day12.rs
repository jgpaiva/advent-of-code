use anyhow::anyhow;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 31);
    assert_eq!(part2(&input), 29);
    let input = "\
    aabcdefghijklmnopqrstuvwxyzza\n\
    SabcdefghijklmnopqrstuvwxyzEa\n\
    aabcdefghijklmnopqrstuvwxyzza";
    assert_eq!(part1(input), 27);
    // test elevations for start and end
    let input = "\
    abcdefghijklmnopqrstuvwxyza\n\
    SbcdefghijklmnopqrstuvwxyEa\n\
    abcdefghijklmnopqrstuvwxyza";
    assert_eq!(part1(input), 25);
    let input = "\
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\
    SabcdefghijklmnoxpqrstuvwxyzEa\n\
    aaaaaaaaaaaaaaaoooaaaaaaaaaaaa";
    assert_eq!(part1(input), 30);
    let input = "\
    aaaaaaaaaiiiaaaaoooaaaaaaaaaaaa\n\
    aaaaaaaaaiaiaaaaoooaaaaaaaaaaaa\n\
    SabcdefghiajklmnoapqrstuvwxyzEa\n\
    aaaaaaaaaiiiaaaaoaoaaaaaaaaaaaa\n\
    aaaaaaaaaaaaaaaaoooaaaaaaaaaaaa\n";
    assert_eq!(part1(input), 33);
}

pub fn part2(lines: &str) -> u64 {
    let input: Map = lines.parse().unwrap();
    run_bfs(
        &input,
        input.end,
        |_, p_height, _, n_height| p_height <= n_height + 1,
        |n| input.at(n) == 0,
    )
}

pub fn part1(lines: &str) -> u64 {
    let input: Map = lines.parse().unwrap();
    run_bfs(
        &input,
        input.start,
        |_, p_height, _, n_height| n_height <= p_height + 1,
        |n| n == input.end,
    )
}

fn run_bfs<F1, F2>(
    input: &Map,
    start_node: (usize, usize),
    filter_condition: F2,
    end_condition: F1,
) -> u64
where
    F1: Fn((usize, usize)) -> bool,
    F2: Fn((usize, usize), u8, (usize, usize), u8) -> bool,
{
    let mut open_nodes = VecDeque::new();
    open_nodes.push_back((start_node, 0));
    let mut visited_nodes = HashSet::new();
    visited_nodes.insert(start_node);
    loop {
        if open_nodes.is_empty() {
            panic!("visited all paths and didn't find end node");
        }
        let (p, cost) = open_nodes.pop_front().unwrap();
        let p_height = input.at(p);
        let neighbors = input.neighbors(p);
        let filtered_neighbors = neighbors
            .into_iter()
            .filter(|(neighbour, n_height)| filter_condition(p, p_height, *neighbour, *n_height));
        for (n, _) in filtered_neighbors {
            if end_condition(n) {
                return cost + 1;
            }
            if visited_nodes.insert(n) {
                open_nodes.push_back((n, cost + 1));
            }
        }
    }
}

struct Map {
    m: Vec<u8>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn at(&self, (x, y): (usize, usize)) -> u8 {
        self.m[x + y * self.width]
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<((usize, usize), u8)> {
        [
            (x as i32 - 1, y as i32),
            (x as i32 + 1, y as i32),
            (x as i32, y as i32 - 1),
            (x as i32, y as i32 + 1),
        ]
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < self.width as i32 && *y < self.height as i32)
        .map(|(x, y)| {
            (
                (*x).try_into().expect("checked bounds on previous step"),
                (*y).try_into().expect("checked bounds on previous step"),
            )
        })
        .map(|pos| (pos, self.at(pos)))
        .collect()
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split_whitespace().collect();
        if input.is_empty() {
            return Err(anyhow!("map needs to have at least one line"));
        }
        let width = input[0].len();
        let height = input.len();
        let mut m = vec![0; width * height];
        let mut start = None;
        let mut end = None;
        for (y, line) in input.into_iter().enumerate() {
            for (x, item) in line.as_bytes().iter().enumerate() {
                let pos = x + y * width;
                if *item == b'S' {
                    let old_start = start.replace((x, y));
                    if old_start.is_some() {
                        return Err(anyhow!("Found two starts. At {old_start:?} and {start:?}"));
                    }
                    m[pos] = 0;
                } else if *item == b'E' {
                    let old_end = end.replace((x, y));
                    if old_end.is_some() {
                        return Err(anyhow!("Found two ends. At {old_end:?} and {end:?}"));
                    }
                    m[pos] = b'z' - b'a';
                } else {
                    m[pos] = *item - b'a';
                }
            }
        }
        if let (Some(start), Some(end)) = (start, end) {
            Ok(Self {
                m,
                width,
                height,
                start,
                end,
            })
        } else {
            Err(anyhow!(
                "Couldn't find start or end. Start: {start:?} End: {end:?}"
            ))
        }
    }
}
