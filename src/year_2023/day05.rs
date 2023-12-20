use core::ops::RangeInclusive;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 35);
    assert_eq!(part2(&input), 46);
}

pub fn part1(input: &str) -> i64 {
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .flat_map(|v| v.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let input = &lines[1..];
    let input = parse_input(input);
    seeds
        .iter()
        .map(|seed| {
            input.iter().fold(*seed, |seed, mappings| {
                let def = (seed, seed..=seed + 1);
                let (new_seed, r) = mappings
                    .iter()
                    .find(|(_destination, r)| r.contains(&seed))
                    .unwrap_or(&def);
                new_seed + (seed - r.start())
            })
        })
        .min()
        .unwrap()
}

fn parse_input(input: &[&str]) -> Vec<Vec<(i64, RangeInclusive<i64>)>> {
    input
        .iter()
        .map(|s| {
            let mut v = s
                .split('\n')
                .skip(1)
                .map(|line| {
                    let [destination, start, cnt] = line
                        .split(' ')
                        .map(|v| v.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()[..]
                    else {
                        unreachable!()
                    };
                    (destination, start..=start + cnt - 1)
                })
                .collect::<Vec<_>>();
            v.sort_by(|a, b| a.1.start().partial_cmp(b.1.start()).unwrap());
            v
        })
        .collect::<Vec<_>>()
}

pub fn part2(input: &str) -> i64 {
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .flat_map(|v| v.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let input = &lines[1..];
    let input = parse_input(input);
    seeds
        .chunks(2)
        .map(|i| {
            let [start, size] = i[..] else { unreachable!() };
            min_overlaps(start..=start + size - 1, &input)
        })
        .min()
        .unwrap()
}
fn min_overlaps(mut v: RangeInclusive<i64>, input: &[Vec<(i64, RangeInclusive<i64>)>]) -> i64 {
    if v.end() < v.start() {
        return i64::MAX;
    }
    if input.is_empty() {
        return *v.start();
    }
    let overlaps = input[0]
        .iter()
        .filter(|i| {
            let r = &i.1;
            r.start() <= v.end() && r.end() >= v.start()
        })
        .collect::<Vec<_>>();
    if overlaps.is_empty() {
        return min_overlaps(v, &input[1..]);
    }
    let mut min = i64::MAX;
    for i in overlaps {
        if i.1.start() > v.start() {
            min = min.min(min_overlaps(*v.start()..=i.1.start() - 1, &input[1..]));
        }
        let start = i.0 + 0.max(v.start() - i.1.start());
        let size = (0).max(v.end().min(i.1.end()) - v.start().max(i.1.start()));
        v = *i.1.end() + 1..=*v.end();
        min = min.min(min_overlaps(start..=start + size - 1, &input[1..]));
    }

    min.min(min_overlaps(v, &input[1..]))
}
