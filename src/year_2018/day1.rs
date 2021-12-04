use std::collections::HashSet;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2018/test_day1");
    assert_eq!(part2(input), "10");
}

pub fn part2(lines: Vec<String>) -> String {
    let a = lines
        .iter()
        .map(|line| {
            let a: Result<i32, _> = line.parse();
            a.expect("could not parse line as integer")
        })
        .cycle()
        .try_fold(
            (|| {
                let mut v = HashSet::new();
                v.insert(0);
                (v, 0)
            })(),
            |(mut accum, last), v| {
                let next = last + v;
                if accum.insert(next) {
                    Ok((accum, next))
                } else {
                    Err((accum, next))
                }
            },
        );
    if let Err((_accum, v)) = a {
        format!("{}", v)
    } else {
        unreachable!();
    }
}
