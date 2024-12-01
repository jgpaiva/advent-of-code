#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 8);
    //assert_eq!(part2(&input), 2);
}

pub fn convert(input: &str) -> String {
    input
        .replace('|', "│")
        .replace('-', "─")
        .replace('L', "└")
        .replace('J', "┘")
        .replace('7', "┐")
        .replace('F', "┌")
        .replace('.', " ")
}

type Map = [Vec<char>];
type Point = (usize, usize);

pub fn part1(input: &str) -> u32 {
    let input = convert(input);
    let input: Vec<Vec<char>> = input.split('\n').map(|l| l.chars().collect()).collect();
    let start = start_index(&input);
    let first_neighbour = connected_neighbours(start, &input)[0].0;
    path_len(start, first_neighbour, &input, 1)
}

fn path_len(from_p: Point, p: Point, input: &Map, len: u32) -> u32 {
    dbg!(&p, input[p.0][p.1]);
    let (next_p, next_item) = connected_neighbours(p, input)
        .into_iter()
        .find(|(p, _)| *p != from_p)
        .unwrap();
    if next_item == 'S' {
        return len;
    }
    assert!(len < 10000);
    path_len(p, next_p, input, len + 1)
}

fn connected_neighbours(p: Point, input: &Map) -> Vec<(Point, char)> {
    neighbours(p, input)
        .into_iter()
        .filter(|(other_p, _)| connected_to(p, *other_p, input))
        .collect()
}

fn connected_to(p: Point, other_p: Point, input: &Map) -> bool {
    enum WhichNeighbour {
        Left,
        Right,
        Top,
        Bottom,
    }
    let which_neighbour = if p.0 == other_p.0 {
        if p.1 == other_p.1 + 1 {
            WhichNeighbour::Left
        } else {
            WhichNeighbour::Right
        }
    } else if p.0 == other_p.0 + 1 {
        WhichNeighbour::Top
    } else {
        WhichNeighbour::Bottom
    };
    matches!(
        (input[other_p.0][other_p.1], which_neighbour),
        ('S', _)
        ('│', WhichNeighbour::Top | WhichNeighbour::Bottom)
            | ('─', WhichNeighbour::Left | WhichNeighbour::Right)
            | ('└', WhichNeighbour::Bottom | WhichNeighbour::Left)
            | ('┘', WhichNeighbour::Bottom | WhichNeighbour::Right)
            | ('┐', WhichNeighbour::Top | WhichNeighbour::Right)
            | ('┌', WhichNeighbour::Top | WhichNeighbour::Left)
    )
}

fn neighbours(p: Point, input: &Map) -> Vec<(Point, char)> {
    [
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
    ]
    .map(move |(i, j)| ((i, j), input[i][j]))
    .to_vec()
}

fn start_index(input: &Map) -> Point {
    for (i, line) in input.iter().enumerate() {
        for (j, item) in line.iter().enumerate() {
            if *item == 'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}
pub fn part2(input: &str) -> i32 {
    todo!()
}
