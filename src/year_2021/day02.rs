#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2021/test_day02");
    assert_eq!(part2(&input), 900);
    assert_eq!(part1(&input), 150);
}

pub fn part2(lines: &[String]) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(c, v)| match c {
            "forward" => Day5Commands::Forward(v.parse().unwrap()),
            "down" => Day5Commands::Down(v.parse().unwrap()),
            "up" => Day5Commands::Up(v.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for i in lines {
        match i {
            Day5Commands::Forward(v) => {
                horizontal += v;
                vertical += aim * v
            }
            Day5Commands::Down(v) => aim += v,
            Day5Commands::Up(v) => aim -= v,
        }
    }
    horizontal * vertical
}

enum Day5Commands {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn part1(lines: &[String]) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(c, v)| match c {
            "forward" => Day5Commands::Forward(v.parse().unwrap()),
            "down" => Day5Commands::Down(v.parse().unwrap()),
            "up" => Day5Commands::Up(v.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut horizontal = 0;
    let mut vertical = 0;
    for i in lines {
        match i {
            Day5Commands::Forward(v) => horizontal += v,
            Day5Commands::Down(v) => vertical += v,
            Day5Commands::Up(v) => vertical -= v,
        }
    }
    horizontal * vertical
}
