#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 2);
    assert_eq!(part2(&input), 4);
}

pub fn part2(lines: &str) -> i32 {
    let lines = parse_input(lines);

    lines
        .into_iter()
        .map(|(elf1, elf2)| match (elf1, elf2) {
            (elf1, elf2) if elf1.0 <= elf2.0 && elf1.1 >= elf2.0 => 1,
            (elf1, elf2) if elf1.0 <= elf2.1 && elf1.1 >= elf2.0 => 1,
            _ => 0,
        })
        .sum()
}

pub fn part1(lines: &str) -> i32 {
    let lines = parse_input(lines);

    lines
        .into_iter()
        .map(|(elf1, elf2)| match (elf1, elf2) {
            (elf1, elf2) if elf1.0 <= elf2.0 && elf1.1 >= elf2.1 => 1,
            (elf1, elf2) if elf1.0 >= elf2.0 && elf1.1 <= elf2.1 => 1,
            _ => 0,
        })
        .sum()
}

fn parse_input(lines: &str) -> Vec<((u32, u32), (u32, u32))> {
    let lines: Vec<((u32, u32), (u32, u32))> = lines
        .split_terminator('\n')
        .map(|line| line.split_once(',').expect("should always have a comma"))
        .map(|(elf1, elf2)| {
            (
                elf1.split_once('-').expect("should always have a dash"),
                elf2.split_once('-').expect("should always have a dash"),
            )
        })
        .map(|((e1_start, e1_end), (e2_start, e2_end))| {
            (
                (
                    e1_start.parse::<u32>().expect("should be num"),
                    e1_end.parse::<u32>().expect("should be num"),
                ),
                (
                    e2_start.parse::<u32>().expect("should be num"),
                    e2_end.parse::<u32>().expect("should be num"),
                ),
            )
        })
        .collect();
    lines
}
