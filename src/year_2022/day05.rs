#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), "CMZ");
    assert_eq!(part2(&input), "MCD");
}

pub fn part2(lines: &str) -> String {
    let (mut stack, instructions) = parse_input(lines);
    for (move_num, source, target) in instructions {
        // need to collect here so I'm not modifying stack concurrently
        #[allow(clippy::needless_collect)]
        let popped: Vec<_> = (0..move_num)
            .map(|_| stack[source].pop().unwrap())
            .collect();
        for el in popped.into_iter().rev() {
            stack[target].push(el);
        }
    }
    stack
        .into_iter()
        .filter_map(|s| s.last().cloned())
        .collect()
}

pub fn part1(lines: &str) -> String {
    let (mut stack, instructions) = parse_input(lines);
    for (move_num, source, target) in instructions {
        for _ in 0..move_num {
            let el = stack[source].pop().unwrap();
            stack[target].push(el);
        }
    }
    stack
        .into_iter()
        .filter_map(|s| s.last().cloned())
        .collect()
}

type Stack = Vec<Vec<char>>;
type Instructions = Vec<(usize, usize, usize)>;

fn parse_input(lines: &str) -> (Stack, Instructions) {
    let mut on_instructions = false;
    let mut stack_lines = vec![];
    let mut instructions_lines = vec![];
    for line in lines.split_terminator('\n') {
        if line.trim().is_empty() {
            on_instructions = true;
        } else if on_instructions {
            instructions_lines.push(line);
        } else {
            stack_lines.push(line);
        }
    }
    let mut stack: Stack = (0..stack_lines
        .last()
        .expect("must have at least one line")
        .split(' ')
        .filter(|x| !x.trim().is_empty())
        .count())
        .map(|_| vec![])
        .collect();
    for line in stack_lines.into_iter().rev().skip(1) {
        let line = line.as_bytes();
        for (i, stack_el) in stack.iter_mut().enumerate() {
            let el = line[i * 4 + 1] as char;
            if el != ' ' {
                stack_el.push(el);
            }
        }
    }
    let instructions: Instructions = instructions_lines
        .into_iter()
        .map(|line| {
            let line: Vec<_> = line.split(' ').collect();
            let move_num: usize = line[1].parse().unwrap();
            let source: usize = line[3].parse().unwrap();
            let dest: usize = line[5].parse().unwrap();
            (move_num, source - 1, dest - 1)
        })
        .collect();
    (stack, instructions)
}
