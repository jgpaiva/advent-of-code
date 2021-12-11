#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day11");
    assert_eq!(part1(input.clone(), 10), 204);
    assert_eq!(part1(input.clone(), 100), 1656);
    assert_eq!(part2(input.clone(), 200), 195);
}

pub fn part2(input: String, steps: u32) -> u32 {
    run_simulation(input, steps).1.unwrap()
}

pub fn part1(input: String, steps: u32) -> u32 {
    run_simulation(input, steps).0
}

fn run_simulation(input: String, steps: u32) -> (u32, Option<u32>) {
    let mut board: Vec<Vec<(u8, bool)>> = input
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| (c.to_string().parse().unwrap(), false))
                .collect()
        })
        .collect();
    let mut flashes = 0;
    let mut all_flash = None;
    for step in 1..=steps {
        for x in 0..board.len() {
            for y in 0..board[0].len() {
                board[x][y] = (board[x][y].0 + 1, false);
            }
        }
        for x in 0..board.len() {
            for y in 0..board[0].len() {
                if board[x][y].0 > 9 {
                    flash(&mut board, x, y);
                }
            }
        }
        if board
            .iter()
            .flat_map(|line| line.to_vec())
            .all(|(_, flash)| flash)
        {
            all_flash.get_or_insert(step);
        }
        for x in 0..board.len() {
            for y in 0..board[0].len() {
                if board[x][y].1 {
                    flashes += 1;
                    board[x][y] = (0, false);
                }
            }
        }
    }
    (flashes, all_flash)
}

fn flash(board: &mut [Vec<(u8, bool)>], x: usize, y: usize) {
    if board[x][y].1 {
        // has flashed, nothing to do
        return;
    }
    board[x][y] = (board[x][y].0, true);
    let x: i32 = x as i32;
    let y: i32 = y as i32;
    for (x, y) in [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ] {
        if x < 0 || y < 0 {
            continue;
        }
        let x: usize = x as usize;
        let y: usize = y as usize;
        if x < board.len() && y < board.len() {
            board[x][y] = (board[x][y].0 + 1, board[x][y].1);
            if board[x][y].0 > 9 {
                flash(board, x, y);
            }
        }
    }
}
