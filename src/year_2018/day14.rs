#[test]
fn test() {
    assert_eq!(part1(9), 5158916779);
    assert_eq!(part1(2018), 5941429882);
    assert_eq!(part2("51589"), 9);
    assert_eq!(part2("59414"), 2018);
    assert_eq!(part2("01245"), 5);
    assert_eq!(part2("92510"), 18);
}

pub fn part2(input: &str) -> usize {
    let mut board: Vec<u8> = vec![3, 7];
    let mut elf1pos = 0;
    let mut elf2pos = 1;
    let input: Vec<u8> = input
        .chars()
        .map(|i| i.to_string().parse().unwrap())
        .collect();
    let max_iter = 1000000000;

    for _ in 0..max_iter {
        if board.len() > input.len() + 1 && input == board[board.len() - input.len()..board.len()] {
            return board.len() - input.len();
        }
        if board.len() > input.len() + 1
            && input == board[board.len() - 1 - input.len()..board.len() - 1]
        {
            return board.len() - input.len() - 1;
        }

        let v = board[elf1pos] + board[elf2pos];
        if v >= 10 {
            board.push(1);
        }
        board.push(v % 10);
        elf1pos = (elf1pos + board[elf1pos] as usize + 1) % board.len();
        elf2pos = (elf2pos + board[elf2pos] as usize + 1) % board.len();
    }
    unreachable!("should not be reached, maybe you need more iterations?")
}
pub fn part1(input: usize) -> u64 {
    let mut board: Vec<u8> = vec![3, 7];
    let mut elf1pos = 0;
    let mut elf2pos = 1;

    while board.len() < input + 10 {
        let v = board[elf1pos] + board[elf2pos];
        let v: Vec<u8> = v
            .to_string()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        for v in v {
            board.push(v)
        }
        elf1pos = (elf1pos + board[elf1pos] as usize + 1) % board.len();
        elf2pos = (elf2pos + board[elf2pos] as usize + 1) % board.len();
    }
    board
        .iter()
        .rev()
        .take(10)
        .rev()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}
