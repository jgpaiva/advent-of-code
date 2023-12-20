use std::collections::HashSet;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 4361);
    assert_eq!(part2(&input), 467835);
}

pub fn part1(input: &str) -> u32 {
    let input = input
        .split('\n')
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut curr_num = None;
    let mut next_to_symbol = false;
    let mut accum = 0;
    let mut next_to_symbol_map: Vec<Vec<bool>> = (0..input.len() + 2)
        .map(|_| vec![false; input[0].len() + 2])
        .collect();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '.' {
                continue;
            } else if !c.is_alphanumeric() {
                for line in &mut next_to_symbol_map[i..=i + 2] {
                    for item in &mut line[j..=j + 2] {
                        *item = true;
                    }
                }
            }
        }
    }
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                curr_num = Some(curr_num.unwrap_or_default() * 10 + d);
                if next_to_symbol_map[i + 1][j + 1] {
                    next_to_symbol = true;
                }
            } else {
                save_accum(&mut accum, &mut curr_num, &mut next_to_symbol);
            }
        }
        save_accum(&mut accum, &mut curr_num, &mut next_to_symbol);
    }
    accum
}

fn save_accum(accum: &mut u32, curr_num: &mut Option<u32>, next_to_symbol: &mut bool) {
    if *next_to_symbol {
        *accum += curr_num.unwrap_or_default();
    }
    *next_to_symbol = false;
    *curr_num = None;
}

pub fn part2(input: &str) -> u32 {
    let input = input
        .split('\n')
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut curr_num = None;
    let mut curr_num_size = 0;
    let mut id_counter = 0;
    let mut next_to_symbol_map: Vec<Vec<Option<(u32, u64)>>> = (0..input.len() + 2)
        .map(|_| vec![None; input[0].len() + 2])
        .collect();

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                curr_num = Some(curr_num.unwrap_or_default() * 10 + d);
                curr_num_size += 1;
            } else {
                save_num(
                    &mut next_to_symbol_map,
                    &mut curr_num,
                    &mut curr_num_size,
                    i,
                    j,
                    &mut id_counter,
                );
            }
        }
        save_num(
            &mut next_to_symbol_map,
            &mut curr_num,
            &mut curr_num_size,
            i,
            input[0].len(),
            &mut id_counter,
        );
    }

    let mut accum = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '*' {
                let mut numbers = HashSet::new();
                for line in &next_to_symbol_map[i..=i + 2] {
                    for item in &line[j..=j + 2] {
                        if let Some(k) = &item {
                            numbers.insert(*k);
                        }
                    }
                }
                if numbers.len() == 2 {
                    let numbers = Vec::from_iter(numbers.iter());
                    accum += numbers[0].0 * numbers[1].0
                }
            }
        }
    }
    accum
}

fn save_num(
    number_map: &mut [Vec<Option<(u32, u64)>>],
    curr_num: &mut Option<u32>,
    curr_num_size: &mut usize,
    i: usize,
    j: usize,
    id_counter: &mut u64,
) {
    let Some(num) = *curr_num else {
        return;
    };
    for j in j - *curr_num_size..j {
        number_map[i + 1][j + 1] = Some((num, *id_counter));
    }
    *curr_num_size = 0;
    *curr_num = None;
    *id_counter += 1;
}
