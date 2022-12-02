use std::str::FromStr;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 15);
    assert_eq!(part2(&input), 12);
}

pub fn part2(lines: &str) -> i32 {
    let lines = lines
        .split_terminator('\n')
        .map(|x| x.split_once(' ').unwrap())
        .map(|(x, y)| {
            (
                x.parse::<RPS>().unwrap(),
                match y.as_bytes()[0] as char {
                    'X' => -1,
                    'Y' => 0,
                    'Z' => 1,
                    _ => unreachable!(),
                },
            )
        });

    lines
        .into_iter()
        .map(|(other, result)| {
            let my_play = match other.play_to(-result) {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scisors => 3,
            };
            let result = match result {
                1 => 6,
                0 => 3,
                -1 => 0,
                _ => unreachable!(),
            };
            my_play + result
        })
        .sum()
}

#[derive(PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scisors,
}

impl FromStr for RPS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = s.as_bytes()[0] as char;
        match char {
            'A' => Ok(RPS::Rock),
            'B' => Ok(RPS::Paper),
            'C' => Ok(RPS::Scisors),
            'X' => Ok(RPS::Rock),
            'Y' => Ok(RPS::Paper),
            'Z' => Ok(RPS::Scisors),
            _ => Err("Couldn't parse".into()),
        }
    }
}

impl RPS {
    /// returns zero is tie, 1 if self wins
    fn winner(&self, other: &RPS) -> i32 {
        match (self, other) {
            (RPS::Rock, RPS::Scisors) => 1,
            (RPS::Paper, RPS::Rock) => 1,
            (RPS::Scisors, RPS::Paper) => 1,
            (a, b) if a == b => 0,
            _ => -1,
        }
    }

    fn play_to(self, final_result: i32) -> RPS {
        match (self, final_result) {
            (RPS::Rock, 1) => RPS::Scisors,
            (RPS::Paper, 1) => RPS::Rock,
            (RPS::Scisors, 1) => RPS::Paper,
            (a, 0) => a,
            (RPS::Rock, -1) => RPS::Paper,
            (RPS::Paper, -1) => RPS::Scisors,
            (RPS::Scisors, -1) => RPS::Rock,
            _ => unreachable!(),
        }
    }
}

pub fn part1(lines: &str) -> i32 {
    let lines = lines
        .split_terminator('\n')
        .map(|x| x.split_once(' ').unwrap())
        .map(|(x, y)| (x.parse::<RPS>().unwrap(), y.parse::<RPS>().unwrap()));

    lines
        .into_iter()
        .map(|(other, me)| {
            let winner_value = match me.winner(&other) {
                1 => 6,
                0 => 3,
                -1 => 0,
                _ => unreachable!(),
            };
            let play_value = match me {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scisors => 3,
            };
            winner_value + play_value
        })
        .sum()
}
