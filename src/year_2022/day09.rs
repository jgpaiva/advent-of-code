use std::{collections::HashSet, error::Error, str::FromStr};

#[cfg(test)]
use {crate::utils, std::fs};

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 13);
    //assert_eq!(part2(&input), 1);
    let input = fs::read_to_string("data/2022/test_day09-2.txt").expect("error reading file");
    assert_eq!(part2(&input), 36);
}

pub fn part2(lines: &str) -> usize {
    let moves: Vec<Move> = lines
        .split_terminator('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let mut tail_pos = [(0, 0); 9];
    let mut head_pos = (0, 0);
    let mut all_tail_pos: HashSet<(i32, i32)> = HashSet::new();
    all_tail_pos.insert((0, 0));
    for mut m in moves {
        while let Some(next_head_pos) = m.move_once(head_pos) {
            head_pos = next_head_pos;
            for i in 0..9 {
                let v = if i == 0 { head_pos } else { tail_pos[i - 1] };
                tail_pos[i] = next_tail_pos(tail_pos[i], v);
            }
            all_tail_pos.insert(tail_pos[8]);
        }
        #[cfg(test)]
        {
            print_map(15, head_pos, &tail_pos);
            println!();
        }
    }
    all_tail_pos.len()
}

#[cfg(test)]
pub fn print_map(size: usize, head: (i32, i32), tails: &[(i32, i32)]) {
    for y in (0..size * 2).rev() {
        let mut line = vec![];
        for x in 0..size * 2 {
            let x = x as i32 - size as i32;
            let y = y as i32 - size as i32;
            if (x, y) == head {
                line.push('H');
            } else {
                let mut v = None;
                for i in (0..9).rev() {
                    if tails[i] == (x, y) {
                        v = Some((i + 1).to_string().chars().next().unwrap());
                    }
                }
                if let Some(v) = v {
                    line.push(v);
                } else if (x, y) == (0, 0) {
                    line.push('s');
                } else {
                    line.push('.');
                }
            }
        }
        println!(
            "{}",
            line.into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

pub fn part1(lines: &str) -> usize {
    let moves: Vec<Move> = lines
        .split_terminator('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let mut tail_pos = (0, 0);
    let mut head_pos = (0, 0);
    let mut all_tail_pos: HashSet<(i32, i32)> = HashSet::new();
    all_tail_pos.insert(tail_pos);
    for mut m in moves {
        while let Some(next_head_pos) = m.move_once(head_pos) {
            head_pos = next_head_pos;
            tail_pos = next_tail_pos(tail_pos, head_pos);
            all_tail_pos.insert(tail_pos);
        }
    }
    all_tail_pos.len()
}

#[derive(Debug)]
enum Dir {
    R,
    L,
    D,
    U,
}

#[derive(Debug)]
struct Move {
    dir: Dir,
    steps: i32,
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s
            .split_once(' ')
            .ok_or_else(|| format!("could not split: {s}"))?;
        let steps = steps.parse()?;
        let dir = match dir
            .chars()
            .next()
            .ok_or_else(|| format!("no first in {dir}"))?
        {
            'R' => Dir::R,
            'L' => Dir::L,
            'D' => Dir::D,
            'U' => Dir::U,
            c => Err(format!("{c} is not acceptable input"))?,
        };
        Ok(Move { dir, steps })
    }
}

fn next_tail_pos(tail_pos: (i32, i32), head_pos: (i32, i32)) -> (i32, i32) {
    let x_diff = head_pos.0 - tail_pos.0;
    let y_diff = head_pos.1 - tail_pos.1;
    match (x_diff, y_diff) {
        // if close enough, don't move
        (x_diff, y_diff) if x_diff.abs() <= 1 && y_diff.abs() <= 1 => tail_pos,
        (2, v) if v.abs() <= 1 => (tail_pos.0 + 1, tail_pos.1 + v),
        (-2, v) if v.abs() <= 1 => (tail_pos.0 - 1, tail_pos.1 + v),
        (v, 2) if v.abs() <= 1 => (tail_pos.0 + v, tail_pos.1 + 1),
        (v, -2) if v.abs() <= 1 => (tail_pos.0 + v, tail_pos.1 - 1),
        (x, y) if x == y && x > 0 => (tail_pos.0 + 1, tail_pos.1 + 1),
        (x, y) if x == y && x < 0 => (tail_pos.0 - 1, tail_pos.1 - 1),
        (x, y) if x == -y && x > 0 => (tail_pos.0 + 1, tail_pos.1 - 1),
        (x, y) if x == -y && x < 0 => (tail_pos.0 - 1, tail_pos.1 + 1),
        _ => panic!("should be unreachable {tail_pos:?}, {head_pos:?}"),
    }
}

impl Move {
    fn move_once(&mut self, pos: (i32, i32)) -> Option<(i32, i32)> {
        if self.steps == 0 {
            return None;
        }
        self.steps -= 1;
        let next_pos = match self.dir {
            Dir::R => (pos.0 + 1, pos.1),
            Dir::L => (pos.0 - 1, pos.1),
            Dir::D => (pos.0, pos.1 - 1),
            Dir::U => (pos.0, pos.1 + 1),
        };
        Some(next_pos)
    }
}
