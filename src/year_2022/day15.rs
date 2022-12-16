#[cfg(test)]
use crate::utils;

use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(calculate_line(&input, 10), 26);
    assert_eq!(part2(&input), 93);
}

pub fn part2(lines: &str) -> i32 {
    parse_input(lines).unwrap();
    0
}

pub fn part1(lines: &str) -> i32 {
    let line = 2000000;
    calculate_line(lines, line)
}

fn calculate_line(lines: &str, line: i32) -> i32 {
    let input = parse_input(lines).unwrap();
    let mut affected_ranges = vec![];
    for (sensor, beacon) in input.iter() {
        let d_beacon = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        // can affect the line if distance from line to sensor is <= than sensor to beacon
        let d_line = (sensor.1 - line).abs();
        if d_line <= d_beacon {
            affected_ranges.push((
                sensor.0 - (d_beacon - d_line),
                sensor.0 + (d_beacon - d_line),
            ));
        }
    }
    affected_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let affected_ranges =
        affected_ranges
            .into_iter()
            .fold(Vec::<(i32, i32)>::new(), |mut acc, v| {
                if let Some(current) = acc.pop() {
                    if v.0 <= current.1 + 1 {
                        acc.push((current.0, v.1.max(current.1)));
                        acc
                    } else {
                        acc.push(v);
                        acc
                    }
                } else {
                    acc.push(v);
                    acc
                }
            });

    let mut beacons_in_line: Vec<_> = input
        .iter()
        .map(|(_, b)| *b)
        .filter(|b| b.1 == line)
        .collect();
    beacons_in_line.sort_by(|a, b| a.1.cmp(&b.1));
    affected_ranges
        .into_iter()
        .map(|r| {
            let b = beacons_in_line
                .iter()
                .filter(|b| b.1 >= r.0 || b.1 <= r.1)
                .unique_by(|b| (b.0, b.1))
                .count();
            r.1 - r.0 + 1 - b as i32
        })
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct Sensor(i32, i32);
#[derive(Debug, Copy, Clone)]
struct Beacon(i32, i32);

fn parse_input(lines: &str) -> anyhow::Result<Vec<(Sensor, Beacon)>> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let captures_iter = re.captures_iter(lines);
    captures_iter
        .into_iter()
        .map(|line_captures| {
            assert_eq!(line_captures.len(), 5);
            let items = [
                &line_captures[1],
                &line_captures[2],
                &line_captures[3],
                &line_captures[4],
            ]
            .iter()
            .map(|c| c.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .context(format!("couldn't parse a number: {line_captures:?}"))?;
            assert_eq!(items.len(), 4);
            let sensor = Sensor(items[0], items[1]);
            let beacon = Beacon(items[2], items[3]);
            Ok((sensor, beacon))
        })
        .collect()
}
