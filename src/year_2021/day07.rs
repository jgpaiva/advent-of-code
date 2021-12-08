#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day07");
    assert_eq!(part1(input.clone()), 37);
    assert_eq!(part2(input), 168);
}

pub fn part2(input: String) -> i32 {
    let input = input
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    //1,2,3,4,5 = 15
    //5+1+4+2+3 - (n+1)*(n // 2) + (n+1)//2
    //1,2,3,4,5,6 = 21
    //6+1+5+2+4+3+3 - (n+1)*(n // 2) + (n+1)//2 - (6+1)*3 + (6+1)/2
    (min..=max)
        .map(|p| {
            (
                p,
                input
                    .iter()
                    .map(|i| (i - p).abs())
                    .map(|n| n * (n + 1) / 2)
                    .sum::<i32>(),
            )
        })
        .min_by(|p1, p2| p1.1.cmp(&p2.1))
        .unwrap()
        .1
}
pub fn part1(input: String) -> i32 {
    let input = input
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..=max)
        .map(|p| (p, input.iter().map(|i| (i - p).abs()).sum::<i32>()))
        .min_by(|p1, p2| p1.1.cmp(&p2.1))
        .unwrap()
        .1
}
