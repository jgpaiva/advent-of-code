use regex;
use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 240);
    assert_eq!(part2(&input), 281);
}

pub fn part1(input: &str) -> i32 {
    let input = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    input
        .into_iter()
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|s| format!("{}{}", s[0], s[s.len() - 1]))
        .map(|s| s.parse::<i32>().expect("oops"))
        .sum::<i32>()
}

pub fn part2(input: &str) -> usize {
    let input = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut r = String::new();
    let mut translator = HashMap::new();

    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits: Vec<String> = (1..10).map(|n| n.to_string()).collect();

    for (i, n) in numbers.iter().enumerate() {
        r.push_str(n);
        r.push('|');
        translator.insert(n.to_string(), i + 1);
    }
    for (i, n) in digits.iter().enumerate() {
        r.push_str(n);
        r.push('|');
        translator.insert(n.to_string(), i + 1);
    }

    /* turns out this was just me overcomplicating
    for (i, n1) in numbers.iter().enumerate() {
        for n2 in &numbers {
            if n1.chars().last().unwrap() == n2.chars().next().unwrap() {
                let v: String = n1.chars().chain(n2.chars().skip(1)).collect();
                r.push_str(v.as_str());
                r.push('|');
                translator.insert(v, i + 1);
            }
        }
    }
     */
    // remove extra regex 'or'
    r.pop();

    let re = regex::Regex::new(r.as_str()).unwrap();
    let rr = r.chars().rev().collect::<String>();
    let er = regex::Regex::new(&rr).unwrap();
    input
        .iter()
        .map(|s| {
            let c = re.captures(s).unwrap();
            let s1 = &c[0];
            let ss = s.chars().rev().collect::<String>();
            let c2 = er.captures(&ss).unwrap();
            let s2: String = c2[0].chars().rev().collect();
            translator.get(s1).expect(s1) * 10 + translator.get(s2.as_str()).expect(&s2)
        })
        .sum::<usize>()
}
