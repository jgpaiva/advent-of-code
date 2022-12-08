#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 21);
    assert_eq!(part2(&input), 8);
}

pub fn part2(lines: &str) -> usize {
    let lines: Vec<Vec<u8>> = lines
        .split_terminator('\n')
        .map(|line| line.as_bytes().iter().map(|b| *b - b'0').collect())
        .collect();
    let mut max_score = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let el = lines[i][j];
            let up = if i > 0 {
                let v = lines[0..i]
                    .iter()
                    .rev()
                    .take_while(|line| line[j] < el)
                    .count();
                i.min(v + 1)
            } else {
                0
            };
            let down = if i + 1 < lines.len() {
                let v = lines[i + 1..lines.len()]
                    .iter()
                    .take_while(|line| line[j] < el)
                    .count();
                (lines.len() - i - 1).min(v + 1)
            } else {
                0
            };
            let line = &lines[i];
            let left = if j > 0 {
                let v = line[0..j].iter().rev().take_while(|v| **v < el).count();
                j.min(v + 1)
            } else {
                0
            };
            let right = if j + 1 < line.len() {
                let v = line[j + 1..line.len()]
                    .iter()
                    .take_while(|v| **v < el)
                    .count();
                (line.len() - j - 1).min(v + 1)
            } else {
                0
            };
            let score = up * down * left * right;
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

pub fn part1(lines: &str) -> i32 {
    let lines: Vec<Vec<u8>> = lines
        .split_terminator('\n')
        .map(|line| line.as_bytes().iter().map(|b| *b - b'0').collect())
        .collect();
    let mut c = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[i].len() - 1 {
            let el = lines[i][j];
            let up = lines[0..i].iter().all(|line| line[j] < el);
            let down = lines[i + 1..lines.len()].iter().all(|line| line[j] < el);
            let left = lines[i][0..j].iter().all(|v| *v < el);
            let right = lines[i][j + 1..lines[i].len()].iter().all(|v| *v < el);
            if up || down || left || right {
                c += 1;
            }
        }
    }
    c + (lines.len() as i32 * 2) + (lines[0].len() as i32 * 2) - 4
}
