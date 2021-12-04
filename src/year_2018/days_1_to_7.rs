use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

#[cfg(test)]
use crate::utils;

#[test]
fn test_day7_2018() {
    assert_eq!(
        day7_2018(utils::to_vec(&[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin."
        ])),
        "CABDFE"
    );
}
#[allow(dead_code)]
fn day7_2018_v2(lines: Vec<String>) -> String {
    let input = day7_2018_parse(lines);
    let mut edges = HashMap::new();
    for dep in &input.deps {
        edges.entry(dep.from).or_insert(vec![]).push(dep.to);
    }
    let mut output: Vec<char> = vec![];
    let mut open_nodes: HashSet<char> = input
        .deps
        .iter()
        .flat_map(|Dep { from, to }| vec![*from, *to])
        .collect();
    while !open_nodes.is_empty() {
        let mut open_nodes_sorted: Vec<char> = open_nodes.iter().map(|x| *x).collect();
        open_nodes_sorted.sort();
        let current_node = open_nodes_sorted.into_iter().next().unwrap();
        let mut visiting: HashSet<char> = HashSet::new();
        visit(
            current_node,
            &mut visiting,
            &mut open_nodes,
            &mut output,
            &edges,
        );
    }
    output.into_iter().collect()
}

pub fn day7_2018(lines: Vec<String>) -> String {
    let input = day7_2018_parse(lines);
    let mut edges = HashMap::new();
    for dep in &input.deps {
        edges
            .entry(dep.from)
            .or_insert(HashSet::<char>::new())
            .insert(dep.to);
    }
    let mut output: Vec<char> = vec![];
    let mut remaining_nodes: HashSet<char> = input
        .deps
        .iter()
        .flat_map(|Dep { from, to }| vec![*from, *to])
        .collect();
    for node in &remaining_nodes {
        edges.entry(*node).or_insert(HashSet::<char>::new());
    }
    while !remaining_nodes.is_empty() {
        let mut candidates: Vec<char> = edges
            .iter()
            .filter(|(_k, v)| {
                let branches: HashSet<_> = v.intersection(&remaining_nodes).collect();
                branches.is_empty()
            })
            .map(|(k, _v)| *k)
            .collect();
        candidates.sort();
        let node = candidates[0];
        output.push(node);
        remaining_nodes.remove(&node);
        edges.remove(&node);
    }
    output.into_iter().collect()
}

fn visit(
    node: char,
    visiting: &mut HashSet<char>,
    open_nodes: &mut HashSet<char>,
    output: &mut Vec<char>,
    edges: &HashMap<char, Vec<char>>,
) {
    if !open_nodes.contains(&node) {
        return;
    }
    if visiting.contains(&node) {
        panic!(
            "loop found in input? node: {} visiting: {:?}",
            node, visiting
        );
    }

    visiting.insert(node);
    let mut node_s_edges: Vec<char> = edges
        .get(&node)
        .unwrap_or(&vec![])
        .iter()
        .map(|x| *x)
        .collect();
    node_s_edges.sort();
    for node in node_s_edges {
        visit(node, visiting, open_nodes, output, edges);
    }
    visiting.remove(&node);

    open_nodes.remove(&node);
    output.push(node);
}

#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
struct day7_2018_Graph {
    deps: Vec<Dep>,
}

#[derive(Debug, Eq, PartialEq)]
struct Dep {
    from: char,
    to: char,
}

#[test]
fn test_day7_2018_parse() {
    assert_eq!(
        day7_2018_parse(utils::to_vec(&[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
        ])),
        day7_2018_Graph {
            deps: vec![
                Dep { to: 'C', from: 'A' },
                Dep { to: 'C', from: 'F' },
                Dep { to: 'A', from: 'B' },
            ]
        }
    );
}

fn day7_2018_parse(lines: Vec<String>) -> day7_2018_Graph {
    day7_2018_Graph {
        deps: lines
            .iter()
            .map(|line| {
                let (to, from) = line
                    .split_once(" must be finished before step ")
                    .expect("line needs to be splitable");
                let to = to
                    .chars()
                    .nth(to.len() - 1)
                    .expect("line needs to be longer than 1 char");
                let from = from
                    .chars()
                    .next()
                    .expect("line needs to have at least one item");
                Dep { from, to }
            })
            .collect(),
    }
}

