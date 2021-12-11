#[test]
fn test() {
    assert_eq!(part1(9), 5158916779);
    assert_eq!(part1(2018), 5941429882);
    //assert_eq!(part2(51589), 9);
    //assert_eq!(part2(59414), 2018);
}

pub fn part2(input: usize) -> u64 {
    todo!()
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
