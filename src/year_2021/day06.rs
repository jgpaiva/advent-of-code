#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day06");
    assert_eq!(part1(input.clone(), 18), 26);
    assert_eq!(part1(input.clone(), 80), 5934);
    assert_eq!(part2(input, 256), 26984457539);
}

pub fn part2(input: String, days: i32) -> u64 {
    part1(input, days)
}

pub fn part1(input: String, days: i32) -> u64 {
    let input: Vec<u64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let mut fish_per_day = vec![0; 9];
    for i in input {
        fish_per_day[i as usize] += 1;
    }
    for _ in 1..=days {
        let previous_day = fish_per_day.clone();
        fish_per_day[..(9 - 1)].clone_from_slice(&previous_day[1..9]);
        fish_per_day[6] += previous_day[0];
        fish_per_day[8] = previous_day[0];
    }
    fish_per_day.iter().sum()
}
