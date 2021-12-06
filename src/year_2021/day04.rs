#[cfg(test)]
use crate::utils;
use std::collections::HashSet;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day4");
    assert_eq!(part1(input.clone()), 4512);
    assert_eq!(part2(input), 1924);
}

pub fn part2(lines: String) -> i32 {
    let (numbers, mut matrices) = parse(lines);
    let mut winners = HashSet::new();
    for i in 0..matrices.len() {
        winners.insert(i);
    }
    let mut last;
    for i in numbers {
        for (matrix_index, matrix) in matrices.iter_mut().enumerate() {
            for line in matrix.iter_mut() {
                for mut item in line {
                    if item.n == i {
                        item.f = true;
                    }
                }
            }
            for line in matrix.iter() {
                if line.iter().map(|x| x.f).all(|x| x) {
                    // found winner
                    winners.remove(&matrix_index);
                    last = matrix_index;
                    if winners.is_empty() {
                        return calc_winner_day_4_winner(&matrices[last], i);
                    }
                }
            }
            for col_num in 0..matrix.len() {
                let col: Vec<&MatrixEntry> = matrix.iter().map(|line| &line[col_num]).collect();
                if col.iter().map(|x| x.f).all(|x| x) {
                    // found winner
                    winners.remove(&matrix_index);
                    last = matrix_index;
                    if winners.is_empty() {
                        return calc_winner_day_4_winner(&matrices[last], i);
                    }
                }
            }
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct MatrixEntry {
    n: i32,
    f: bool,
}

pub fn part1(lines: String) -> i32 {
    let (numbers, mut matrices) = parse(lines);

    for i in numbers {
        for matrix in matrices.iter_mut() {
            for line in matrix.iter_mut() {
                for mut item in line {
                    if item.n == i {
                        item.f = true;
                    }
                }
            }
            for line in matrix.iter() {
                if line.iter().map(|x| x.f).all(|x| x) {
                    // found winner
                    return calc_winner_day_4_winner(matrix, i);
                }
            }
            for col_num in 0..matrix.len() {
                let col: Vec<&MatrixEntry> = matrix.iter().map(|line| &line[col_num]).collect();
                if col.iter().map(|x| x.f).all(|x| x) {
                    // found winner
                    return calc_winner_day_4_winner(matrix, i);
                }
            }
        }
    }
    unreachable!()
}

fn parse(lines: String) -> (Vec<i32>, Vec<Vec<Vec<MatrixEntry>>>) {
    let lines: Vec<&str> = lines.split('\n').collect();
    let numbers = lines[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    let mut matrices: Vec<Vec<Vec<MatrixEntry>>> = vec![];
    let mut accum: Vec<Vec<MatrixEntry>> = vec![];
    for line in lines.into_iter().skip(2).collect::<Vec<&str>>() {
        if line.is_empty() {
            matrices.push(accum);
            accum = vec![];
            continue;
        }
        accum.push(
            line.split_whitespace()
                .map(|n| MatrixEntry {
                    n: n.parse().unwrap(),
                    f: false,
                })
                .collect(),
        );
    }
    if !accum.is_empty() {
        matrices.push(accum);
    }
    (numbers, matrices)
}

fn calc_winner_day_4_winner(matrix: &[Vec<MatrixEntry>], i: i32) -> i32 {
    let sum: i32 = matrix
        .iter()
        .flat_map(|line| line.iter().map(|x| if x.f { 0 } else { x.n }))
        .sum();
    sum * i
}