#[test]
fn test_day6_2018_part2() {
    assert_eq!(
        day6_2018_part2(
            utils::to_vec(&["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"]),
            32
        ),
        "16"
    );
}

pub fn day6_2018_part2(lines: Vec<String>, n: i32) -> String {
    let map = day6_2018_parse(lines);
    let min_x = map.v.iter().map(|p| p.x).min().unwrap() - 1;
    let max_x = map.v.iter().map(|p| p.x).max().unwrap() + 1;
    let min_y = map.v.iter().map(|p| p.y).min().unwrap() - 1;
    let max_y = map.v.iter().map(|p| p.y).max().unwrap() + 1;
    (min_x..max_x + 1)
        .flat_map(|x| {
            let map = &map;
            (min_y..max_y + 1).map(move |y| {
                if map
                    .v
                    .iter()
                    .map(|p| p.distance(&Point { x, y }))
                    .sum::<i32>()
                    < n
                {
                    1
                } else {
                    0
                }
            })
        })
        .sum::<i32>()
        .to_string()
}

#[test]
fn test_day6_2018() {
    assert_eq!(
        day6_2018(utils::to_vec(&[
            "1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"
        ])),
        "17"
    );
}

pub fn day6_2018(lines: Vec<String>) -> String {
    let map = day6_2018_parse(lines);
    let map = map.fill_map();
    let map_height = map.len();
    let map_width = map[0].len();
    let points_to_ignore: HashSet<Point> = map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.into_iter().enumerate().map(move |(j, item)| {
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
        .into_iter()
        .map(|(_k, v)| v)
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
    let pm = day6_2018_parse(utils::to_vec(&[
        "1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9",
    ]));
    assert_eq!(pm.to_string(), "AAAAA·CCCC\nAAAAA·CCCC\nAAADDECCCC\nAADDDECCCC\n··DDDEECCC\nBB·DEEEECC\nBBB·EEEE··\nBBB·EEEFFF\nBBB·EEFFFF\nBBB·FFFFFF\nBBB·FFFFFF\n")
}

#[test]
fn test_min_distance_point() {
    let pm = day6_2018_parse(utils::to_vec(&[
        "1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9",
    ]));
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
fn test_day6_2018_parse() {
    assert_eq!(
        day6_2018_parse(utils::to_vec(&[
            "1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"
        ])),
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

fn day6_2018_parse(lines: Vec<String>) -> PointMap {
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

#[test]
fn test_day5_2018() {
    assert_eq!(
        day5_2018(utils::to_vec(&["dabAcCaCBAcCcaDA"])),
        "part 1: 10 part2: 4"
    );
}

pub fn day5_2018(lines: Vec<String>) -> String {
    let polymer = lines.into_iter().next().unwrap();
    let a_to_z = "abcdefghijklmnopqrstuvwxyz";
    let a_to_z_capital = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut mapping = HashMap::<char, char>::new();
    for (char1, char2) in a_to_z.chars().zip(a_to_z_capital.chars()) {
        mapping.insert(char1, char2);
        mapping.insert(char2, char1);
    }
    let mut min_size = None;
    let mut _min_polymer = None;
    for c in a_to_z.chars() {
        let other_c = *mapping.get(&c).unwrap();
        let new_polymer: String = polymer
            .chars()
            .filter(|x| *x != c && *x != other_c)
            .collect();
        let (size, polymer) = day5_2018_react(&mapping, new_polymer);
        match min_size {
            None => {
                min_size = Some(size);
                _min_polymer = Some(polymer);
            }
            Some(current_min_size) => {
                if current_min_size > size {
                    min_size = Some(size);
                    _min_polymer = Some(polymer);
                }
            }
        };
    }
    let (size, _final_polymer) = day5_2018_react(&mapping, polymer);
    format!("part 1: {} part2: {}", size, min_size.unwrap())
}

fn day5_2018_react(mapping: &HashMap<char, char>, polymer: String) -> (usize, String) {
    let mut queue: Vec<char> = vec![];
    for c in polymer.chars() {
        if let Some(top_of_queue) = queue.pop() {
            if *(mapping.get(&top_of_queue).unwrap()) == c {
                // two reacted, top one removed
                continue;
            }
            queue.push(top_of_queue);
        }
        queue.push(c);
    }
    let size = queue.len();
    let final_polymer: String = queue.iter().collect();
    (size, final_polymer)
}

#[test]
fn test_day4_2018() {
    assert_eq!(
        day4_2018(utils::to_vec(&[
            "[1518-11-05 00:55] wakes up",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
        ])),
        "part 1: 240 part2: 4455"
    );
}

pub fn day4_2018(lines: Vec<String>) -> String {
    #[derive(Debug)]
    struct SleepCycle {
        id: i32,
        duration: i32,
        start: i32,
        end: i32,
    }
    let lines = day4_2018_parse(lines);
    let mut sleep_cycles = vec![];
    let mut current_guard = None;
    let mut start_sleep = None;
    let mut current_day = None;
    for line in lines {
        match line.state {
            day4_2018State::BeginShift { id } => {
                current_guard = Some(id);
                assert_eq!(start_sleep, None);
            }
            day4_2018State::FallAsleep => {
                assert_eq!(start_sleep, None);
                start_sleep = Some(line.ts.minute);
                current_day = Some(line.ts.day);
            }
            day4_2018State::WakeUp => {
                if current_day.unwrap() != line.ts.day {
                    panic!(
                        "current_day: {:?} line: {:?} guard: {:?}",
                        current_day, line, current_guard
                    );
                }
                sleep_cycles.push(SleepCycle {
                    id: current_guard.unwrap(),
                    duration: line.ts.minute - start_sleep.unwrap(),
                    start: start_sleep.unwrap(),
                    end: line.ts.minute,
                });
                start_sleep = None;
            }
        }
    }
    let mut id_to_duration = HashMap::new();
    for cycle in &sleep_cycles {
        let counter = id_to_duration.entry(cycle.id).or_insert(0);
        *counter += cycle.duration;
    }
    let (max_sleep_id, _duration) = id_to_duration
        .into_iter()
        .max_by(|(_id1, dur1), (_id2, dur2)| dur1.cmp(dur2))
        .unwrap();
    let best_sleeper_sleep_cycles: Vec<&SleepCycle> = (&sleep_cycles)
        .into_iter()
        .filter(|x| x.id == max_sleep_id)
        .collect();
    let mut minute_to_sleep = HashMap::new();
    for cycle in best_sleeper_sleep_cycles {
        for minute in cycle.start..cycle.end {
            let counter = minute_to_sleep.entry(minute).or_insert(0);
            *counter += 1;
        }
    }
    let (max_sleep_minute, _count) = minute_to_sleep
        .into_iter()
        .max_by(|(_minute1, count1), (_minute2, count2)| count1.cmp(count2))
        .unwrap();
    let mut sleeper_per_minute: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for cycle in &sleep_cycles {
        for minute in cycle.start..cycle.end {
            let per_id_counter = sleeper_per_minute.entry(minute).or_insert(HashMap::new());
            let counter = per_id_counter.entry(cycle.id).or_insert(0);
            *counter += 1;
        }
    }
    let mut max_sleeper = None;
    for (minute, sleepers) in sleeper_per_minute {
        for (id, counter) in sleepers {
            max_sleeper = match max_sleeper {
                None => Some((id, minute, counter)),
                Some((id_best, minute_best, counter_best)) => {
                    if counter_best < counter {
                        Some((id, minute, counter))
                    } else {
                        Some((id_best, minute_best, counter_best))
                    }
                }
            }
        }
    }
    let (id_best, minute_best, _counter_best) = max_sleeper.unwrap();
    format!(
        "part 1: {} part2: {}",
        max_sleep_id * max_sleep_minute,
        id_best * minute_best
    )
}

#[test]
fn test_day4_2018_parse() {
    assert_eq!(
        day4_2018_parse(utils::to_vec(&[
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-02 00:25] wakes up",
            "[1518-11-02 00:00] Guard #11 begins shift",
            "[1518-11-01 23:59] falls asleep",
            "[1518-09-27 00:59] Guard #12 begins shift",
        ])),
        vec![
            day4_2018Input {
                ts: Ts {
                    day: 927,
                    minute: 59
                },
                state: day4_2018State::BeginShift { id: 12 }
            },
            day4_2018Input {
                ts: Ts {
                    day: 1101,
                    minute: 0
                },
                state: day4_2018State::BeginShift { id: 10 }
            },
            day4_2018Input {
                ts: Ts {
                    day: 1101,
                    minute: 5
                },
                state: day4_2018State::FallAsleep
            },
            day4_2018Input {
                ts: Ts {
                    day: 1102,
                    minute: -1
                },
                state: day4_2018State::FallAsleep
            },
            day4_2018Input {
                ts: Ts {
                    day: 1102,
                    minute: 0
                },
                state: day4_2018State::BeginShift { id: 11 }
            },
            day4_2018Input {
                ts: Ts {
                    day: 1102,
                    minute: 25
                },
                state: day4_2018State::WakeUp
            },
        ]
    );
}

#[derive(PartialEq, Eq, Debug, Hash)]
#[allow(non_camel_case_types)]
struct day4_2018Input {
    state: day4_2018State,
    ts: Ts,
}
type GuardId = i32;
#[derive(PartialEq, Eq, Debug, Hash)]
#[allow(non_camel_case_types)]
enum day4_2018State {
    BeginShift { id: GuardId },
    FallAsleep,
    WakeUp,
}

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Ts {
    day: i32,
    minute: i32,
}

impl FromStr for day4_2018Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ts, rest) = s
            .split_once("]")
            .ok_or(format!("couldn't parse line {}", s))?;
        let (_, ts) = ts
            .split_once("1518-")
            .ok_or(format!("couldn't parse line {}", s))?;
        let (day, minute) = ts
            .split_once(" ")
            .ok_or(format!("couldn't parse line {}", s))?;
        let day: i32 = day
            .chars()
            .filter(|x| *x != '-')
            .collect::<String>()
            .parse()?;
        let minute: i32 = minute
            .chars()
            .filter(|x| *x != ':')
            .collect::<String>()
            .parse()?;
        let (day, minute) = if minute > 60 {
            (day + 1, -(2360 - minute))
        } else {
            (day, minute)
        };
        let ts = Ts { day, minute };
        let c = rest
            .chars()
            .skip(1)
            .next()
            .ok_or(format!("couldn't parse line {}", s))?;
        match c {
            'f' => Ok(day4_2018Input {
                ts,
                state: day4_2018State::FallAsleep,
            }),
            'G' => {
                let (id, _) = rest
                    .split_once(" begins")
                    .ok_or(format!("couldn't parse line {}", s))?;
                let id: i32 = id.chars().skip(8).collect::<String>().parse()?;
                Ok(day4_2018Input {
                    ts,
                    state: day4_2018State::BeginShift { id },
                })
            }
            'w' => Ok(day4_2018Input {
                ts,
                state: day4_2018State::WakeUp,
            }),
            _ => unreachable!(),
        }
    }
}

fn day4_2018_parse(lines: Vec<String>) -> Vec<day4_2018Input> {
    let mut lines: Vec<_> = lines
        .iter()
        .map(|line| line.parse::<day4_2018Input>().unwrap())
        .collect();
    lines.sort_by(|p1, p2| p1.ts.cmp(&p2.ts));
    lines
}

#[test]
fn test_day3_2018() {
    assert_eq!(
        day3_2018(utils::to_vec(&[
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2",
            "#4 @ 2,3: 2x1"
        ])),
        "part1: 5 part2: 3"
    );
}

pub fn day3_2018(lines: Vec<String>) -> String {
    let lines = day3_2018_parse(lines);
    let mut intersections = vec![];
    let mut ids_with_intersections = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, other_line) in lines.iter().enumerate() {
            if j > i {
                match line.square.intersect(&other_line.square) {
                    Some(intersection) => {
                        ids_with_intersections.insert(line.id);
                        ids_with_intersections.insert(other_line.id);
                        intersections.push(intersection);
                    }
                    _ => {}
                }
            }
        }
    }
    let mut mini_squares = HashSet::new();
    for intersection in intersections {
        for mini_square in intersection.decompose() {
            mini_squares.insert(mini_square);
        }
    }
    let part1 = mini_squares.iter().map(|x| x.area()).sum::<i32>();
    let all_ids = lines.iter().map(|x| x.id).collect::<HashSet<_>>();
    let part2: Vec<_> = all_ids.difference(&ids_with_intersections).collect();
    assert_eq!(part2.len(), 1);
    format!("part1: {} part2: {}", part1, part2[0])
}

