use std::{error::Error, str::FromStr};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 13140);
    assert_eq!(
        part2(&input),
        r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
    );
}

pub fn part2(lines: &str) -> String {
    let commands = parse_input(lines);
    let mut x = 1;
    let mut cycle = 0;
    let mut res = vec!['\n'];
    for c in commands {
        let cycles = match c {
            Command::Noop => 1,
            Command::Add(_) => 2,
        };
        for _ in 0..cycles {
            if cycle % 40 == x - 1 || cycle % 40 == x || cycle % 40 == x + 1 {
                res.push('#');
            } else {
                res.push('.');
            }
            cycle += 1;
            if cycle % 40 == 0 {
                res.push('\n');
            }
        }
        match c {
            Command::Noop => (),
            Command::Add(v) => x += v,
        }
    }
    res.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

enum Command {
    Noop,
    Add(i32),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Self::Noop)
        } else if s.starts_with("addx ") {
            if let Some((_, value)) = s.split_once(' ') {
                Ok(Self::Add(value.parse()?))
            } else {
                Err(format!("couldn't split {s}").into())
            }
        } else {
            Err(format!("unknown command: {s}").into())
        }
    }
}

pub fn part1(lines: &str) -> i32 {
    let commands = parse_input(lines);
    let mut x = 1;
    let mut cycle = 0;
    let values = [20, 60, 100, 140, 180, 220];
    let mut res = vec![];
    for c in commands {
        let cycles = match c {
            Command::Noop => 1,
            Command::Add(_) => 2,
        };
        for _ in 0..cycles {
            cycle += 1;
            if values.contains(&cycle) {
                res.push(x * cycle);
            }
        }
        match c {
            Command::Noop => (),
            Command::Add(v) => x += v,
        }
    }
    res.into_iter().sum()
}

fn parse_input(lines: &str) -> Vec<Command> {
    let commands: Vec<Command> = lines
        .split_terminator('\n')
        .map(|line| {
            line.parse()
                .unwrap_or_else(|e| panic!("should have parsed: {line}\n original err: {e}"))
        })
        .collect();
    commands
}
