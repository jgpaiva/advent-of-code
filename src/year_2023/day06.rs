use core::ops::RangeInclusive;
// this needs refactoring to fit into the same format as others, since it was done on my phone while traveling

//use regex; // 1.10.2
//use std::collections::HashMap;
//use std::collections::HashSet;

fn main() {
    let lines = MINE.split("\n").collect::<Vec<&str>>();
    println!("{:?}", part1(&lines));
    let lines = lines.into_iter().map(|l| l.replace(" ","")).collect::<Vec<String>>();
    let lines_str = lines.iter().map(|l|l.as_str()).collect::<Vec<&str>>();
     println!("{:?}", part1(&lines_str));
}

fn part1(input: &[&str]) -> i64 {
    let input = dbg!(parse_input(input));
    let mut total_wins= 1;
    for (t,d) in input {
    let mut wins = 0;
        for i in (1..t){
            if (t-i)*i >d {
                wins+=1;
            }
        }
        total_wins*=wins;
    }
    total_wins
}

fn parse_input(input: &[&str]) -> Vec<(u64, u64)> {
    let parse_line = |line: &str| {
        line.split_once(":")
            .unwrap()
            .1
            .split(" ")
            .map(|i| i.parse::<u64>().ok())
            .flatten()
            .collect::<Vec<_>>()
    };

    let t = parse_line(input[0]);
    let d = parse_line(input[1]);
    t.into_iter().zip(d.into_iter()).collect::<Vec<_>>()
}

fn part2(seeds: Vec<i64>, input: &[&str]) -> i64 {
    todo!()
}

const V1: &str = "Time:      7  15   30
Distance:  9  40  200";

const V2: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

const MINE: &str = "Time:        38     94     79     70
Distance:   241   1549   1074   1091";
