use std::collections::{HashMap, HashSet};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day19");
    assert_eq!(part1(input.clone()), 79);
    assert_eq!(part2(input), 3621);
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Rotation {
    X0Z0Y0,
    X1Z0Y0,
    X2Z0Y0,
    X3Z0Y0,
    X0Z1Y0,
    X1Z1Y0,
    X2Z1Y0,
    X3Z1Y0,
    X0Z2Y0,
    X1Z2Y0,
    X2Z2Y0,
    X3Z2Y0,
    X1Z4Y0,
    X2Z4Y0,
    X3Z4Y0,
    X4Z4Y0,
    X0Z0Y1,
    X1Z0Y1,
    X2Z0Y1,
    X3Z0Y1,
    X0Z0Y3,
    X1Z0Y3,
    X2Z0Y3,
    X3Z0Y3,
}

impl Rotation {
    fn apply(&self, v: &P) -> P {
        self.apply_aux(v, false)
    }

    fn apply_reverse(&self, v: &P) -> P {
        self.apply_aux(v, true)
    }

    fn apply_aux(&self, v: &P, reverse: bool) -> P {
        const SIN_000: i32 = 0;
        const SIN_090: i32 = 1;
        const SIN_180: i32 = 0;
        const SIN_270: i32 = -1;
        const COS_000: i32 = 1;
        const COS_090: i32 = 0;
        const COS_180: i32 = -1;
        const COS_270: i32 = 0;

        let x_rotations: Vec<fn(P) -> P> = vec![
            |(x, y, z)| (x, y * COS_000 - z * SIN_000, y * SIN_000 + z * COS_000),
            |(x, y, z)| (x, y * COS_090 - z * SIN_090, y * SIN_090 + z * COS_090),
            |(x, y, z)| (x, y * COS_180 - z * SIN_180, y * SIN_180 + z * COS_180),
            |(x, y, z)| (x, y * COS_270 - z * SIN_270, y * SIN_270 + z * COS_270),
        ];
        let y_rotations: Vec<fn(P) -> P> = vec![
            |(x, y, z)| (x * COS_000 + z * SIN_000, y, -x * SIN_000 + z * COS_000),
            |(x, y, z)| (x * COS_090 + z * SIN_090, y, -x * SIN_090 + z * COS_090),
            |(x, y, z)| (x * COS_180 + z * SIN_180, y, -x * SIN_180 + z * COS_180),
            |(x, y, z)| (x * COS_270 + z * SIN_270, y, -x * SIN_270 + z * COS_270),
        ];
        let z_rotations: Vec<fn(P) -> P> = vec![
            |(x, y, z)| (x * COS_000 - y * SIN_000, x * SIN_000 + y * COS_000, z),
            |(x, y, z)| (x * COS_090 - y * SIN_090, x * SIN_090 + y * COS_090, z),
            |(x, y, z)| (x * COS_180 - y * SIN_180, x * SIN_180 + y * COS_180, z),
            |(x, y, z)| (x * COS_270 - y * SIN_270, x * SIN_270 + y * COS_270, z),
        ];
        let rotations = match self {
            Rotation::X0Z0Y0 => (0, 0, 0),
            Rotation::X1Z0Y0 => (1, 0, 0),
            Rotation::X2Z0Y0 => (2, 0, 0),
            Rotation::X3Z0Y0 => (3, 0, 0),
            Rotation::X0Z1Y0 => (0, 1, 0),
            Rotation::X1Z1Y0 => (1, 1, 0),
            Rotation::X2Z1Y0 => (2, 1, 0),
            Rotation::X3Z1Y0 => (3, 1, 0),
            Rotation::X0Z2Y0 => (0, 2, 0),
            Rotation::X1Z2Y0 => (1, 2, 0),
            Rotation::X2Z2Y0 => (2, 2, 0),
            Rotation::X3Z2Y0 => (3, 2, 0),
            Rotation::X1Z4Y0 => (0, 3, 0),
            Rotation::X2Z4Y0 => (1, 3, 0),
            Rotation::X3Z4Y0 => (2, 3, 0),
            Rotation::X4Z4Y0 => (3, 3, 0),
            Rotation::X0Z0Y1 => (0, 0, 1),
            Rotation::X1Z0Y1 => (1, 0, 1),
            Rotation::X2Z0Y1 => (2, 0, 1),
            Rotation::X3Z0Y1 => (3, 0, 1),
            Rotation::X0Z0Y3 => (0, 0, 3),
            Rotation::X1Z0Y3 => (1, 0, 3),
            Rotation::X2Z0Y3 => (2, 0, 3),
            Rotation::X3Z0Y3 => (3, 0, 3),
        };
        if !reverse {
            let v = y_rotations[rotations.2](*v);
            let v = z_rotations[rotations.1](v);
            x_rotations[rotations.0](v)
        } else {
            let v = x_rotations[(4 - rotations.0) % 4](*v);
            let v = z_rotations[(4 - rotations.1) % 4](v);
            y_rotations[(4 - rotations.2) % 4](v)
        }
    }

