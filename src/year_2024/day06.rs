use std::collections::{HashMap, HashSet};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 41);
    assert_eq!(part2(&input), 6);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1(input: &str) -> i32 {
    find_path(input).unwrap()
}

pub fn find_path(input: &str) -> Result<i32, ()> {
    let input: Vec<_> = input.split("\n").collect();
    let mut p = (0, 0);
    let mut obstacles_line: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut obstacles_column: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut direction = Direction::Up;
    for (i_line, line) in input.iter().enumerate() {
        let line = line.as_bytes();
        for (i_column, item) in line.iter().enumerate() {
            if *item == b'#' {
                let line = obstacles_line.entry(i_line).or_default();
                line.push((i_line, i_column));
                let line = obstacles_column.entry(i_column).or_default();
                line.push((i_line, i_column));
            } else if *item == b'^' {
                p = (i_line, i_column);
            }
        }
    }
    let mut retval = HashSet::new();
    let mut move_p = |start: &mut (usize, usize), end: (usize, usize), direction| {
        if start.0 == end.0 {
            let (start, end) = if start.1 > end.1 {
                (end, *start)
            } else {
                (*start, end)
            };
            for i in start.1..=end.1 {
                if !retval.insert(((start.0, i), direction)) {
                    return Err(());
                }
            }
        } else {
            assert_eq!(start.1, end.1);
            let (start, end) = if start.0 > end.0 {
                (end, *start)
            } else {
                (*start, end)
            };
            for i in start.0..=end.0 {
                if !retval.insert(((i, start.1), direction)) {
                    return Err(());
                }
            }
        }
        *start = end;
        Ok(())
    };
    let empty_vec = Vec::new();
    loop {
        match direction {
            Direction::Up => {
                let obstacle = obstacles_column
                    .get(&p.1)
                    .unwrap_or(&empty_vec)
                    .iter()
                    .filter(|(line, _)| *line < p.0)
                    .max_by(|(line1, _), (line2, _)| line1.cmp(line2));
                if let Some(obstacle) = obstacle {
                    move_p(&mut p, (obstacle.0 + 1, obstacle.1), direction)?;
                    direction = Direction::Right;
                } else {
                    // no obstacle in the way, done
                    let column = p.1;
                    move_p(&mut p, (0, column), direction)?;
                    break;
                }
            }
            Direction::Down => {
                let obstacle = obstacles_column
                    .get(&p.1)
                    .unwrap_or(&empty_vec)
                    .iter()
                    .filter(|(line, _)| *line > p.0)
                    .min_by(|(a, _), (b, _)| a.cmp(b));
                if let Some(obstacle) = obstacle {
                    move_p(&mut p, (obstacle.0 - 1, obstacle.1), direction)?;
                    direction = Direction::Left;
                } else {
                    // no obstacle in the way, done
                    let column = p.1;
                    move_p(&mut p, (input.len() - 1, column), direction)?;
                    break;
                }
            }
            Direction::Left => {
                let obstacle = obstacles_line
                    .get(&p.0)
                    .unwrap_or(&empty_vec)
                    .iter()
                    .filter(|(_, item)| *item < p.1)
                    .max_by(|(_, a), (_, b)| a.cmp(b));
                if let Some(obstacle) = obstacle {
                    move_p(&mut p, (obstacle.0, obstacle.1 + 1), direction)?;
                    direction = Direction::Up;
                } else {
                    // no obstacle in the way, done
                    let line_i = p.0;
                    move_p(&mut p, (line_i, 0), direction)?;
                    break;
                }
            }
            Direction::Right => {
                let obstacle = obstacles_line
                    .get(&p.0)
                    .unwrap_or(&empty_vec)
                    .iter()
                    .filter(|(_, item)| *item > p.1)
                    .min_by(|(_, a), (_, b)| a.cmp(b));
                if let Some(obstacle) = obstacle {
                    move_p(&mut p, (obstacle.0, obstacle.1 - 1), direction)?;
                    direction = Direction::Down;
                } else {
                    // no obstacle in the way, done
                    let line_i = p.0;
                    move_p(&mut p, (line_i, input[0].len() - 1), direction)?;
                    break;
                }
            }
        }
    }
    let retval: HashSet<_> = retval.into_iter().map(|(v, _dir)| v).collect();
    Ok(retval.len() as i32)
}

pub fn part2(input: &str) -> i32 {
    let mut retval = 0;
    for i in 0..input.len() {
        let mut new_input = input.as_bytes().to_vec();
        if new_input[i] == b'.' {
            new_input[i] = b'#';
        } else {
            continue;
        }
        let input = String::from_utf8(new_input).unwrap();
        match find_path(&input) {
            Ok(_) => {} // uninteresting
            Err(_) => retval += 1,
        }
    }
    retval
}
