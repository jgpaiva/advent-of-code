#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2018/test_day12");
    assert_eq!(part1(input, 20), 325);
}

#[allow(dead_code)]
pub fn part2(input: String, days: i32) -> i32 {
    part1(input, days)
}

pub fn part1(input: String, days: i32) -> i32 {
    let input: Vec<&str> = input.split_terminator('\n').collect();
    let mut values: Vec<bool> = input[0]
        .split_once(": ")
        .unwrap()
        .1
        .chars()
        .map(|x| x == '#')
        .collect();
    let patterns: Vec<(Vec<bool>, bool)> = input
        .into_iter()
        .skip(2)
        .map(|line| {
            let (pattern, res) = line.split_once(" => ").unwrap();
            let result = res.starts_with('#');
            let pattern: Vec<bool> = pattern.chars().map(|x| x == '#').collect();
            assert_eq!(pattern.len(), 5);
            (pattern, result)
        })
        .collect();
    let mut start: i32 = -5;
    values = (0..5)
        .map(|_| false)
        .chain(values)
        .chain((0..5).map(|_| false))
        .collect();
    for _ in 0..days {
        let previous = values.clone();
        for i in 2..previous.len() - 2 {
            let current = &previous[i - 2..=i + 2];
            let (_pattern, res) = patterns
                .iter()
                .find(|(p, _r)| p.iter().zip(current.iter()).all(|(x1, x2)| *x1 == *x2))
                .unwrap_or_else(|| panic!("should work for {:?}", current));
            values[i] = *res;
        }
        if values[0..5].iter().any(|x| *x) {
            values = (0..5).map(|_| false).chain(values.into_iter()).collect();
            start -= 5;
        }
        if values[values.len() - 5..values.len()].iter().any(|x| *x) {
            values = values.into_iter().chain((0..5).map(|_| false)).collect();
        }
    }
    values
        .iter()
        .enumerate()
        .map(|(i, x)| if *x { i as i32 + start } else { 0 })
        .sum()
}
