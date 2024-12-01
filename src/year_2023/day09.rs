use itertools::Itertools;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 114);
    assert_eq!(part2(&input), 2);
}

pub fn part1(input: &str) -> i32 {
    solve(input, true)
}
pub fn part2(input: &str) -> i32 {
    solve(input, false)
}

pub fn solve(input: &str, last: bool) -> i32 {
    let input = input.split('\n').collect::<Vec<&str>>();
    input
        .into_iter()
        .map(|input_line| {
            let mut lines = vec![input_line
                .split(' ')
                .map(|n| n.parse())
                .collect::<Result<Vec<i32>, _>>()
                .unwrap()];
            for i in 0_usize.. {
                let new_line: Vec<_> = lines[i]
                    .iter()
                    .tuple_windows()
                    .map(|(v1, v2)| v2 - v1)
                    .collect();
                lines.push(new_line);
                let new_line = &lines[lines.len() - 1];
                if new_line.iter().all(|v| *v == 0) {
                    break;
                }
            }
            let mut new_column = vec![0];
            for i in (1..lines.len()).rev() {
                let next_line = &lines[i - 1];
                let curr_value = &new_column[lines.len() - i - 1];
                if last {
                    let new_v = next_line[next_line.len() - 1] + curr_value;
                    new_column.push(new_v);
                } else {
                    let new_v = next_line[0] - curr_value;
                    new_column.push(new_v);
                }
            }
            // dbg!(&lines);
            new_column[new_column.len() - 1]
        })
        .sum()
}
