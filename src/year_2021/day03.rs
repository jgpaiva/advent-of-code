#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2021/test_day3");
    assert_eq!(part1(&input), 198);
    assert_eq!(part2(&input), 230);
}

pub fn part2(lines: &[String]) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let oxygen: Vec<Vec<char>> = (0..lines[0].len()).fold(lines.clone(), |lines, i| {
        if lines.len() == 1 {
            return lines;
        };
        let zeroes = lines.iter().map(|x| x[i]).filter(|x| *x == '0').count();
        let most_common = if zeroes > lines.len() / 2 { '0' } else { '1' };
        lines
            .into_iter()
            .filter(|line| line[i] == most_common)
            .collect::<Vec<Vec<char>>>()
    });
    let co2: Vec<Vec<char>> = (0..lines[0].len()).fold(lines, |lines, i| {
        if lines.len() == 1 {
            return lines;
        };
        let zeroes = lines.iter().map(|x| x[i]).filter(|x| *x == '0').count();
        let most_common = if zeroes > lines.len() / 2 { '0' } else { '1' };
        lines
            .into_iter()
            .filter(|line| line[i] != most_common)
            .collect::<Vec<Vec<char>>>()
    });

    let oxygen = i32::from_str_radix(oxygen[0].iter().collect::<String>().as_str(), 2).unwrap();
    let co2 = i32::from_str_radix(co2[0].iter().collect::<String>().as_str(), 2).unwrap();
    oxygen * co2
}

pub fn part1(lines: &[String]) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let gamma: String = (0..lines[0].len())
        .map(|i| {
            let ones = lines.iter().map(|x| x[i]).filter(|x| *x == '1').count();
            if ones > lines.len() / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    let epsilon: String = gamma
        .chars()
        .map(|x| if x == '1' { '0' } else { '1' })
        .collect();

    let gamma = i32::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = i32::from_str_radix(epsilon.as_str(), 2).unwrap();
    gamma * epsilon
}