    fn rotations() -> Vec<Rotation> {
        vec![
            Rotation::X0Z0Y0,
            Rotation::X1Z0Y0,
            Rotation::X2Z0Y0,
            Rotation::X3Z0Y0,
            Rotation::X0Z1Y0,
            Rotation::X1Z1Y0,
            Rotation::X2Z1Y0,
            Rotation::X3Z1Y0,
            Rotation::X0Z2Y0,
            Rotation::X1Z2Y0,
            Rotation::X2Z2Y0,
            Rotation::X3Z2Y0,
            Rotation::X1Z4Y0,
            Rotation::X2Z4Y0,
            Rotation::X3Z4Y0,
            Rotation::X4Z4Y0,
            Rotation::X0Z0Y1,
            Rotation::X1Z0Y1,
            Rotation::X2Z0Y1,
            Rotation::X3Z0Y1,
            Rotation::X0Z0Y3,
            Rotation::X1Z0Y3,
            Rotation::X2Z0Y3,
            Rotation::X3Z0Y3,
        ]
    }
}

type P = (i32, i32, i32);

pub fn part2(input: String) -> i32 {
    let scanners = parse(input);
    calculate(scanners).1
}

pub fn part1(input: String) -> usize {
    let scanners = parse(input);
    calculate(scanners).0
}

fn calculate(mut scanners: Vec<HashSet<(i32, i32, i32)>>) -> (usize, i32) {
    // this could be used to infer rotations + translations beyond just the pair
    // (e.g. if I can turn 0 into 2, and I can turn 2 into 3, then I can turn 0 into 3)
    // however, I'm already tired of all this, and won't do it
    let mut found_rotation_translations = HashMap::<(usize, usize), (Rotation, P)>::new();
    for first_scanner_i in 0..scanners.len() {
        for second_scanner_i in 0..scanners.len() {
            if first_scanner_i == second_scanner_i {
                continue;
            }
            let (key, max_rotation, first_scanner_i, second_scanner_i) = {
                let key = (first_scanner_i, second_scanner_i);
                let rotation = found_rotation_translations.get(&key).cloned();
                if rotation.is_some() {
                    (key, rotation, second_scanner_i, first_scanner_i)
                } else {
                    let key = (second_scanner_i, first_scanner_i);
                    let rotation = found_rotation_translations.get(&key).cloned();
                    (key, rotation, first_scanner_i, second_scanner_i)
                }
            };
            let max_rotation = max_rotation
                .or_else(|| find_valid_rotation(&scanners, second_scanner_i, first_scanner_i));
            if let Some(max_rotation) = max_rotation {
                let rotation = max_rotation.0;
                let translation = max_rotation.1;
                found_rotation_translations.insert(key, (rotation.clone(), translation));
                let second_scanner: Vec<_> = scanners[second_scanner_i].iter().cloned().collect();
                for point in second_scanner {
                    let point = rotation.apply(&point);
                    let new_point = (
                        point.0 - translation.0,
                        point.1 - translation.1,
                        point.2 - translation.2,
                    );
                    scanners[first_scanner_i].insert(new_point);
                }
                let first_scanner: Vec<_> = scanners[first_scanner_i].iter().cloned().collect();
                for point in first_scanner {
                    let new_point = (
                        point.0 + translation.0,
                        point.1 + translation.1,
                        point.2 + translation.2,
                    );
                    let new_point = rotation.apply_reverse(&new_point);
                    scanners[second_scanner_i].insert(new_point);
                }
            }
        }
    }
    let max_distance = found_rotation_translations
        .values()
        .map(|(_r, t)| t.0.abs() + t.1.abs() + t.2.abs())
        .max()
        .unwrap();
    (scanners[0].len(), max_distance)
}

fn find_valid_rotation(
    scanners: &[HashSet<(i32, i32, i32)>],
    second_scanner_i: usize,
    first_scanner_i: usize,
) -> Option<(Rotation, (i32, i32, i32))> {
    let mut rotations_and_translations = HashMap::<(Rotation, P), u32>::new();
    for rotation in Rotation::rotations() {
        for point in scanners[second_scanner_i].iter() {
            let new_point = rotation.apply(point);
            for i in scanners[first_scanner_i].iter().cloned() {
                let translation = (new_point.0 - i.0, new_point.1 - i.1, new_point.2 - i.2);
                let e = rotations_and_translations
                    .entry((rotation.clone(), translation))
                    .or_insert(0);
                *e += 1;
                if *e >= 12 {
                    return Some((rotation, translation));
                }
            }
        }
    }
    None
}

fn parse(input: String) -> Vec<HashSet<(i32, i32, i32)>> {
    input
        .split_terminator('\n')
        .fold(Vec::new(), |mut acc, line| {
            if line.is_empty() {
            } else if line.starts_with("---") {
                acc.push(HashSet::new());
            } else {
                let acc_len = acc.len();
                let scanner = &mut acc[acc_len - 1];
                let mut it = line
                    .split_terminator(',')
                    .into_iter()
                    .map(|v| v.parse().unwrap());
                let point: (i32, i32, i32) =
                    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
                assert!(it.next().is_none());
                scanner.insert(point);
            }
            acc
        })
}
