use std::collections::HashSet;

#[test]
fn test_day4() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"#;
    assert_eq!(part1(input), 4512);
    assert_eq!(part2(input), 1924);
}

pub fn part2(lines: &str) -> i32 {
    let (numbers, mut matrices) = parse(lines);
    let mut winners = HashSet::new();
    for i in 0..matrices.len() {
        winners.insert(i);
    }
    let mut last;
    for i in numbers {
        for matrix_index in 0..matrices.len() {
            for line in &mut matrices[matrix_index] {
                for mut item in line {
                    if item.n == i {
                        item.f = true;
                    }
                }
            }
            let matrix = &matrices[matrix_index];
            for line in matrix {
                if line.iter().map(|x| x.f).all(|x| x) {
                    // found winner
                    winners.remove(&matrix_index);
                    last = matrix_index;
                    if winners.len() == 0 {
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
                    if winners.len() == 0 {
                        return calc_winner_day_4_winner(&matrices[last], i);
                    }
                }
            }
        }
    }
    unreachable!()
}

struct MatrixEntry {
    n: i32,
    f: bool,
}

pub fn part1(lines: &str) -> i32 {
    let (numbers, mut matrices) = parse(lines);

    for i in numbers {
        for matrix in 0..matrices.len() {
            for line in &mut matrices[matrix] {
                for mut item in line {
                    if item.n == i {
                        item.f = true;
                    }
                }
            }
            for line in &matrices[matrix] {
                if line.iter().map(|x| x.f).all(|x| x) {
                    // found winner
                    return calc_winner_day_4_winner(&matrices[matrix], i);
                }
            }
            for col_num in 0..matrices[matrix].len() {
                let col: Vec<&MatrixEntry> =
                    matrices[matrix].iter().map(|line| &line[col_num]).collect();
                if col.iter().map(|x| x.f).all(|x| x) {
                    // found winner
                    return calc_winner_day_4_winner(&matrices[matrix], i);
                }
            }
        }
    }
    unreachable!()
}

fn parse(lines: &str) -> (Vec<i32>, Vec<Vec<Vec<MatrixEntry>>>) {
    let lines: Vec<&str> = lines.split("\n").collect();
    let numbers = lines[0]
        .split(",")
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
    (numbers, matrices)
}

fn calc_winner_day_4_winner(matrix: &Vec<Vec<MatrixEntry>>, i: i32) -> i32 {
    let sum: i32 = matrix
        .into_iter()
        .flat_map(|line| line.into_iter().map(|x| if x.f { 0 } else { x.n }))
        .sum();
    sum * i
}
