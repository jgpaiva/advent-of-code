use std::collections::{HashMap, LinkedList};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day14");
    assert_eq!(part1(input.clone()), 1588);
    assert_eq!(part2(input.clone()), 2188189693529);
}

pub fn part2(input: String) -> u64 {
    let iterations = 40;
    todo!();
    calculate(input, iterations)
}

pub fn part1(input: String) -> u64 {
    let iterations = 10;
    calculate(input, iterations)
}

fn calculate(input: String, iterations: i32) -> u64 {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<(char, char), char> = rules
        .split_terminator('\n')
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(part1, part2)| {
            (
                (part1.chars().nth(0).unwrap(), part1.chars().nth(1).unwrap()),
                part2.chars().nth(0).unwrap(),
            )
        })
        .collect();
    let mut template: LinkedList<char> = template.chars().collect();
    for _ in 0..iterations {
        let start_len = template.len();
        dbg!(start_len);
        for _ in 0..start_len - 1 {
            let current = template.pop_front().unwrap();
            let next = template.front().cloned().unwrap();
            template.push_back(current);
            if let Some(to_add) = rules.get(&(current, next)).cloned() {
                template.push_back(to_add);
            }
        }
        // rotate 1 because we don't end at `start_len`
        let el = template.pop_front().unwrap();
        template.push_back(el);
    }
    let mut counter: HashMap<char, u64> = HashMap::new();
    for c in template {
        *counter.entry(c).or_insert(0) += 1;
    }
    let min = counter.iter().min_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap();
    let max = counter.iter().max_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap();
    max.1 - min.1
}
