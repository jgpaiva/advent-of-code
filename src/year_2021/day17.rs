#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day17");
    assert_eq!(part1(input.clone()), ((6, 9), 45));
    assert_eq!(part2(input.clone()), 112);
}

pub fn part2(input: String) -> usize {
    let (x_min, x_max, y_min, y_max) = parse(input);

    let (xv_min, xv_max, yv_min, yv_max) = calculate_bounds(x_min, x_max, y_min);

    run_sym(xv_min, xv_max, yv_min, yv_max, x_min, x_max, y_min, y_max).1
}

pub fn part1(input: String) -> ((i32, i32), i32) {
    let (x_min, x_max, y_min, y_max) = parse(input);

    let (xv_min, xv_max, yv_min, yv_max) = calculate_bounds(x_min, x_max, y_min);

    run_sym(xv_min, xv_max, yv_min, yv_max, x_min, x_max, y_min, y_max).0
}

#[allow(clippy::too_many_arguments)]
fn run_sym(
    xv_min: i32,
    xv_max: i32,
    yv_min: i32,
    yv_max: i32,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
) -> (((i32, i32), i32), usize) {
    let mut max = ((-1, -1), 0);
    let mut hits = 0;
    for xv_init in xv_min..=xv_max {
        for yv_init in yv_min..=yv_max {
            let mut xv = xv_init;
            let mut yv = yv_init;
            let mut p = (0, 0);

            let mut hit = false;
            let mut max_y = 0;
            for i in 0..1000 {
                assert!(i != 999);
                if p.1 > max_y {
                    max_y = p.1;
                }
                if p.0 >= x_min && p.0 <= x_max && p.1 >= y_min && p.1 <= y_max {
                    hit = true;
                    hits += 1;
                    break;
                }
                if p.0 > x_max || p.1 < y_min {
                    break; // overshoot
                }
                //dbg!(&p);
                p = (p.0 + xv, p.1 + yv);
                yv -= 1;
                xv = if xv == 0 { 0 } else { xv - 1 };
            }
            if hit && max_y > max.1 {
                max = ((xv_init, yv_init), max_y);
            }
        }
    }
    (max, hits)
}

fn calculate_bounds(x_min: i32, x_max: i32, y_min: i32) -> (i32, i32, i32, i32) {
    // if x initial velocity is not enough, we stop before reaching the area
    let mut xv_min = 1;
    for i in 0..x_min {
        if (0..=i).sum::<i32>() >= x_min {
            xv_min = i;
            break;
        }
    }
    let xv_min = xv_min;
    // if x initial velocity is too much, we overshoot
    let xv_max = x_max + 1;
    // if y initial velocity is not enough, we go bellow y_min before reaching the area
    let yv_min = y_min - 1;
    // if y initial velocity is too much, when coming down we blow past the area
    let yv_max = (-y_min) + 1;

    (xv_min, xv_max, yv_min, yv_max)
}

fn parse(input: String) -> (i32, i32, i32, i32) {
    let coords = input.split_once("area: x=").unwrap().1;
    let (xcoords, ycoords) = coords.split_once(", y=").unwrap();
    let (x_min, x_max): (i32, i32) = xcoords
        .split_once("..")
        .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
        .unwrap();
    let (y_min, y_max): (i32, i32) = ycoords
        .split_once("..")
        .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
        .unwrap();
    (x_min, x_max, y_min, y_max)
}
