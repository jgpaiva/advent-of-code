use std::collections::HashSet;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 13);
    assert_eq!(part2(&input), 30);
}

pub fn part1(input: &str) -> i32 {
    let input = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    input
        .iter()
        .map(|s| s.split_once(':').unwrap().1)
        .map(|s| s.split_once('|').unwrap())
        .map(|(winning, all)| (parse_numbers(winning), parse_numbers(all)))
        .map(|(winning, all)| winning.intersection(&all).count())
        .filter(|winner| *winner > 0)
        .map(|winner| 2_i32.pow(winner as u32 - 1))
        .sum::<i32>()
}

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split(' ').flat_map(|s| s.parse::<u32>()).collect()
}

pub fn part2(input: &str) -> usize {
    let input = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let winners: Vec<usize> = input
        .iter()
        .map(|s| s.split_once(':').unwrap().1)
        .map(|s| s.split_once('|').unwrap())
        .map(|(winning, all)| (parse_numbers(winning), parse_numbers(all)))
        .map(|(winning, all)| winning.intersection(&all).count())
        .collect();
    let mut generated = vec![0; winners.len()];
    for i in (0..winners.len()).rev() {
        let winner = winners[i];
        let mut gen = 0;
        for j in 0..winner {
            gen += winners[i + j + 1];
            gen += generated[i + j + 1];
        }
        generated[i] = gen;
    }
    winners.len() + winners.iter().sum::<usize>() + generated.iter().sum::<usize>()
}
