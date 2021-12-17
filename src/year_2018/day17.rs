#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let _input = utils::read_file("2018/test_day17");
    //assert_eq!(part1(input.clone()), 57);
}

pub fn part2(_input: String) -> usize {
    todo!()
}

pub fn part1(input: String) -> u32 {
    let veins: Vec<&str> = input.split_terminator('\n').collect();
    let horizontals: Vec<(u32, (u32, u32))> = veins
        .iter()
        .filter(|l| l.starts_with('x'))
        .cloned()
        .map(|line| {
            let (x, y) = line.split_once(", y=").unwrap();
            let x = x.split_once("x=").unwrap().1.parse().unwrap();
            let (ymin, ymax) = y.split_once("..").unwrap();
            (x, (ymin.parse().unwrap(), ymax.parse().unwrap()))
        })
        .collect();
    let verticals: Vec<(u32, (u32, u32))> = veins
        .iter()
        .filter(|l| l.starts_with('y'))
        .cloned()
        .map(|line| {
            let (y, x) = line.split_once(", x=").unwrap();
            let y = y.split_once("y=").unwrap().1.parse().unwrap();
            let (xmin, xmax) = x.split_once("..").unwrap();
            (y, (xmin.parse().unwrap(), xmax.parse().unwrap()))
        })
        .collect();

    let xmin = horizontals.iter().map(|h| h.0).min().unwrap();
    let xmax = horizontals.iter().map(|h| h.0).max().unwrap();
    // let ymin = verticals.iter().map(|v| v.0).min().unwrap();
    let ymax = verticals.iter().map(|v| v.0).max().unwrap();

    let fountain = (500, 0);
    let mut map: Vec<Vec<char>> = (0..=ymax + 1)
        .map(|_| (xmin - 1..=xmax + 1).map(|_| '.').collect())
        .collect();
    map[fountain.1][fountain.0 - xmin as usize + 1] = '+';
    for line in horizontals {
        let x = line.0;
        for y in line.1 .0..=line.1 .1 {
            map[y as usize][x as usize - xmin as usize + 1] = '#';
        }
    }
    for line in verticals {
        let y = line.0;
        for x in line.1 .0..=line.1 .1 {
            map[y as usize][x as usize - xmin as usize + 1] = '#';
        }
    }
    for line in map {
        println!("{}", line.into_iter().collect::<String>())
    }
    todo!()
}
