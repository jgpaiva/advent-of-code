use regex::Regex;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 130);
    assert_eq!(part2(&input), 48);
}

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r#"mul\((\d\d?\d?),(\d\d?\d?)\)"#).unwrap();
    let mut ret_val = 0;
    for caps in re.captures_iter(input) {
        let v1: i32 = caps[2].parse().unwrap();
        let v2: i32 = caps[2].parse().unwrap();
        ret_val += v1 * v2;
    }
    ret_val
}

enum Instructions {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r#"(mul\((\d\d?\d?),(\d\d?\d?)\))|(do\(\))|(don't\(\))"#).unwrap();
    let instructions: Vec<_> = re
        .captures_iter(input)
        .map(|caps| {
            if caps.get(4).is_some() {
                Instructions::Do
            } else if caps.get(5).is_some() {
                Instructions::Dont
            } else {
                let v1 = caps[2].parse().unwrap();
                let v2 = caps[3].parse().unwrap();
                Instructions::Mul(v1, v2)
            }
        })
        .collect();
    let mut ret_val = 0;
    let mut enabled = true;
    for i in instructions {
        match i {
            Instructions::Mul(v1, v2) => {
                if enabled {
                    ret_val += v1 * v2
                }
            }
            Instructions::Do => enabled = true,
            Instructions::Dont => enabled = false,
        }
    }
    ret_val
}
