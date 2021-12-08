use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day08");
    assert_eq!(part1(input.clone()), 26);
    assert_eq!(
        create_decoder_map(&[
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"
        ]),
        [
            ("ab", 1),
            ("abef", 4),
            ("abd", 7),
            ("abcdefg", 8),
            ("abcdf", 3),
            ("bcdef", 5),
            ("acdfg", 2),
            ("abcdef", 9),
            ("abcdeg", 0),
            ("bcdefg", 6),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    );
    assert_eq!(part2(input), 61229);
}

pub fn part2(input: String) -> i32 {
    let input: Vec<(Vec<&str>, Vec<&str>)> = input
        .split_terminator('\n')
        .map(|line| line.split_once("| ").unwrap())
        .map(|(part1, part2)| {
            (
                part1.split(' ').collect(),
                part2.split_terminator(' ').collect(),
            )
        })
        .collect();
    input
        .iter()
        .map(|(patterns, codes)| {
            let decoder_map = create_decoder_map(patterns);
            let codes: Vec<String> = codes
                .iter()
                .map(|c| {
                    let mut c: Vec<char> = c.chars().collect();
                    c.sort_unstable();
                    c.into_iter().collect()
                })
                .collect();
            let codes: String = codes.iter().map(|c| decoder_map[c].to_string()).collect();
            codes.parse::<i32>().unwrap()
        })
        .sum()
}

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

pub fn create_decoder_map(patterns: &[&str]) -> HashMap<String, i32> {
    let mut decoder = HashMap::<i32, String>::new();
    let mut patterns: Vec<String> = patterns
        .iter()
        .map(|p| {
            let mut p: Vec<char> = p.chars().collect();
            p.sort_unstable();
            p.into_iter().collect()
        })
        .collect();
    // 1 - has 2 segments
    decode_digit(&mut patterns, |p: &String| p.len() == 2, &mut decoder, 1);
    // 4 - has 4 segments
    decode_digit(&mut patterns, |p: &String| p.len() == 4, &mut decoder, 4);
    // 7 - has 3 segments
    decode_digit(&mut patterns, |p: &String| p.len() == 3, &mut decoder, 7);
    // 8 - has 7 segments
    decode_digit(&mut patterns, |p: &String| p.len() == 7, &mut decoder, 8);
    // 9 - has 6 segments, includes all segments from 4
    let four = decoder[&4].clone();
    decode_digit(
        &mut patterns,
        |p: &String| p.len() == 6 && four.chars().all(|x| p.contains(x)),
        &mut decoder,
        9,
    );
    let one = decoder[&1].clone();
    // 0 - has 6 segments, includes all segments from 1
    decode_digit(
        &mut patterns,
        |p: &String| p.len() == 6 && one.chars().all(|x| p.contains(x)),
        &mut decoder,
        0,
    );
    // 6 - has 6 segments
    decode_digit(&mut patterns, |p: &String| p.len() == 6, &mut decoder, 6);
    // 3 - (has 5 segments,) includes all segments from 1
    decode_digit(
        &mut patterns,
        |p: &String| one.chars().all(|x| p.contains(x)),
        &mut decoder,
        3,
    );
    let nine = decoder[&9].clone();
    // 2 - (has 5 segments,) includes 4 segments from 9
    decode_digit(
        &mut patterns,
        |p: &String| nine.chars().filter(|x| p.contains(*x)).count() == 4,
        &mut decoder,
        2,
    );
    // 5 - (has 5 segments)
    decode_digit(&mut patterns, |_p: &String| true, &mut decoder, 5);

    decoder
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<String, i32>>()
}

fn decode_digit<F: Fn(&String) -> bool>(
    patterns: &mut Vec<String>,
    f: F,
    decoder: &mut HashMap<i32, String>,
    digit: i32,
) {
    let i = patterns
        .iter()
        .enumerate()
        .find(|(_i, v)| f(v))
        .map(|(i, v)| (i, v.clone()))
        .unwrap();
    patterns.remove(i.0);
    decoder.insert(digit, i.1);
}

pub fn part1(input: String) -> usize {
    let input: Vec<(Vec<&str>, Vec<&str>)> = input
        .split_terminator('\n')
        .map(|line| line.split_once('|').unwrap())
        .map(|(part1, part2)| (part1.split(' ').collect(), part2.split(' ').collect()))
        .collect();
    input
        .iter()
        .flat_map(|(_pattern, code)| code.iter().map(|i| i.len()))
        .filter(|len| [2, 3, 4, 7].contains(len))
        .count()
}
