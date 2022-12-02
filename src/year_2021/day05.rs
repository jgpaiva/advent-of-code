use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(input.clone()), 5);
    assert_eq!(part2(input), 12);
}

pub fn part2(lines: String) -> usize {
    calculate(lines, true)
}

pub fn part1(lines: String) -> usize {
    calculate(lines, false)
}

fn calculate(lines: String, include_diagonal: bool) -> usize {
    let input = parse(lines, include_diagonal);
    let mut taken_points: HashMap<(i32, i32), i32> = HashMap::new();
    for (from, to) in input {
        if from.0 == to.0 {
            let (from, to) = sort_by_y(from, to);
            for y in from.1..=to.1 {
                record_point((from.0, y), &mut taken_points);
            }
        } else if from.1 == to.1 {
            let (from, to) = sort_by_x(from, to);
            for x in from.0..=to.0 {
                record_point((x, from.1), &mut taken_points);
            }
        } else if (from.0 >= to.0 && from.1 >= to.1) || (to.0 >= from.0 && to.1 >= from.1) {
            let (from, to) = sort_by_x(from, to);
            for x in from.0..=to.0 {
                record_point((x, from.1 + x - from.0), &mut taken_points);
            }
        } else {
            let (from, to) = sort_by_x(from, to);
            for x in from.0..=to.0 {
                record_point((x, from.1 - x + from.0), &mut taken_points);
            }
        }
    }
    taken_points.values().filter(|x| **x > 1).count()
}

fn record_point(point: (i32, i32), taken_points: &mut HashMap<(i32, i32), i32>) {
    let entry = taken_points.entry(point).or_insert(0);
    *entry += 1;
}

fn sort_by_y(from: (i32, i32), to: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    if from.1 <= to.1 {
        (from, to)
    } else {
        (to, from)
    }
}

fn sort_by_x(from: (i32, i32), to: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    if from.0 <= to.0 {
        (from, to)
    } else {
        (to, from)
    }
}

fn parse(lines: String, include_diagonal: bool) -> Vec<((i32, i32), (i32, i32))> {
    let mut input: Vec<((i32, i32), (i32, i32))> = lines
        .split_terminator('\n')
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(from, to)| (from.split_once(',').unwrap(), to.split_once(',').unwrap()))
        .map(|((fx, fy), (tx, ty))| {
            (
                (fx.parse().unwrap(), fy.parse().unwrap()),
                (tx.parse().unwrap(), ty.parse().unwrap()),
            )
        })
        .collect();
    if !(include_diagonal) {
        input.retain(|(from, to)| from.0 == to.0 || from.1 == to.1);
    }
    input
}
