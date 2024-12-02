use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 11);
    assert_eq!(part2(&input), 31);
}

pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let input: Vec<_> = input.split('\n').collect();
    let lines: Vec<(i32, i32)> = input
        .into_iter()
        .map(|line| {
            line.split(' ')
                .filter(|e| !e.is_empty())
                .collect::<Vec<_>>()
        })
        .map(|line| (line[0].parse().unwrap(), line[1].parse().unwrap()))
        .collect();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (l, r) in lines {
        left.push(l);
        right.push(r);
    }
    (left, right)
}

pub fn part2(input: &str) -> i32 {
    let (l_input, r_input) = parse_input(input);
    let mut r_map = HashMap::new();
    for i in r_input {
        let entry = r_map.entry(i).or_default();
        *entry += 1;
    }
    l_input
        .into_iter()
        .map(|e| r_map.get(&e).unwrap_or(&0) * e)
        .sum()
}
