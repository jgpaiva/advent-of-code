#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day17");
    assert_eq!(part1(input.clone()), ((6, 9), 45));
    assert_eq!(part2(input), 112);
}

pub fn part2(input: String) -> usize {
    let area = parse(input);
    let velocities = calculate_bounds(area);
    run_sym(velocities, area).1
}

pub fn part1(input: String) -> ((i32, i32), i32) {
    let area = parse(input);
    let velocities = calculate_bounds(area);
    run_sym(velocities, area).0
}

#[derive(Clone, Copy, Debug)]
struct Bounds {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

fn run_sym(velocities: Bounds, area: Bounds) -> (((i32, i32), i32), usize) {
    let mut max = ((-1, -1), 0);
    let mut hits = 0;
    for xv_init in velocities.xmin..=velocities.xmax {
        for yv_init in velocities.ymin..=velocities.ymax {
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
                if p.0 >= area.xmin && p.0 <= area.xmax && p.1 >= area.ymin && p.1 <= area.ymax {
                    hit = true;
                    hits += 1;
                    break;
                }
                if p.0 > area.xmax || p.1 < area.ymin {
                    break; // overshoot
                }
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

fn calculate_bounds(area: Bounds) -> Bounds {
    // if x initial velocity is not enough, we stop before reaching the area
    let mut xv_min = 1;
    for i in 0..area.xmin {
        if (0..=i).sum::<i32>() >= area.xmin {
            xv_min = i;
            break;
        }
    }
    // if x initial velocity is too much, we overshoot
    let xv_max = area.xmax + 1;
    // if y initial velocity is not enough, we go bellow y_min before reaching the area
    let yv_min = area.ymin - 1;
    // if y initial velocity is too much, when coming down we blow past the area
    let yv_max = (-area.ymin) + 1;

    Bounds {
        xmin: xv_min,
        xmax: xv_max,
        ymin: yv_min,
        ymax: yv_max,
    }
}

fn parse(input: String) -> Bounds {
    let coords = input.split_once("area: x=").unwrap().1;
    let (xcoords, ycoords) = coords.split_once(", y=").unwrap();
    let (xmin, xmax): (i32, i32) = xcoords
        .split_once("..")
        .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
        .unwrap();
    let (ymin, ymax): (i32, i32) = ycoords
        .split_once("..")
        .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
        .unwrap();
    Bounds {
        xmin,
        xmax,
        ymin,
        ymax,
    }
}
