use std::collections::HashSet;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day13");
    assert_eq!(part1(input.clone()), 17);
    assert_eq!(
        part2(input),
        r#"#####
#...#
#...#
#...#
#####"#
    );
}

#[allow(clippy::branches_sharing_code)]
pub fn part2(input: String) -> String {
    let (points, folds) = parse(input);
    let mut points: HashSet<(i32, i32)> = points.into_iter().collect();
    for fold in folds {
        points = if fold.0 == 'y' {
            let y = fold.1;
            points
                .into_iter()
                .map(|p| if p.1 > y { (p.0, y - (p.1 - y)) } else { p })
                .collect()
        } else {
            let x = fold.1;
            points
                .into_iter()
                .map(|p| if p.0 > x { (x - (p.0 - x), p.1) } else { p })
                .collect()
        };
    }
    let x_min = points.iter().map(|(x, _y)| x).cloned().min().unwrap();
    let y_min = points.iter().map(|(_x, y)| y).cloned().min().unwrap();
    let x_max = points.iter().map(|(x, _y)| x).cloned().max().unwrap();
    let y_max = points.iter().map(|(_x, y)| y).cloned().max().unwrap();
    let mut to_print: Vec<Vec<char>> = (y_min..y_max + 1)
        .map(|_| (x_min..x_max + 1).map(|_| '.').collect())
        .collect();
    for point in points {
        to_print[(point.1 - y_min) as usize][(point.0 - x_min) as usize] = '#';
    }
    to_print
        .into_iter()
        .map(|line| line.into_iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

#[allow(clippy::branches_sharing_code)]
pub fn part1(input: String) -> usize {
    let (points, folds) = parse(input);
    let first_fold = folds.first().cloned().unwrap();
    let points: HashSet<(i32, i32)> = if first_fold.0 == 'y' {
        let y = first_fold.1;
        points
            .into_iter()
            .map(|p| if p.1 > y { (p.0, y - (p.1 - y)) } else { p })
            .collect()
    } else {
        let x = first_fold.1;
        points
            .into_iter()
            .map(|p| if p.0 > x { (x - (p.0 - x), p.1) } else { p })
            .collect()
    };
    points.len()
}

#[allow(clippy::type_complexity)]
fn parse(input: String) -> (Vec<(i32, i32)>, Vec<(char, i32)>) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: Vec<(i32, i32)> = points
        .split_terminator('\n')
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<(char, i32)> = folds
        .split_terminator('\n')
        .map(|line| line.split_once("fold along ").unwrap().1)
        .map(|fold| fold.split_once('=').unwrap())
        .map(|(d, v)| (d.chars().next().unwrap(), v.parse().unwrap()))
        .collect();
    (points, folds)
}
