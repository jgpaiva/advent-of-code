use std::collections::{HashMap, HashSet};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 143);
    assert_eq!(part2(&input), 123);
}

pub fn part1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);

    let mut ret_value = 0;
    'outer: for update in updates {
        let u: HashMap<usize, usize> = update
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        for (before, after) in &rules {
            let Some(before) = u.get(before) else {
                continue; // before doesn't exist
            };
            let Some(after) = u.get(after) else {
                continue; // after doesn't exist
            };
            if before >= after {
                continue 'outer; // rule failed
            }
        }
        // all rules passed
        let index = update.len() / 2;
        ret_value += update[index] as i32;
    }
    ret_value
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = rules
        .split("\n")
        .map(|line| line.split_once("|").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let updates: Vec<Vec<usize>> = updates
        .split("\n")
        .map(|line| line.split(",").map(|i| i.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn try_recover_update(mut update: Vec<usize>, rules: &[(usize, usize)]) -> i32 {
    let update_contents: HashSet<_> = update.iter().cloned().collect();
    let mut rules: Vec<_> = rules
        .iter()
        .cloned()
        .filter(|(v1, v2)| update_contents.contains(v1) && update_contents.contains(v2))
        .collect();
    let mut final_update = Vec::new();
    'outer: loop {
        if rules.is_empty() {
            break;
        }
        'inner: for i in (0..update.len()).rev() {
            let v = update[i];
            for (l, _) in rules.iter() {
                if *l == v {
                    continue 'inner;
                }
            }
            // has no dependencies, is last
            final_update.push(v);
            update.remove(i);

            // remove rules refencing this value
            rules = rules.iter().cloned().filter(|(_, r)| *r != v).collect();
            continue 'outer;
        }
        // no progress, can't fix this one?
        return 0;
    }
    // final_update is actually reversed, but that doesn't matter since we're taking the mid point
    let index = final_update.len() / 2;
    final_update[index] as i32
}

pub fn part2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);

    let mut ret_value = 0;
    'outer: for update in updates {
        let u: HashMap<usize, usize> = update
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        for (before, after) in &rules {
            let Some(before) = u.get(before) else {
                continue; // before doesn't exist
            };
            let Some(after) = u.get(after) else {
                continue; // after doesn't exist
            };
            if before >= after {
                // rule failed
                ret_value += try_recover_update(update, &rules);
                continue 'outer;
            }
        }
        // all rules passed
    }
    ret_value
}
