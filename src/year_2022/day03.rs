use std::collections::HashSet;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 157);
    assert_eq!(part2(&input), 70);
}

pub fn part2(lines: &str) -> i32 {
    let lines: Vec<&str> = lines.split_terminator('\n').collect();
    let mut badges = vec![];
    for group in lines.chunks_exact(3) {
        let p1 = group[0].as_bytes();
        let p2 = group[1].as_bytes();
        let p3 = group[2].as_bytes();
        let p1: HashSet<u8> = p1.iter().cloned().collect();
        let p2: HashSet<u8> = p2.iter().cloned().collect();
        let p3: HashSet<u8> = p3.iter().cloned().collect();
        let intersect: HashSet<u8> = p1.intersection(&p2).cloned().collect();
        let mut intersect = intersect.intersection(&p3);
        let v = intersect.next().unwrap();
        assert_eq!(intersect.next(), None);
        badges.push(*v);
    }

    badges
        .into_iter()
        .map(convert_letter)
        .map(|x| x as i32)
        .sum()
}

#[allow(clippy::char_lit_as_u8)]
fn convert_letter(letter: u8) -> u8 {
    if letter >= ('a' as u8) && letter <= ('z' as u8) {
        letter - ('a' as u8) + 1
    } else if letter >= ('A' as u8) && letter <= ('Z' as u8) {
        letter - ('A' as u8) + 27
    } else {
        unreachable!("should only be a-z or A-Z")
    }
}

pub fn part1(lines: &str) -> i32 {
    let lines = lines.split_terminator('\n');
    let mut diff_letters = vec![];
    for line in lines {
        let p1 = &line.as_bytes()[(line.len() / 2)..];
        let p2 = &line.as_bytes()[..(line.len() / 2)];
        assert_eq!(p1.len(), p2.len());
        let p1: HashSet<u8> = p1.iter().cloned().collect();
        let p2: HashSet<u8> = p2.iter().cloned().collect();
        let mut intersect = p2.intersection(&p1);
        let v = intersect.next().unwrap();
        assert_eq!(intersect.next(), None);
        diff_letters.push(*v);
    }
    diff_letters
        .into_iter()
        .map(convert_letter)
        .map(|x| x as i32)
        .sum()
}
