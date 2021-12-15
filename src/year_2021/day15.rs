use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day15");
    assert_eq!(part1(input.clone()), 40);
    assert_eq!(part2(input.clone()), 315);
}

pub fn part2(input: String) -> u64 {
    let board: Vec<Vec<u8>> = input
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let board: Vec<Vec<u8>> = (0..5)
        .flat_map(|y_multip| {
            board.clone().into_iter().map(move |line| {
                (0..5)
                    .flat_map(|x_multip| {
                        line.clone()
                            .into_iter()
                            .map(move |el| (el + y_multip + x_multip - 1) % 9 + 1)
                    })
                    .collect()
            })
        })
        .collect();
    let board = Board::new(board);
    board
        .distance((0, 0), (board.width - 1, board.height - 1))
        .unwrap()
}

pub fn part1(input: String) -> u64 {
    let board: Vec<Vec<u8>> = input
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let board = Board::new(board);
    board
        .distance((0, 0), (board.width - 1, board.height - 1))
        .unwrap()
}

struct Board {
    b: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(board: Vec<Vec<u8>>) -> Self {
        let width = board.len();
        let height = board[0].len();
        Self {
            b: board,
            width,
            height,
        }
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        if x > 0 {
            ret.push((x - 1, y));
        }
        if y > 0 {
            ret.push((x, y - 1));
        }
        if x < self.width - 1 {
            ret.push((x + 1, y));
        }
        if y < self.height - 1 {
            ret.push((x, y + 1));
        }
        ret
    }

    fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.b[y][x]
    }

    fn distance(&self, from: (usize, usize), to: (usize, usize)) -> Option<u64> {
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut next = BinaryHeap::from([Reverse((0, from))]);
        while let Some(Reverse((distance, neighbour))) = next.pop() {
            if neighbour == to {
                return Some(distance);
            }
            if !seen.insert(neighbour) {
                continue;
            }
            for neighbour in self.neighbours(neighbour) {
                next.push(Reverse((distance + self.get(neighbour) as u64, neighbour)));
            }
        }
        None
    }
}
