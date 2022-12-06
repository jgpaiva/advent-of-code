use std::collections::HashSet;

#[test]
fn test() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

pub fn part2(line: &str) -> usize {
    let line = line.as_bytes();
    for i in 13..line.len() {
        if line[i - 13..=i]
            .iter()
            .cloned()
            .collect::<HashSet<u8>>()
            .len()
            == 14
        {
            return i + 1;
        }
    }
    unreachable!()
}

pub fn part1(line: &str) -> usize {
    let line = line.as_bytes();
    for i in 3..line.len() {
        if line[i - 3..=i]
            .iter()
            .cloned()
            .collect::<HashSet<u8>>()
            .len()
            == 4
        {
            return i + 1;
        }
    }
    unreachable!()
}
