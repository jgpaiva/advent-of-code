#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 2);
    assert_eq!(part2(&input), 4);
}

pub fn part1(input: &str) -> i32 {
    let lines = parse_input(input);

    let mut ret_val = 0;
    for line in lines {
        let mut line_sorted = line.clone();
        line_sorted.sort();
        if line != line_sorted {
            line_sorted.reverse();
            if line != line_sorted {
                continue;
            }
        }
        if !line
            .iter()
            .zip(line.iter().skip(1))
            .all(|(v1, v2)| (v1 - v2).abs() <= 3 && (v1 - v2).abs() >= 1)
        {
            continue;
        }
        ret_val += 1;
    }
    ret_val
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let lines: Vec<_> = input.lines().collect();
    let lines: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            let line: Vec<_> = line.split(' ').collect();
            line.into_iter().map(|v| v.parse().unwrap()).collect()
        })
        .collect();
    lines
}

pub fn part2(input: &str) -> i32 {
    let lines = parse_input(input);

    let mut ret_val = 0;
    for line in lines {
        if line.is_empty() || line.len() == 1 || line.len() == 2 {
            ret_val += 1;
            continue;
        }
        if line.len() == 3 {
            let v1 = (line[0] - line[1]).abs();
            let v2 = (line[0] - line[2]).abs();
            let v3 = (line[1] - line[2]).abs();
            if (1..=3).contains(&v1) || (1..=3).contains(&v2) || (1..=3).contains(&v3) {
                ret_val += 1;
            }
            continue;
        }
        if check_line(line) {
            ret_val += 1;
        }
    }
    ret_val
}

fn check_line(mut line: Vec<i32>) -> bool {
    if !is_asc(&line) {
        line.reverse();
    }

    for i in 0..line.len() {
        let mut line = line.clone();
        line.remove(i);

        if line
            .iter()
            .zip(line.iter().skip(1))
            .all(|(v1, v2)| (1..=3).contains(&(*v2 - *v1)))
        {
            return true;
        }
    }
    false
}

// checks if current line is potentially in ascending order
fn is_asc(line: &[i32]) -> bool {
    let mut counter = 0;
    for i in 0..=2 {
        if line[i] < line[i + 1] {
            counter += 1
        }
    }
    counter >= 2
}

#[test]
fn test_is_asc() {
    assert!(is_asc(&[1, 2, 3, 4]));
    assert!(is_asc(&[1, 0, 3, 4]));
    assert!(is_asc(&[1, 2, 0, 4]));
    assert!(is_asc(&[1, 2, 3, 0]));
    assert!(!is_asc(&[4, 3, 2, 1]));
    assert!(!is_asc(&[0, 3, 2, 1]));
    assert!(!is_asc(&[4, 0, 2, 1]));
    assert!(!is_asc(&[4, 3, 0, 1]));
}
