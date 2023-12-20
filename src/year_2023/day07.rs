use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 6440);
    assert_eq!(part2(&input), 5905);
}

pub fn part1(input: &str) -> u32 {
    let input = input.split('\n').collect::<Vec<&str>>();
    let input = parse_input(&input, 11);
    let ranked: Vec<_> = input.into_iter().map(|(h, b)| (rank(&h), h, b)).collect();
    order_and_sum(ranked)
}

pub fn part2(input: &str) -> u32 {
    let input = input.split('\n').collect::<Vec<&str>>();
    let input = parse_input(&input, 1);
    let ranked: Vec<_> = input.into_iter().map(|(h, b)| (rank(&h), h, b)).collect();
    order_and_sum(ranked)
}

fn parse_input(input: &[&str], jvalue: u32) -> Vec<(Vec<u32>, u32)> {
    input
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = hand
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => jvalue,
                    'T' => 10,
                    n => n.to_digit(10).unwrap(),
                })
                .collect();
            let bid: u32 = bid.parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn rank(hand: &[u32]) -> u32 {
    let mut m = HashMap::new();
    for v in hand {
        m.entry(v).and_modify(|v| *v += 1).or_insert(1);
    }

    if m.len() == 1 {
        return 7;
    }
    if m.len() == 5 {
        if let Some(1) = m.get(&1) {
            return 2;
        }
        return 1;
    }
    if m.len() == 4 {
        if m.get(&1).is_some() {
            return 4;
        }
        return 2;
    }
    if m.len() == 3 {
        for (_k, v) in m.iter() {
            if *v == 3 {
                if m.get(&1).is_some() {
                    return 6;
                }
                return 4;
            }
        }
        if let Some(1) = m.get(&1) {
            return 5;
        }
        if let Some(2) = m.get(&1) {
            return 6;
        }
        return 3;
    }
    // len == 2
    if m.get(&1).is_some() {
        return 7;
    }
    for (_k, v) in m {
        if v == 4 {
            return 6;
        }
    }
    5
}

fn order_and_sum(mut ranked: Vec<(u32, Vec<u32>, u32)>) -> u32 {
    ranked.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            let [a1, a2, a3, a4, a5] = a.1[..] else {
                unreachable!()
            };
            let [b1, b2, b3, b4, b5] = b.1[..] else {
                unreachable!()
            };
            (a1, a2, a3, a4, a5).cmp(&(b1, b2, b3, b4, b5))
        }
    });
    ranked
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, b))| b * (i as u32 + 1))
        .sum()
}
