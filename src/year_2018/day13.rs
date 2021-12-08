use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2018/test_day13");
    assert_eq!(part1(input), (7, 3));
}

pub fn part2(input: String) -> (usize, usize) {
    run(input).last().cloned().unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    t: ElementType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ElementType {
    Horizontal,
    Vertical,
    Intersect,
    SlashRight,
    SlashLeft,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Car {
    x: usize,
    y: usize,
    d: Direction,
    // next turn
    nt: Turn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

pub fn part1(input: String) -> (usize, usize) {
    run(input)[0]
}

pub fn run(input: String) -> Vec<(usize, usize)> {
    let input: Vec<Vec<char>> = input
        .split_terminator('\n')
        .map(|line| line.chars().collect())
        .collect();
    let board: HashMap<(usize, usize), Point> = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .flat_map(|(x, c)| {
                    let t = match *c {
                        '|' | '^' | 'v' => Some(ElementType::Vertical),
                        '-' | '>' | '<' => Some(ElementType::Horizontal),
                        '+' => Some(ElementType::Intersect),
                        '/' => Some(ElementType::SlashRight),
                        '\\' => Some(ElementType::SlashLeft),
                        _ => None,
                    };
                    t.map(|t| ((x, y), Point { x, y, t }))
                })
                .collect::<Vec<((usize, usize), Point)>>()
        })
        .collect();
    let mut cars: Vec<Car> = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .flat_map(|(x, c)| {
                    let direction = match *c {
                        '^' => Some(Direction::Up),
                        'v' => Some(Direction::Down),
                        '>' => Some(Direction::Right),
                        '<' => Some(Direction::Left),
                        _ => None,
                    };
                    direction.map(|d| Car {
                        x,
                        y,
                        d,
                        nt: Turn::Left,
                    })
                })
                .collect::<Vec<Car>>()
        })
        .collect();
    let max_iterations = 100000;
    let mut collisions = Vec::new();
    for _iter in 0..max_iterations {
        cars.sort_by(|c1, c2| (c1.y, c1.x).cmp(&(c2.y, c2.x)));
        let mut i = 0;
        let mut max = cars.len();
        while i < max {
            let car = cars[i];
            let (x, y, d, nt) = match (
                car.d,
                board
                    .get(&(car.x, car.y))
                    .unwrap_or_else(|| panic!("{:?}", car))
                    .t,
            ) {
                (Direction::Up, ElementType::Vertical) => (car.x, car.y - 1, car.d, car.nt),
                (Direction::Up, ElementType::Intersect) => match car.nt {
                    Turn::Left => (car.x - 1, car.y, Direction::Left, Turn::Straight),
                    Turn::Straight => (car.x, car.y - 1, car.d, Turn::Right),
                    Turn::Right => (car.x + 1, car.y, Direction::Right, Turn::Left),
                },
                (Direction::Up, ElementType::SlashRight) => {
                    (car.x + 1, car.y, Direction::Right, car.nt)
                }
                (Direction::Up, ElementType::SlashLeft) => {
                    (car.x - 1, car.y, Direction::Left, car.nt)
                }
                (Direction::Down, ElementType::Vertical) => (car.x, car.y + 1, car.d, car.nt),
                (Direction::Down, ElementType::Intersect) => match car.nt {
                    Turn::Left => (car.x + 1, car.y, Direction::Right, Turn::Straight),
                    Turn::Straight => (car.x, car.y + 1, car.d, Turn::Right),
                    Turn::Right => (car.x - 1, car.y, Direction::Left, Turn::Left),
                },
                (Direction::Down, ElementType::SlashRight) => {
                    (car.x - 1, car.y, Direction::Left, car.nt)
                }
                (Direction::Down, ElementType::SlashLeft) => {
                    (car.x + 1, car.y, Direction::Right, car.nt)
                }
                (Direction::Left, ElementType::Horizontal) => (car.x - 1, car.y, car.d, car.nt),
                (Direction::Left, ElementType::Intersect) => match car.nt {
                    Turn::Left => (car.x, car.y + 1, Direction::Down, Turn::Straight),
                    Turn::Straight => (car.x - 1, car.y, car.d, Turn::Right),
                    Turn::Right => (car.x, car.y - 1, Direction::Up, Turn::Left),
                },
                (Direction::Left, ElementType::SlashRight) => {
                    (car.x, car.y + 1, Direction::Down, car.nt)
                }
                (Direction::Left, ElementType::SlashLeft) => {
                    (car.x, car.y - 1, Direction::Up, car.nt)
                }
                (Direction::Right, ElementType::Horizontal) => (car.x + 1, car.y, car.d, car.nt),
                (Direction::Right, ElementType::Intersect) => match car.nt {
                    Turn::Left => (car.x, car.y - 1, Direction::Up, Turn::Straight),
                    Turn::Straight => (car.x + 1, car.y, car.d, Turn::Right),
                    Turn::Right => (car.x, car.y + 1, Direction::Down, Turn::Left),
                },
                (Direction::Right, ElementType::SlashRight) => {
                    (car.x, car.y - 1, Direction::Up, car.nt)
                }
                (Direction::Right, ElementType::SlashLeft) => {
                    (car.x, car.y + 1, Direction::Down, car.nt)
                }
                v => unreachable!("{:?}", v),
            };
            let new_car = Car { x, y, d, nt };

            if max == 1 {
                collisions.push((x, y));
                return collisions;
            }

            let mut collision = cars
                .iter()
                .enumerate()
                .map(|(i, c)| (i, (c.x, c.y)))
                .filter(|(_, (x2, y2))| (*x2, *y2) == (x, y));
            if let Some(collision) = collision.next() {
                if collision.0 < i {
                    cars.remove(i);
                    i -= 1;
                    cars.remove(collision.0);
                } else {
                    cars.remove(collision.0);
                    cars.remove(i);
                }
                max -= 2;
                collisions.push(collision.1);
            } else {
                cars[i] = new_car;
            }
            i += 1;
        }
    }
    unreachable!("Should not run out of iterations. Cars: {:?}", cars)
}
