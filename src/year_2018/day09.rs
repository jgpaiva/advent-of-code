use std::collections::LinkedList;

#[test]
fn test() {
    assert_eq!(part1(9, 25), 32);
    assert_eq!(part2(9, 25), 32);
}

pub fn part2(players: usize, last_marble: i32) -> u64 {
    let mut board = LinkedList::from([2, 1, 0]);
    let mut player = 3;
    let mut scoreboard: Vec<u64> = (0..=players).map(|_| 0).collect();
    for marble in 3..=last_marble {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let v = board.pop_front().unwrap();
                board.push_back(v);
            }
            let removed = board.pop_back().unwrap();
            scoreboard[player] += marble as u64 + removed as u64;
        } else {
            for _ in 0..2 {
                let v = board.pop_back().unwrap();
                board.push_front(v);
            }
            board.push_back(marble);
        }
        player = (player + 1) % players;
    }
    scoreboard.into_iter().max().unwrap()
}

#[allow(dead_code)]
pub fn part1(players: usize, last_marble: i32) -> i32 {
    let mut board = vec![0, 2, 1];
    let mut player = 3;
    let mut last = 1;
    let mut scoreboard: Vec<i32> = (0..=players).map(|_| 0).collect();
    for marble in 3..=last_marble {
        let mut insert_at = (last + 2) % board.len();
        if marble % 23 == 0 {
            let remove_at = (last + board.len() - 7) % board.len();
            let removed = board.remove(remove_at);
            scoreboard[player] += marble + removed;
            insert_at = remove_at % board.len();
        } else {
            board.insert(insert_at, marble);
        }
        last = insert_at;
        player = (player + 1) % players;
    }
    scoreboard.into_iter().max().unwrap()
}
