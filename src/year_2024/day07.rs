#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 3749);
    assert_eq!(part2(&input), 11387);
}

pub fn part1(input: &str) -> i64 {
    let input = parse_input(input);
    let mut retval = 0;
    for (target, values) in input {
        if check_validity(target, &values) {
            retval += target;
        }
    }
    retval
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    let input: Vec<_> = input
        .split("\n")
        .map(|line| {
            let (target, values) = line.split_once(": ").unwrap();
            let target: i64 = target.parse().unwrap();
            let values: Vec<i64> = values.split(" ").map(|v| v.parse().unwrap()).collect();
            (target, values)
        })
        .collect();
    input
}

fn check_validity(target: i64, values: &[i64]) -> bool {
    if values.len() == 2 {
        values[0] * values[1] == target || values[0] + values[1] == target
    } else {
        (target % values[values.len() - 1] == 0
            && check_validity(
                target / values[values.len() - 1],
                &values[0..values.len() - 1],
            ))
            || (target - values[values.len() - 1] >= 0
                && check_validity(
                    target - values[values.len() - 1],
                    &values[0..values.len() - 1],
                ))
    }
}

pub fn part2(input: &str) -> i64 {
    let input = parse_input(input);
    let mut retval = 0;
    for (target, values) in input {
        if check_validity2(target, &values) {
            retval += target;
        }
    }
    retval
}

fn concat_numbers(a: i64, b: i64) -> i64 {
    a.checked_mul(10i64.checked_pow(b.ilog10() + 1).unwrap())
        .unwrap()
        + b
}

#[test]
fn test_number_ends_with() {
    assert_eq!(number_ends_with(123, 2), None);
    assert_eq!(number_ends_with(123, 3), Some(12));
    assert_eq!(number_ends_with(3, 123), None);
    assert_eq!(number_ends_with(1234, 34), Some(12));
}

fn number_ends_with(target: i64, a: i64) -> Option<i64> {
    let mask = 10i64.checked_pow(a.ilog10() + 1).unwrap();
    if target % mask == a {
        Some(target / mask)
    } else {
        None
    }
}

fn check_validity2(target: i64, values: &[i64]) -> bool {
    if values.len() == 2 {
        values[0] * values[1] == target
            || values[0] + values[1] == target
            || target == concat_numbers(values[0], values[1])
    } else {
        (target % values[values.len() - 1] == 0
            && check_validity2(
                target / values[values.len() - 1],
                &values[0..values.len() - 1],
            ))
            || (target - values[values.len() - 1] >= 0
                && check_validity2(
                    target - values[values.len() - 1],
                    &values[0..values.len() - 1],
                ))
            || {
                let target = number_ends_with(target, values[values.len() - 1]);
                match target {
                    Some(target) => check_validity2(target, &values[0..values.len() - 1]),
                    None => false,
                }
            }
    }
}

#[test]
fn test_case() {
    assert!(check_validity2(7290, &[6, 8, 6, 15]));
}
