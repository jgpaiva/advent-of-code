use std::collections::{HashMap, HashSet};

#[cfg(test)]
use crate::utils;

#[test]
fn test_part2() {
    let input = utils::read_lines("2018/test_day06");
    assert_eq!(part2(input, 32), "16");
}

pub fn part2(lines: Vec<String>, n: i32) -> String {
    let map = parse(lines);
    let min_x = map.v.iter().map(|p| p.x).min().unwrap() - 1;
    let max_x = map.v.iter().map(|p| p.x).max().unwrap() + 1;
    let min_y = map.v.iter().map(|p| p.y).min().unwrap() - 1;
    let max_y = map.v.iter().map(|p| p.y).max().unwrap() + 1;
    (min_x..max_x + 1)
        .flat_map(|x| {
            let map = &map;
            (min_y..max_y + 1).map(move |y| {
                i32::from(
                    map.v
                        .iter()
                        .map(|p| p.distance(&Point { x, y }))
                        .sum::<i32>()
                        < n,
                )
            })
        })
        .sum::<i32>()
        .to_string()
}

#[test]
fn test_part1() {
    let input = utils::read_lines("2018/test_day06");
    assert_eq!(part1(input), "17");
}

#[allow(dead_code)]
pub fn part1(lines: Vec<String>) -> String {
    let map = parse(lines);
    let map = map.fill_map();
    let map_height = map.len();
    let map_width = map[0].len();
    let points_to_ignore: HashSet<Point> = map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().map(move |(j, item)| {
                if i == 0 || i == map_height - 1 || j == 0 || j == map_width - 1 {
                    *item
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();
    map.into_iter()
        .flat_map(|line| {
            line.into_iter().map(|item| {
                item.and_then(|p| {
                    if points_to_ignore.contains(&p) {
                        None
                    } else {
                        Some(p)
                    }
                })
            })
        })
        .flatten()
        .fold(HashMap::<Point, i32>::new(), |mut accum, item| {
            *accum.entry(item).or_insert(0) += 1;
            accum
        })
        .into_values()
        .max()
        .unwrap()
        .to_string()
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct PointMap {
    v: Vec<Point>,
}

#[test]
fn test_point_map_to_string() {
    let input = utils::read_lines("2018/test_day06");
    let pm = parse(input);
    assert_eq!(pm.to_string(), "AAAAA·CCCC\nAAAAA·CCCC\nAAADDECCCC\nAADDDECCCC\n··DDDEECCC\nBB·DEEEECC\nBBB·EEEE··\nBBB·EEEFFF\nBBB·EEFFFF\nBBB·FFFFFF\nBBB·FFFFFF\n")
}

#[test]
fn test_min_distance_point() {
    let input = utils::read_lines("2018/test_day06");
    let pm = parse(input);
    assert_eq!(
        pm.min_distance_point(Point { x: 1, y: 1 }),
        Some(Point { x: 1, y: 1 })
    );
    assert_eq!(
        pm.min_distance_point(Point { x: 2, y: 1 }),
        Some(Point { x: 1, y: 1 })
    );
    assert_eq!(pm.min_distance_point(Point { x: 1, y: 4 }), None);
}

impl std::fmt::Display for PointMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890abcdefghijklmnopqrstuvwxyz"
            .chars()
            .collect::<Vec<char>>();
        let mapping: HashMap<Point, char> =
            self.v.iter().zip(names).map(|(n, p)| (*n, p)).collect();
        let full_map = self.fill_map();
        let mut retval = "".to_string();
        for line in full_map {
            for item in line {
                let c = item.and_then(|p| mapping.get(&p)).unwrap_or(&'·');
                retval.push(*c);
            }
            retval.push('\n');
        }
        f.write_str(retval.as_str())
    }
}

impl PointMap {
    fn fill_map(&self) -> Vec<Vec<Option<Point>>> {
        let min_x = self.v.iter().map(|p| p.x).min().unwrap() - 1;
        let max_x = self.v.iter().map(|p| p.x).max().unwrap() + 1;
        let min_y = self.v.iter().map(|p| p.y).min().unwrap() - 1;
        let max_y = self.v.iter().map(|p| p.y).max().unwrap() + 1;
        (min_y..max_y + 1)
            .map(move |y| {
                (min_x..max_x + 1)
                    .map(move |x| self.min_distance_point(Point { x, y }))
                    .collect()
            })
            .collect()
    }

    fn min_distance_point(&self, target: Point) -> Option<Point> {
        let mut ordered: Vec<(Point, i32)> =
            self.v.iter().map(|p| (*p, p.distance(&target))).collect();
        ordered.sort_by(|(_p1, d1), (_p2, d2)| d1.cmp(d2));
        let (p1, d1) = ordered[0];
        let (_p2, d2) = ordered[1];
        if d1 == d2 {
            None
        } else {
            Some(p1)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[test]
fn test_parse() {
    let input = utils::read_lines("2018/test_day06");
    assert_eq!(
        parse(input),
        PointMap {
            v: vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 6 },
                Point { x: 8, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 5, y: 5 },
                Point { x: 8, y: 9 }
            ]
        }
    )
}

fn parse(lines: Vec<String>) -> PointMap {
    PointMap {
        v: lines
            .into_iter()
            .map(|line| {
                let (x, y) = line.split_once(", ").unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect(),
    }
}
