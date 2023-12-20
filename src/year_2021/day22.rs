use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let test1 = r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#
        .to_string();
    assert_eq!(part1(test1), 39);
    let test1 = r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#
        .to_string();
    assert_eq!(part2(test1), 39);
    let input = utils::read_file("2021/test_day22-1");
    assert_eq!(part1(input), 590784);
    let input = utils::read_file("2021/test_day22-2");
    assert_eq!(part2(input), 2758514936282235);
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cube {
    is_on: bool,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
}

impl Cube {
    pub fn new(
        is_on: bool,
        (xmin, xmax): (i32, i32),
        (ymin, ymax): (i32, i32),
        (zmin, zmax): (i32, i32),
    ) -> Option<Cube> {
        if xmax >= xmin && ymax >= ymin && zmax >= zmin {
            Some(Cube {
                is_on,
                xmin,
                xmax,
                ymin,
                ymax,
                zmin,
                zmax,
            })
        } else {
            None
        }
    }

    /// divide one cube into up to 26 cubes, to remove the second cube from itself
    /// 1 1 1     . . .    1 2 3
    /// 1 1 1  +  . X . =  4 X 5
    /// 1 1 1     . . .    6 7 8
    fn divide_by(self, c2: &Cube) -> Vec<Cube> {
        // no intersect
        if !(((c2.xmin <= self.xmax) && (self.xmin <= c2.xmax))
            && ((c2.ymin <= self.ymax) && (self.ymin <= c2.ymax))
            && ((c2.zmin <= self.zmax) && (self.zmin <= c2.zmax)))
        {
            return vec![self];
        }
        // some intersect, split the cube
        (0..=2)
            .flat_map(|x| {
                (0..=2).flat_map(move |y| {
                    (0..=2).flat_map(move |z| {
                        if x == 1 && y == 1 && z == 1 {
                            None
                        } else {
                            let x_coords = if x == 0 {
                                (self.xmin, (c2.xmin - 1).min(self.xmax))
                            } else if x == 1 {
                                (c2.xmin.max(self.xmin), c2.xmax.min(self.xmax))
                            } else {
                                ((c2.xmax + 1).max(self.xmin), self.xmax)
                            };
                            let y_coords = if y == 0 {
                                (self.ymin, (c2.ymin - 1).min(self.ymax))
                            } else if y == 1 {
                                (c2.ymin.max(self.ymin), c2.ymax.min(self.ymax))
                            } else {
                                ((c2.ymax + 1).max(self.ymin), self.ymax)
                            };
                            let z_coords = if z == 0 {
                                (self.zmin, (c2.zmin - 1).min(self.zmax))
                            } else if z == 1 {
                                (c2.zmin.max(self.zmin), c2.zmax.min(self.zmax))
                            } else {
                                ((c2.zmax + 1).max(self.zmin), self.zmax)
                            };
                            Cube::new(self.is_on, x_coords, y_coords, z_coords)
                        }
                    })
                })
            })
            .collect()
    }
}

pub fn part2(input: String) -> u64 {
    let input = parse(input);
    input
        .into_iter()
        .fold(vec![], |acc: Vec<Cube>, c| {
            let mut ret = vec![];
            for other_c in acc {
                for cube in other_c.divide_by(&c) {
                    ret.push(cube)
                }
            }
            ret.push(c);
            ret
        })
        .into_iter()
        .filter(|c| c.is_on)
        .map(|c| {
            (c.xmax - c.xmin + 1) as u64
                * (c.ymax - c.ymin + 1) as u64
                * (c.zmax - c.zmin + 1) as u64
        })
        .sum()
}

#[test]
fn test_divide_by() {
    let c1 = Cube::new(true, (0, 2), (0, 2), (0, 2)).unwrap();
    let c2 = Cube::new(false, (3, 3), (3, 3), (3, 3)).unwrap();
    assert_eq!(c1.clone().divide_by(&c2), vec![c1]);
    let c1 = Cube::new(true, (0, 2), (0, 2), (0, 2)).unwrap();
    let c2 = Cube::new(false, (1, 1), (1, 1), (3, 3)).unwrap();
    assert_eq!(c1.clone().divide_by(&c2), vec![c1]);
    let c1 = Cube::new(true, (0, 2), (0, 2), (0, 2)).unwrap();
    let c2 = Cube::new(false, (-1, 3), (-1, 3), (-1, 3)).unwrap();
    assert_eq!(c1.divide_by(&c2), vec![]);
    let c1 = Cube::new(true, (0, 2), (0, 2), (0, 2)).unwrap();
    let c2 = Cube::new(false, (0, 2), (0, 2), (0, 1)).unwrap();
    assert_eq!(
        c1.divide_by(&c2),
        vec![Cube::new(true, (0, 2), (0, 2), (2, 2)).unwrap()]
    );
    let c1 = Cube::new(true, (0, 2), (0, 2), (0, 2)).unwrap();
    let c2 = Cube::new(false, (0, 2), (0, 1), (0, 2)).unwrap();
    assert_eq!(
        c1.divide_by(&c2),
        vec![Cube::new(true, (0, 2), (2, 2), (0, 2)).unwrap()]
    );
    let c1 = Cube::new(true, (0, 2), (0, 2), (0, 2)).unwrap();
    let c2 = Cube::new(false, (1, 1), (1, 1), (1, 1)).unwrap();
    assert_eq!(c1.divide_by(&c2).len(), 26);
}

pub fn part1(input: String) -> usize {
    let input = parse(input);
    let mut is_on = HashMap::<(i32, i32, i32), bool>::new();
    for i in input {
        for x in i.xmin..=i.xmax {
            if !(-50..=50).contains(&x) {
                continue;
            }
            for y in i.ymin..=i.ymax {
                if !(-50..=50).contains(&y) {
                    continue;
                }
                for z in i.zmin..=i.zmax {
                    if !(-50..=50).contains(&z) {
                        continue;
                    }
                    is_on.insert((x, y, z), i.is_on);
                }
            }
        }
    }
    is_on.values().filter(|x| **x).count()
}

fn parse(input: String) -> Vec<Cube> {
    let input = input
        .split_terminator('\n')
        .map(|line| {
            let (on_off, coords) = line.split_once(' ').unwrap();
            let is_on = on_off.starts_with("on");
            let mut v = coords.split(',');
            let x_coords = extract_coords(&mut v);
            let y_coords = extract_coords(&mut v);
            let z_coords = extract_coords(&mut v);
            assert_eq!(v.next(), None);
            Cube {
                is_on,
                xmin: x_coords.0.parse().unwrap(),
                xmax: x_coords.1.parse().unwrap(),
                ymin: y_coords.0.parse().unwrap(),
                ymax: y_coords.1.parse().unwrap(),
                zmin: z_coords.0.parse().unwrap(),
                zmax: z_coords.1.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    input
}

fn extract_coords<'a>(v: &mut std::str::Split<'a, char>) -> (&'a str, &'a str) {
    v.next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split_once("..")
        .unwrap()
}