#[test]
fn test_day3_2018_intersect() {
    assert_eq!(
        Square {
            x: 1,
            y: 3,
            w: 4,
            h: 4
        }
        .intersect(&Square {
            x: 0,
            y: 1,
            w: 1,
            h: 1,
        }),
        None
    );
    assert_eq!(
        Square {
            x: 1,
            y: 3,
            w: 4,
            h: 4,
        }
        .intersect(&Square {
            x: 1,
            y: 3,
            w: 4,
            h: 4,
        }),
        Some(Square {
            x: 1,
            y: 3,
            w: 4,
            h: 4,
        })
    );
    assert_eq!(
        Square {
            x: 1,
            y: 0,
            w: 2,
            h: 1,
        }
        .intersect(&Square {
            x: 2,
            y: 0,
            w: 1,
            h: 1,
        }),
        Some(Square {
            x: 2,
            y: 0,
            w: 1,
            h: 1
        })
    );
    assert_eq!(
        Square {
            x: 5,
            y: 5,
            w: 2,
            h: 2
        }
        .intersect(&Square {
            x: 3,
            y: 1,
            w: 4,
            h: 4
        }),
        None
    )
}

#[derive(PartialEq, Eq, Debug, Hash)]
#[allow(non_camel_case_types)]
struct day3_2018Input {
    id: i32,
    square: Square,
}

