#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 18);
    assert_eq!(part2(&input), 9);
}

pub fn part1(input: &str) -> i32 {
    let input: Vec<_> = input.split("\n").collect();
    let mut count = 0;
    for i_line in 0..input.len() {
        let line = input[i_line].as_bytes();
        for (i_item, item) in line.iter().enumerate() {
            if *item as char == 'X' {
                count += check(i_line as i32, i_item as i32, &input);
            }
        }
    }
    count
}

fn check(i_line: i32, i_item: i32, input: &[&str]) -> i32 {
    let mut count = 0;
    let char_at = |i_line: i32, i_item: i32, input: &[&str]| {
        input[usize::try_from(i_line).unwrap()].as_bytes()[usize::try_from(i_item).unwrap()] as char
    };
    if i_line - 3 >= 0
        && char_at(i_line - 1, i_item, input) == 'M'
        && char_at(i_line - 2, i_item, input) == 'A'
        && char_at(i_line - 3, i_item, input) == 'S'
    {
        count += 1;
    }
    if i_line + 3 < input.len() as i32
        && char_at(i_line + 1, i_item, input) == 'M'
        && char_at(i_line + 2, i_item, input) == 'A'
        && char_at(i_line + 3, i_item, input) == 'S'
    {
        count += 1;
    }
    if i_item + 3 < input[0].len() as i32
        && char_at(i_line, i_item + 1, input) == 'M'
        && char_at(i_line, i_item + 2, input) == 'A'
        && char_at(i_line, i_item + 3, input) == 'S'
    {
        count += 1;
    }
    if i_item - 3 >= 0
        && char_at(i_line, i_item - 1, input) == 'M'
        && char_at(i_line, i_item - 2, input) == 'A'
        && char_at(i_line, i_item - 3, input) == 'S'
    {
        count += 1;
    }
    if i_line - 3 >= 0
        && i_item - 3 >= 0
        && char_at(i_line - 1, i_item - 1, input) == 'M'
        && char_at(i_line - 2, i_item - 2, input) == 'A'
        && char_at(i_line - 3, i_item - 3, input) == 'S'
    {
        count += 1;
    }
    if i_line + 3 < input.len() as i32
        && i_item + 3 < input[0].len() as i32
        && char_at(i_line + 1, i_item + 1, input) == 'M'
        && char_at(i_line + 2, i_item + 2, input) == 'A'
        && char_at(i_line + 3, i_item + 3, input) == 'S'
    {
        count += 1;
    }
    if i_item + 3 < input[0].len() as i32
        && i_line - 3 >= 0
        && char_at(i_line - 1, i_item + 1, input) == 'M'
        && char_at(i_line - 2, i_item + 2, input) == 'A'
        && char_at(i_line - 3, i_item + 3, input) == 'S'
    {
        count += 1;
    }
    if i_item - 3 >= 0
        && i_line + 3 < input.len() as i32
        && char_at(i_line + 1, i_item - 1, input) == 'M'
        && char_at(i_line + 2, i_item - 2, input) == 'A'
        && char_at(i_line + 3, i_item - 3, input) == 'S'
    {
        count += 1;
    }
    count
}

pub fn part2(input: &str) -> i32 {
    let input: Vec<_> = input.split("\n").collect();
    let mut count = 0;
    for i_line in 0..input.len() {
        let line = input[i_line].as_bytes();
        for (i_item, item) in line.iter().enumerate() {
            if *item as char == 'A' {
                count += check_mas(i_line as i32, i_item as i32, &input);
            }
        }
    }
    count
}

fn check_mas(i_line: i32, i_item: i32, input: &[&str]) -> i32 {
    let char_at = |i_line: i32, i_item: i32| {
        input[usize::try_from(i_line).unwrap()].as_bytes()[usize::try_from(i_item).unwrap()] as char
    };
    if !(i_line > 0
        && i_item > 0
        && i_line + 1 < input.len() as i32
        && i_item + 1 < input[0].len() as i32)
    {
        return 0;
    }

    let first = (char_at(i_line - 1, i_item - 1) == 'M' && char_at(i_line + 1, i_item + 1) == 'S')
        || (char_at(i_line - 1, i_item - 1) == 'S' && char_at(i_line + 1, i_item + 1) == 'M');
    let second = (char_at(i_line - 1, i_item + 1) == 'M' && char_at(i_line + 1, i_item - 1) == 'S')
        || (char_at(i_line - 1, i_item + 1) == 'S' && char_at(i_line + 1, i_item - 1) == 'M');

    if first && second {
        1
    } else {
        0
    }
}
