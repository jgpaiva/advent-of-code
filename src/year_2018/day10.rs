use std::io::Write;
use std::{
    collections::HashSet,
    io::{self, Read},
};

use regex::Regex;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2018/test_day10");
    assert_eq!(
        part1(input.clone(), 3),
        r#"█···█··███
█···█···█·
█···█···█·
█████···█·
█···█···█·
█···█···█·
█···█···█·
█···█··███
"#
    );
    //assert_eq!(part2(input.clone()), 12);
}

#[allow(dead_code)]
pub fn part1_iter(lines: String) {
    let mut counter = 0;
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    loop {
        counter += 1;
        let v = part1(lines.clone(), counter);
        println!("iteration {}", counter);
        println!("{}", v);
        if v.contains("sorry") {
            counter += 100;
            continue;
        }
        println!("< enter to continue >");
        stdout.flush().unwrap();
        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}

pub fn part1(lines: String, iter: i32) -> String {
    let mut input = parse(lines);
    for _ in 0..iter {
        input = input
            .into_iter()
            .map(|(p, v)| ((p.0 + v.0, p.1 + v.1), v))
            .collect()
    }
    print_current(input)
}

#[test]
fn test_display() {
    let input = utils::read_file("2018/test_day10");
    let parsed = parse(input);
    let current = print_current(parsed);
    assert_eq!(
        current,
        r#"········█·············
················█·····
·········█·█··█·······
······················
█··········█·█·······█
···············█······
····█·················
··█·█····█············
·······█··············
······█···············
···█···█·█···█········
····█··█··█·········█·
·······█··············
···········█··█·······
█···········█·········
···█·······█··········
"#
    );
}

fn parse(input: String) -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r".*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+).*").unwrap();
    let points: Vec<Vec<i32>> = re
        .captures_iter(input.as_str())
        .map(|line| {
            line.iter()
                .skip(1)
                .take(4)
                .map(|c| c.unwrap().as_str().parse().unwrap())
                .collect()
        })
        .collect();
    points
        .iter()
        .map(|cap| ((cap[0], cap[1]), (cap[2], cap[3])))
        .collect::<Vec<_>>()
}

const MAX_WINDOW_SIZE: u64 = 200 * 200;

fn print_current(input: Vec<((i32, i32), (i32, i32))>) -> String {
    let current: HashSet<_> = input.iter().map(|(p, _v)| p).collect();
    let min_x = current.iter().map(|(x, _y)| x).min().unwrap().to_owned();
    let max_x = current.iter().map(|(x, _y)| x).max().unwrap().to_owned();
    let min_y = current.iter().map(|(_x, y)| y).min().unwrap().to_owned();
    let max_y = current.iter().map(|(_x, y)| y).max().unwrap().to_owned();

    if (max_x - min_x) as u64 > MAX_WINDOW_SIZE
        || (max_y - min_y) as u64 > MAX_WINDOW_SIZE
        || (max_x - min_x) as u64 * (max_y - min_y) as u64 > MAX_WINDOW_SIZE
    {
        return format!(
            "sorry, over window size: {} X {}",
            max_x - min_x,
            max_y - min_y
        );
    }

    (min_y..=max_y)
        .map(|y| {
            let line: String = (min_x..=max_x)
                .map(|x| {
                    if current.contains(&(x, y)) {
                        "█"
                    } else {
                        "·"
                    }
                })
                .collect();
            line + "\n"
        })
        .collect()
}