impl FromStr for day3_2018Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(" ").map(|x| x.to_string()).collect();
        if parts.len() != 4 {
            return Err(Box::<dyn Error>::from(format!(
                "Line needs to have 4 parts, had {}. Line: {}",
                parts.len(),
                s
            )));
        }
        let (x, y) = parts[2].split_once(",").ok_or("couldn't parse x and y")?;
        let (w, h) = parts[3]
            .split_once("x")
            .ok_or("couldn't parse width and height")?;

        let id = parts[0][1..].parse()?;
        Ok(day3_2018Input {
            id,
            square: Square {
                x: x.parse()?,
                y: y[..y.len() - 1].parse()?,
                w: w.parse()?,
                h: h.parse()?,
            },
        })
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Square {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Square {
    fn area(&self) -> i32 {
        self.w * self.h
    }
    fn intersect(&self, other: &Square) -> Option<Square> {
        match self.intersection_aux_x(other, false) {
            Some((x, w)) => match self.intersection_aux_y(other, false) {
                Some((y, h)) => Some(Square { x, y, w, h }),
                None => None,
            },
            None => None,
        }
    }
    fn intersection_aux_x(&self, other: &Square, checked_other: bool) -> Option<(i32, i32)> {
        if self.x + self.w > other.x {
            let x = self.x.max(other.x);
            let x_plus_w = (self.x + self.w).min(other.x + other.w);
            if x < x_plus_w {
                return Some((x, x_plus_w - x));
            }
        }
        if checked_other {
            None
        } else {
            other.intersection_aux_x(self, true)
        }
    }

    fn intersection_aux_y(&self, other: &Square, checked_other: bool) -> Option<(i32, i32)> {
        if self.y + self.h > other.y {
            let y = self.y.max(other.y);
            let y_plus_h = (self.y + self.h).min(other.y + other.h);
            if y < y_plus_h {
                return Some((y, y_plus_h - y));
            }
        }
        if checked_other {
            None
        } else {
            other.intersection_aux_y(self, true)
        }
    }

    fn decompose(&self) -> Vec<Square> {
        (0..self.w)
            .flat_map(|i| {
                let x = self.x + i;
                (0..self.h).map(move |j| {
                    let y = self.y + j;
                    Square { x, y, w: 1, h: 1 }
                })
            })
            .collect()
    }
}

#[test]
fn test_day3_2018_parse() {
    assert_eq!(
        day3_2018_parse(utils::to_vec(&[
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2"
        ])),
        vec![
            day3_2018Input {
                id: 1,
                square: Square {
                    x: 1,
                    y: 3,
                    w: 4,
                    h: 4,
                },
            },
            day3_2018Input {
                id: 2,
                square: Square {
                    x: 3,
                    y: 1,
                    w: 4,
                    h: 4
                },
            },
            day3_2018Input {
                id: 3,
                square: Square {
                    x: 5,
                    y: 5,
                    w: 2,
                    h: 2
                }
            }
        ]
    );
}

fn day3_2018_parse(lines: Vec<String>) -> Vec<day3_2018Input> {
    lines
        .iter()
        .map(|line| line.parse::<day3_2018Input>())
        .collect::<Result<Vec<day3_2018Input>, Box<dyn Error>>>()
        .unwrap()
}

#[test]
fn test_day2_2018() {
    assert_eq!(
        day2_2018(utils::to_vec(&[
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ])),
        "12"
    );
}

pub fn day2_2018(lines: Vec<String>) -> String {
    let (two_letter, three_letter) =
        lines
            .iter()
            .fold((0, 0), |(two_letter, three_letter), line| {
                let (v1, v2) = day2_2018_aux(line);
                (two_letter + v1, three_letter + v2)
            });
    format!("{}", two_letter * three_letter)
}

#[test]
fn test_day2_2018_part2() {
    assert_eq!(
        day2_2018_part2(utils::to_vec(&[
            "aaaa", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ])),
        "fgij"
    );
    assert_eq!(
        day2_2018_part2(utils::to_vec(&[
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ])),
        "fgij"
    );
}

pub fn day2_2018_part2(lines: Vec<String>) -> String {
    let mut h = HashSet::new();
    for line in lines {
        let line = line.chars().collect::<Vec<_>>();
        let mut line_set = HashSet::new();
        for i in 0..line.len() {
            let line: String = line
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, c)| c)
                .collect();
            line_set.insert(line);
        }
        for line in line_set {
            if !h.insert(line.to_owned()) {
                return line;
            }
        }
    }
    "".to_string()
}

