use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day09");
    assert_eq!(part1(input.clone()), 15);
    assert_eq!(part2(input.clone()), 1134);
}

pub fn part2(input: String) -> i32 {
    let input: Vec<Vec<u8>> = input
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let mut basins: Vec<Vec<i32>> = vec![vec![0; input[0].len()]; input.len()];
    let mut basin_counter = 0;

    for (x, line) in input.iter().enumerate() {
        for (y, e) in line.iter().enumerate() {
            if basins[x][y] > 0 || *e == 9 {
                continue;
            }
            basin_counter += 1;
            let mut to_explore = vec![];
            to_explore.push((x, y));

            while let Some((x, y)) = to_explore.pop() {
                if basins[x][y] > 0 || input[x][y] == 9 {
                    continue;
                }
                basins[x][y] = basin_counter;

                if x > 0 {
                    to_explore.push((x - 1, y))
                }
                if y > 0 {
                    to_explore.push((x, y - 1))
                }
                if x < input.len() - 1 {
                    to_explore.push((x + 1, y))
                }
                if y < input[0].len() - 1 {
                    to_explore.push((x, y + 1))
                }
            }
        }
    }

    let mut basins_size = HashMap::new();
    for id in basins.iter().flat_map(|line| line.iter()) {
        basins_size.entry(*id).and_modify(|e| *e += 1).or_insert(1);
    }
    basins_size.remove(&0);
    let mut basins: Vec<i32> = basins_size.values().cloned().collect();
    basins.sort_unstable();
    basins.iter().rev().take(3).product::<i32>()
}

pub fn part1(input: String) -> u32 {
    let input: Vec<Vec<u8>> = input
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut ret: u32 = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, e) in line.iter().enumerate() {
            if (y == 0 || input[y - 1][x] > *e)
                && (x == 0 || input[y][x - 1] > *e)
                && (y == input.len() - 1 || input[y + 1][x] > *e)
                && (x == input[0].len() - 1 || input[y][x + 1] > *e)
            {
                ret += *e as u32 + 1;
            }
        }
    }

    ret
}
