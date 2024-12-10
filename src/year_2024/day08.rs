use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 14);
    assert_eq!(part2(&input), 34);
}

pub fn part1(input: &str) -> usize {
    let (antennae, input) = parse_input(input);
    let mut antinodes = HashSet::new();
    for antennae in antennae.values() {
        let an = get_antinodes(antennae);
        antinodes.extend(an);
    }
    antinodes
        .into_iter()
        .filter(|(line, column)| {
            *line >= 0
                && *line < input.len() as i32
                && *column >= 0
                && *column < input[0].len() as i32
        })
        .count()
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (HashMap<u8, Vec<(usize, usize)>>, Vec<&str>) {
    let mut antennae: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let input: Vec<_> = input.split("\n").collect();
    for (line_i, line) in input.iter().enumerate() {
        for (column_i, &column) in line.as_bytes().iter().enumerate() {
            if column != b'.' {
                let entry = antennae.entry(column).or_default();
                entry.push((line_i, column_i));
            }
        }
    }
    (antennae, input)
}

#[test]
fn test_get_antinodes() {
    let test = |v1: HashSet<(i32, i32)>, v2: &[(i32, i32)]| {
        let v2: HashSet<_> = v2.iter().cloned().collect();
        assert_eq!(v1, v2);
    };
    test(get_antinodes(&[(1, 1), (2, 2)]), &[(0, 0), (3, 3)]);
    test(get_antinodes(&[(2, 2), (1, 1)]), &[(0, 0), (3, 3)]);
    test(get_antinodes(&[(1, 2), (2, 1)]), &[(0, 3), (3, 0)]);
    test(get_antinodes(&[(2, 1), (1, 2)]), &[(0, 3), (3, 0)]);
    test(get_antinodes(&[(1, 1), (1, 3)]), &[(1, 5), (1, -1)]);
}

fn get_antinodes(antennae: &[(usize, usize)]) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    for (a, b) in antennae.iter().tuple_combinations() {
        let (a, b) = ((a.0 as i32, a.1 as i32), (b.0 as i32, b.1 as i32));
        let min_column = a.1.min(b.1);
        let max_column = a.1.max(b.1);
        let min_line = a.0.min(b.0);
        let max_line = a.0.max(b.0);
        if (a.0 <= b.0 && a.1 <= b.1) || (b.0 <= a.0 && b.1 <= a.1) {
            antinodes.insert((
                min_line - (max_line - min_line),
                min_column - (max_column - min_column),
            ));
            antinodes.insert((
                max_line + (max_line - min_line),
                max_column + (max_column - min_column),
            ));
        } else {
            antinodes.insert((
                min_line - (max_line - min_line),
                max_column + (max_column - min_column),
            ));
            antinodes.insert((
                max_line + (max_line - min_line),
                min_column - (max_column - min_column),
            ));
        }
    }
    antinodes
}

pub fn part2(input: &str) -> usize {
    let (antennae, input) = parse_input(input);
    let mut antinodes = HashSet::new();
    for antennae in antennae.values() {
        let an = get_multi_antinodes(antennae, input.len(), input[0].len());
        antinodes.extend(an);
    }
    antinodes.len()
}

#[test]
fn test_get_multi_antinodes() {
    let test = |real, expected: &[(i32, i32)]| {
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(real, expected);
    };
    test(
        get_multi_antinodes(&[(3, 3), (5, 3)], 9, 9),
        &[(1, 3), (3, 3), (7, 3), (5, 3)],
    );
    test(
        get_multi_antinodes(&[(3, 3), (5, 5)], 9, 9),
        &[(1, 1), (3, 3), (7, 7), (5, 5)],
    );
    test(
        get_multi_antinodes(&[(2, 2), (3, 3)], 6, 6),
        &[(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)],
    );
    test(
        get_multi_antinodes(&[(2, 3), (3, 2)], 6, 6),
        &[(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)],
    );
    test(
        get_multi_antinodes(&[(3, 5), (5, 3)], 9, 9),
        &[(1, 7), (3, 5), (5, 3), (7, 1)],
    );
}

fn get_multi_antinodes(
    antennae: &[(usize, usize)],
    lines: usize,
    columns: usize,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    let lines = lines as i32;
    let columns = columns as i32;
    for (a, b) in antennae.iter().tuple_combinations() {
        let (a, b) = ((a.0 as i32, a.1 as i32), (b.0 as i32, b.1 as i32));
        let min_line = a.0.min(b.0);
        let max_line = a.0.max(b.0);
        let min_column = a.1.min(b.1);
        let max_column = a.1.max(b.1);
        let line_width = max_line - min_line;
        let column_width = max_column - min_column;
        antinodes.insert(a);
        antinodes.insert(b);
        if (a.0 <= b.0 && a.1 <= b.1) || (b.0 <= a.0 && b.1 <= a.1) {
            let mut p = (min_line - line_width, min_column - column_width);
            loop {
                if p.0 >= 0 && p.1 >= 0 && p.0 < lines && p.1 < columns {
                    antinodes.insert(p);
                    p = (p.0 - line_width, p.1 - column_width);
                } else {
                    break;
                }
            }
            let mut p = (max_line + line_width, max_column + column_width);
            loop {
                if p.0 >= 0 && p.1 >= 0 && p.0 < lines && p.1 < columns {
                    antinodes.insert(p);
                    p = (p.0 + line_width, p.1 + column_width);
                } else {
                    break;
                }
            }
        } else {
            let mut p = (min_line - line_width, max_column + column_width);
            loop {
                if p.0 >= 0 && p.1 >= 0 && p.0 < lines && p.1 < columns {
                    antinodes.insert(p);
                    p = (p.0 - line_width, p.1 + column_width);
                } else {
                    break;
                }
            }
            let mut p = (max_line + line_width, min_column - column_width);
            loop {
                if p.0 >= 0 && p.1 >= 0 && p.0 < lines && p.1 < columns {
                    antinodes.insert(p);
                    p = (p.0 + line_width, p.1 - column_width);
                } else {
                    break;
                }
            }
        }
    }
    antinodes
}