#[test]
fn test_day2_2018_aux() {
    assert_eq!(day2_2018_aux(&"abcdef".to_string()), (0, 0));
    assert_eq!(day2_2018_aux(&"bababc".to_string()), (1, 1));
    assert_eq!(day2_2018_aux(&"abbcde".to_string()), (1, 0));
    assert_eq!(day2_2018_aux(&"abcccd".to_string()), (0, 1));
    assert_eq!(day2_2018_aux(&"aabcdd".to_string()), (1, 0));
    assert_eq!(day2_2018_aux(&"abcdee".to_string()), (1, 0));
    assert_eq!(day2_2018_aux(&"ababab".to_string()), (0, 1));
    assert_eq!(day2_2018_aux(&"aaa".to_string()), (0, 1));
    assert_eq!(day2_2018_aux(&"aa".to_string()), (1, 0));
    assert_eq!(day2_2018_aux(&"aaaa".to_string()), (0, 0));
    assert_eq!(day2_2018_aux(&"caaaabb".to_string()), (1, 0));
}

fn day2_2018_aux(s: &String) -> (i32, i32) {
    let mut sorted = s.chars().collect::<Vec<_>>();
    sorted.sort();
    let mut two_letter = 0;
    let mut three_letter = 0;
    let mut i = 0;
    while i < sorted.len() {
        if sorted.len() == i + 1 {
            //done, nothing else to check
            i += 1;
        } else if sorted[i + 1] != sorted[i] {
            // move forward, next is different char
            i += 1;
        } else if sorted.len() == i + 2 {
            // there's only one more, and it's the same
            two_letter = 1;
            i += 2;
        } else if sorted[i + 2] != sorted[i] {
            // there's two equal in a row, and then a different one
            two_letter = 1;
            i += 2;
        } else if sorted.len() == i + 3 {
            // there's three equal in a row and then ends
            three_letter = 1;
            i += 3;
        } else if sorted[i + 3] != sorted[i] {
            // there's three in a row, and then a different one
            three_letter = 1;
            i += 3;
        } else {
            // at least 4 in a row of the same, consume everything
            let mut j = i + 1;
            while j < sorted.len() && sorted[j] == sorted[i] {
                j += 1;
            }
            i = j;
        }
    }
    (two_letter, three_letter)
}

#[test]
fn test_day1_2018() {
    assert_eq!(day1_2018(utils::to_vec(&["3", "3", "4", "-2", "-4"])), "10");
}

pub fn day1_2018(lines: Vec<String>) -> String {
    let a = lines
        .iter()
        .map(|line| {
            let a: Result<i32, _> = line.parse();
            a.expect("could not parse line as integer")
        })
        .cycle()
        .try_fold(
            (|| {
                let mut v = HashSet::new();
                v.insert(0);
                (v, 0)
            })(),
            |(mut accum, last), v| {
                let next = last + v;
                if accum.insert(next) {
                    Ok((accum, next))
                } else {
                    Err((accum, next))
                }
            },
        );
    if let Err((_accum, v)) = a {
        format!("{}", v)
    } else {
        unreachable!();
    }
}
