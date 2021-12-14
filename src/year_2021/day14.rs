use std::collections::{HashMap, LinkedList};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day14");
    assert_eq!(part1(input.clone()), 1588);
    assert_eq!(part2(input.clone(), 1), 1);
    assert_eq!(part2(input.clone(), 10), 1588);
    let l1_mapping = HashMap::from([(('N', 'N'), 'N')]);
    let mut memoization = HashMap::new();
    assert_eq!(
        calculate_efficiently('N', 'N', 1, &l1_mapping, &mut memoization),
        HashMap::from([('N', 1)])
    );
    let mut memoization = HashMap::new();
    assert_eq!(
        calculate_efficiently('N', 'N', 2, &l1_mapping, &mut memoization),
        HashMap::from([('N', 3)])
    );
    assert_eq!(part2(input.clone(), 40), 2188189693529);
}

pub fn part2(input: String, iterations: u8) -> u64 {
    let (rules, template) = parse(input);
    let mut memoization = HashMap::new();

    let results: Vec<HashMap<_, _>> = template
        .clone()
        .into_iter()
        .zip(template.clone().into_iter().skip(1))
        .map(|(c1, c2)| calculate_efficiently(c1, c2, iterations, &rules, &mut memoization))
        .collect();
    let mut counter = HashMap::new();
    for result in results {
        for (c, count) in result {
            *counter.entry(c).or_insert(0) += count;
        }
    }
    for c in template {
        *counter.entry(c).or_insert(0) += 1;
    }
    let min = counter.iter().min_by(|c1, c2| c1.1.cmp(c2.1)).unwrap();
    let max = counter.iter().max_by(|c1, c2| c1.1.cmp(c2.1)).unwrap();
    max.1 - min.1
}

fn calculate_efficiently(
    c1: char,
    c2: char,
    level: u8,
    l1_mapping: &HashMap<(char, char), char>,
    memoization: &mut HashMap<(char, char, u8), HashMap<char, u64>>,
) -> HashMap<char, u64> {
    if level == 1 {
        if let Some(val) = l1_mapping.get(&(c1, c2)) {
            return HashMap::from([(*val, 1)]);
        }
        return HashMap::new();
    }
    if let Some(mapping) = memoization.get(&(c1, c2, level)) {
        return mapping.clone();
    }
    if let Some(val) = l1_mapping.get(&(c1, c2)).cloned() {
        let mut left_result = calculate_efficiently(c1, val, level - 1, l1_mapping, memoization);
        let right_result = calculate_efficiently(val, c2, level - 1, l1_mapping, memoization);
        for (c, count) in right_result {
            *left_result.entry(c).or_insert(0) += count;
        }
        *left_result.entry(val).or_insert(0) += 1;
        memoization.insert((c1, c2, level), left_result.clone());
        return left_result;
    }
    HashMap::new()
}

pub fn part1(input: String) -> u64 {
    let iterations = 10;
    calculate(input, iterations)
}

fn calculate(input: String, iterations: i32) -> u64 {
    let (rules, mut template) = parse(input);
    for _ in 0..iterations {
        let start_len = template.len();
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
    let min = counter.iter().min_by(|c1, c2| c1.1.cmp(c2.1)).unwrap();
    let max = counter.iter().max_by(|c1, c2| c1.1.cmp(c2.1)).unwrap();
    max.1 - min.1
}

fn parse(input: String) -> (HashMap<(char, char), char>, LinkedList<char>) {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<(char, char), char> = rules
        .split_terminator('\n')
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(part1, part2)| {
            (
                (part1.chars().next().unwrap(), part1.chars().nth(1).unwrap()),
                part2.chars().next().unwrap(),
            )
        })
        .collect();
    (rules, template.chars().collect())
}
