#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2022/test_day01");
    assert_eq!(part1(&input), 24000);
    assert_eq!(part2(&input), 45000);
}

pub fn part2(lines: &[String]) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.parse().ok())
        .collect::<Vec<Option<i32>>>();
    let mut parsed_lines: Vec<Vec<i32>> = vec![vec![]];
    for line in lines {
        match line {
            Some(v) => parsed_lines.last_mut().unwrap().push(v),
            None => parsed_lines.push(vec![]),
        }
    }
    let mut lines: Vec<i32> = parsed_lines
        .into_iter()
        .map(|line| line.into_iter().sum())
        .collect();
    lines.sort();
    lines.reverse();
    lines[0] + lines[1] + lines[2]
}

pub fn part1(lines: &[String]) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.parse().ok())
        .collect::<Vec<Option<i32>>>();
    let mut parsed_lines: Vec<Vec<i32>> = vec![vec![]];
    for line in lines {
        match line {
            Some(v) => parsed_lines.last_mut().unwrap().push(v),
            None => parsed_lines.push(vec![]),
        }
    }
    let mut max_sum = 0;
    for line in parsed_lines {
        let sum = line.iter().sum();
        if sum > max_sum {
            max_sum = sum
        }
    }
    max_sum
}
