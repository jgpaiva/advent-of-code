use std::collections::HashSet;

#[cfg(test)]
use crate::utils;

#[test]
fn test_part1() {
    assert_eq!(
        part1(utils::to_vec(&[
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ])),
        "12"
    );
}

pub fn part1(lines: Vec<String>) -> String {
    let (two_letter, three_letter) =
        lines
            .iter()
            .fold((0, 0), |(two_letter, three_letter), line| {
                let (v1, v2) = aux(line);
                (two_letter + v1, three_letter + v2)
            });
    format!("{}", two_letter * three_letter)
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(utils::to_vec(&[
            "aaaa", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ])),
        "fgij"
    );
    assert_eq!(
        part2(utils::to_vec(&[
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ])),
        "fgij"
    );
}

pub fn part2(lines: Vec<String>) -> String {
    let mut h = HashSet::new();
    for line in lines {
        let line = line.chars().collect::<Vec<_>>();
        let mut line_set = HashSet::new();
        for i in 0..line.len() {
            let line: String = line
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, c)| c)
                .collect();
            line_set.insert(line);
        }
        for line in line_set {
            if !h.insert(line.to_owned()) {
                return line;
            }
        }
    }
    "".to_string()
}

#[test]
fn test_aux() {
    assert_eq!(aux(&"abcdef".to_string()), (0, 0));
    assert_eq!(aux(&"bababc".to_string()), (1, 1));
    assert_eq!(aux(&"abbcde".to_string()), (1, 0));
    assert_eq!(aux(&"abcccd".to_string()), (0, 1));
    assert_eq!(aux(&"aabcdd".to_string()), (1, 0));
    assert_eq!(aux(&"abcdee".to_string()), (1, 0));
    assert_eq!(aux(&"ababab".to_string()), (0, 1));
    assert_eq!(aux(&"aaa".to_string()), (0, 1));
    assert_eq!(aux(&"aa".to_string()), (1, 0));
    assert_eq!(aux(&"aaaa".to_string()), (0, 0));
    assert_eq!(aux(&"caaaabb".to_string()), (1, 0));
}

fn aux(s: &str) -> (i32, i32) {
    let mut sorted = s.chars().collect::<Vec<_>>();
    sorted.sort_unstable();
    let mut two_letter = 0;
    let mut three_letter = 0;
    let mut i = 0;
    while i < sorted.len() {
        #[allow(clippy::if_same_then_else)]
        if sorted.len() == i + 1 {
            //done, nothing else to check
            i += 1;
        } else if sorted[i + 1] != sorted[i] {
            // move forward, next is different char
            i += 1;
        } else if sorted.len() == i + 2 {
            // there's only one more, and it's the same
            two_letter = 1;
            i += 2;
        } else if sorted[i + 2] != sorted[i] {
            // there's two equal in a row, and then a different one
            two_letter = 1;
            i += 2;
        } else if sorted.len() == i + 3 {
            // there's three equal in a row and then ends
            three_letter = 1;
            i += 3;
        } else if sorted[i + 3] != sorted[i] {
            // there's three in a row, and then a different one
            three_letter = 1;
            i += 3;
        } else {
            // at least 4 in a row of the same, consume everything
            let mut j = i + 1;
            while j < sorted.len() && sorted[j] == sorted[i] {
                j += 1;
            }
            i = j;
        }
    }
    (two_letter, three_letter)
}
