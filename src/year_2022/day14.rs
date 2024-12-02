use std::fmt::Display;

use itertools::Itertools;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 24);
    assert_eq!(part2(&input), 93);
}

pub fn part2(lines: &str) -> i32 {
    let input = parse_input(lines);
    let map = create_map(input, true);
    let (map, result) = run_sim(map);
    let y_max = map.y_max;
    let x_max = map.x_max;
    let x_min = map.x_min;
    let right_hand_side: i32 = (0..=(y_max as i32 - (x_max as i32 - 500))).sum();
    let left_hand_side: i32 = (0..=(y_max as i32 - (500 - x_min as i32))).sum();

    #[cfg(test)]
    {
        println!("{}", map,);
        dbg!(&result);
        dbg!(&right_hand_side);
        dbg!(&left_hand_side);
        dbg!(&x_min);
        dbg!(&x_max);
        dbg!(&y_max);
    }

    result as i32 + right_hand_side + left_hand_side
}

#[allow(clippy::needless_range_loop)]
pub fn part1(lines: &str) -> usize {
    let input = parse_input(lines);
    let map = create_map(input, false);
    let (_map, result) = run_sim(map);
    result
}

fn run_sim(mut map: Map) -> (Map, usize) {
    let start_point = (500, 0);
    *map.at(start_point) = El::Sand;
    for i in 0..100000 {
        let mut current_point = start_point;
        loop {
            if current_point.1 == map.y_max {
                return (map, i);
            }
            if try_move(
                &mut map,
                (current_point.0, current_point.1 + 1),
                &mut current_point,
            ) {
                continue;
            }
            if try_move(
                &mut map,
                (current_point.0 - 1, current_point.1 + 1),
                &mut current_point,
            ) {
                continue;
            }
            if try_move(
                &mut map,
                (current_point.0 + 1, current_point.1 + 1),
                &mut current_point,
            ) {
                continue;
            }
            if current_point == start_point {
                return (map, i + 1);
            }
            *map.at(current_point) = El::Sand;
            break;
        }
    }
    unreachable!("reached maximum iterations with no result");
}

fn try_move(map: &mut Map, next_point: (usize, usize), current_point: &mut (usize, usize)) -> bool {
    if let El::Air = map.at(next_point) {
        *map.at(next_point) = El::CurrentSand;
        *map.at(*current_point) = El::Air;
        *current_point = next_point;
        true
    } else {
        false
    }
}

fn parse_input(lines: &str) -> Vec<Vec<(usize, usize)>> {
    lines
        .split_terminator('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|item| {
                    item.split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn create_map(mut input: Vec<Vec<(usize, usize)>>, surrounded: bool) -> Map {
    let all_points: Vec<(usize, usize)> = input.iter().flat_map(|x| x.iter().copied()).collect();
    let x_max = all_points.iter().map(|x| x.0).max().unwrap() + 2;
    let x_min = all_points.iter().map(|x| x.0).min().unwrap() - 2;
    let y_max = all_points.iter().map(|x| x.1).max().unwrap() + 2;
    let mut map = Map::new(x_min, x_max, y_max);
    if surrounded {
        input.push(vec![(x_min, 0), (x_min, y_max), (x_max, y_max), (x_max, 0)])
    }
    for line in input {
        for (start, end) in line.iter().zip(line.iter().skip(1)) {
            if start.0 == end.0 {
                let (start, end) = ((start.0, start.1.min(end.1)), (end.0, start.1.max(end.1)));
                for y in start.1..=end.1 {
                    *map.at((start.0, y)) = El::Rock;
                }
            } else if start.1 == end.1 {
                let (start, end) = ((start.0.min(end.0), start.1), (start.0.max(end.0), end.1));
                for x in start.0..=end.0 {
                    *map.at((x, start.1)) = El::Rock;
                }
            } else {
                unreachable!("lines should be horizontal or vertical")
            }
        }
    }
    map
}

struct Map {
    m: Vec<Vec<El>>,
    x_min: usize,
    x_max: usize,
    y_max: usize,
}

impl Map {
    fn new(x_min: usize, x_max: usize, y_max: usize) -> Self {
        let map: Vec<Vec<El>> = (0..=y_max)
            .map(|_| (x_min..=x_max).map(|_| El::Air).collect())
            .collect();
        Self {
            m: map,
            x_min,
            x_max,
            y_max,
        }
    }

    fn at(&mut self, (x, y): (usize, usize)) -> &mut El {
        &mut self.m[y][x - self.x_min]
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .m
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|el| el.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

enum El {
    Air,
    Rock,
    Sand,
    CurrentSand,
}

impl Display for El {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            El::Air => " ",
            El::Rock => "#",
            El::Sand => "o",
            El::CurrentSand => "x",
        })
    }
}
