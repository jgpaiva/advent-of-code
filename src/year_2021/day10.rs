#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day10");
    assert_eq!(part1(input.clone()), 26397);
    assert_eq!(part2(input), 288957);
}

pub fn part2(input: String) -> u64 {
    let mut res: Vec<_> = input
        .split_terminator('\n')
        .filter_map(|line| calculate_line(line).1)
        .collect();
    res.sort_unstable();
    res[res.len() / 2]
}

pub fn part1(input: String) -> u32 {
    input
        .split_terminator('\n')
        .map(|line| calculate_line(line).0)
        .sum()
}

pub fn calculate_line(line: &str) -> (u32, Option<u64>) {
    let mut open = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => open.push(c),
            ')' => {
                let o = open.pop().unwrap_or(' ');
                if o != '(' {
                    return (3, None);
                }
            }
            ']' => {
                let o = open.pop().unwrap_or(' ');
                if o != '[' {
                    return (57, None);
                }
            }
            '}' => {
                let o = open.pop().unwrap_or(' ');
                if o != '{' {
                    return (1197, None);
                }
            }
            '>' => {
                let o = open.pop().unwrap_or(' ');
                if o != '<' {
                    return (25137, None);
                }
            }
            v => unreachable!("{}", v),
        }
    }
    (0, Some(calculate_score(open)))
}

fn calculate_score(open: Vec<char>) -> u64 {
    open.iter().rev().fold(0, |accum, c| {
        accum * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                v => unreachable!("{}", v),
            }
    })
}
