#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 288);
    assert_eq!(part2(&input), 71503);
}

pub fn part1(input: &str) -> i64 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let input = parse_input(&lines);
    let mut total_wins = 1;
    for (t, d) in input {
        let mut wins = 0;
        for i in 1..t {
            if (t - i) * i > d {
                wins += 1;
            }
        }
        total_wins *= wins;
    }
    total_wins
}

fn parse_input(input: &[&str]) -> Vec<(u64, u64)> {
    let parse_line = |line: &str| {
        line.split_once(':')
            .unwrap()
            .1
            .split(' ')
            .flat_map(|i| i.parse::<u64>().ok())
            .collect::<Vec<_>>()
    };

    let t = parse_line(input[0]);
    let d = parse_line(input[1]);
    t.into_iter().zip(d).collect::<Vec<_>>()
}

pub fn part2(input: &str) -> i64 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let lines = lines
        .into_iter()
        .map(|l| l.replace(' ', ""))
        .collect::<Vec<String>>();
    let lines = lines.iter().map(|l| l.as_str()).collect::<Vec<&str>>();
    let input = parse_input(&lines);
    let mut total_wins = 1;
    for (t, d) in input {
        let mut wins = 0;
        for i in 1..t {
            if (t - i) * i > d {
                wins += 1;
            }
        }
        total_wins *= wins;
    }
    total_wins
}
