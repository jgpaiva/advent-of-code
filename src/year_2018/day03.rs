use std::{collections::HashSet, error::Error, str::FromStr};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2018/test_day03");
    assert_eq!(day03(input), "part1: 5 part2: 3");
}

pub fn day03(lines: Vec<String>) -> String {
    let lines = parse(lines);
    let mut intersections = vec![];
    let mut ids_with_intersections = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, other_line) in lines.iter().enumerate() {
            if j > i {
                if let Some(intersection) = line.square.intersect(&other_line.square) {
                    ids_with_intersections.insert(line.id);
                    ids_with_intersections.insert(other_line.id);
                    intersections.push(intersection);
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
fn test_intersect() {
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
struct Input {
    id: i32,
    square: Square,
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = str.split(' ').map(|x| x.to_string()).collect();
        if parts.len() != 4 {
            return Err(Box::<dyn Error>::from(format!(
                "Line needs to have 4 parts, had {}. Line: {}",
                parts.len(),
                str
            )));
        }
        let (x, y) = parts[2].split_once(",").ok_or("couldn't parse x and y")?;
        let (w, h) = parts[3]
            .split_once("x")
            .ok_or("couldn't parse width and height")?;

        let id = parts[0][1..].parse()?;
        Ok(Input {
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
            Some((x, w)) => {
                self.intersection_aux_y(other, false)
                    .map(|(y, h)| Square { x, y, w, h })
            }
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
fn test_parse() {
    assert_eq!(
        parse(utils::to_vec(&[
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2"
        ])),
        vec![
            Input {
                id: 1,
                square: Square {
                    x: 1,
                    y: 3,
                    w: 4,
                    h: 4,
                },
            },
            Input {
                id: 2,
                square: Square {
                    x: 3,
                    y: 1,
                    w: 4,
                    h: 4
                },
            },
            Input {
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

fn parse(lines: Vec<String>) -> Vec<Input> {
    lines
        .iter()
        .map(|line| line.parse::<Input>())
        .collect::<Result<Vec<Input>, Box<dyn Error>>>()
        .unwrap()
}
