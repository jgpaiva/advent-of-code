use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let mut v = [1, 2, 3, 4];
    Op::Addr(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Addi(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 2, 4]);

    let mut v = [3, 2, 3, 4];
    Op::Mulr(0, 1, 2).apply(&mut v);
    assert_eq!(v, [3, 2, 6, 4]);

    let mut v = [3, 2, 3, 4];
    Op::Muli(0, 2, 2).apply(&mut v);
    assert_eq!(v, [3, 2, 6, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Banr(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 0, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Bani(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 1, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Borr(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Bori(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 1, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Setr(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 1, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Seti(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 0, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Gtir(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 0, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Gtri(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 0, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Gtrr(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 0, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Eqir(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 0, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Eqri(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 1, 4]);

    let mut v = [1, 2, 3, 4];
    Op::Eqrr(0, 1, 2).apply(&mut v);
    assert_eq!(v, [1, 2, 0, 4]);

    let input = utils::read_file("2018/test_day16");
    assert_eq!(part1(input), 2);
}

#[allow(unused)]
enum Op {
    Addr(usize, usize, usize),
    Addi(usize, usize, usize),
    Mulr(usize, usize, usize),
    Muli(usize, usize, usize),
    Banr(usize, usize, usize),
    Bani(usize, usize, usize),
    Borr(usize, usize, usize),
    Bori(usize, usize, usize),
    Setr(usize, usize, usize),
    Seti(usize, usize, usize),
    Gtir(usize, usize, usize),
    Gtri(usize, usize, usize),
    Gtrr(usize, usize, usize),
    Eqir(usize, usize, usize),
    Eqri(usize, usize, usize),
    Eqrr(usize, usize, usize),
}

impl Op {
    fn build(op: usize, a: usize, b: usize, c: usize) -> Self {
        match op {
            1 => Op::Addr(a, b, c),
            2 => Op::Addi(a, b, c),
            3 => Op::Mulr(a, b, c),
            4 => Op::Muli(a, b, c),
            5 => Op::Banr(a, b, c),
            6 => Op::Bani(a, b, c),
            7 => Op::Borr(a, b, c),
            8 => Op::Bori(a, b, c),
            9 => Op::Setr(a, b, c),
            10 => Op::Seti(a, b, c),
            11 => Op::Gtir(a, b, c),
            12 => Op::Gtri(a, b, c),
            13 => Op::Gtrr(a, b, c),
            14 => Op::Eqir(a, b, c),
            15 => Op::Eqri(a, b, c),
            16 => Op::Eqrr(a, b, c),
            _ => unreachable!(),
        }
    }
    fn apply(&self, r: &mut [usize]) {
        match self {
            Op::Addr(a1, a2, reg) => r[*reg] = r[*a1] + r[*a2],
            Op::Addi(a1, v, reg) => r[*reg] = r[*a1] + v,
            Op::Mulr(a1, a2, reg) => r[*reg] = r[*a1] * r[*a2],
            Op::Muli(a1, v, reg) => r[*reg] = r[*a1] * v,
            Op::Banr(a1, a2, reg) => r[*reg] = r[*a1] & r[*a2],
            Op::Bani(a1, v, reg) => r[*reg] = r[*a1] & v,
            Op::Borr(a1, a2, reg) => r[*reg] = r[*a1] | r[*a2],
            Op::Bori(a1, v, reg) => r[*reg] = r[*a1] | v,
            Op::Setr(a1, _, reg) => r[*reg] = r[*a1],
            Op::Seti(v, _, reg) => r[*reg] = *v,
            Op::Gtir(v, a2, reg) => r[*reg] = usize::from(*v > r[*a2]),
            Op::Gtri(a1, v, reg) => r[*reg] = usize::from(r[*a1] > *v),
            Op::Gtrr(a1, a2, reg) => r[*reg] = usize::from(r[*a1] > r[*a2]),
            Op::Eqir(v, a2, reg) => r[*reg] = usize::from(*v == r[*a2]),
            Op::Eqri(a1, v, reg) => r[*reg] = usize::from(r[*a1] == *v),
            Op::Eqrr(a1, a2, reg) => r[*reg] = usize::from(r[*a1] == r[*a2]),
        }
    }
}

pub fn part2(input: String) -> usize {
    let (program, samples) = parse(input);

    let mut mapping = HashMap::<usize, usize>::new();
    for _i in 0..16 {
        for (before, original_op, after) in samples.iter().cloned() {
            if mapping.contains_key(&original_op[0]) {
                continue;
            }
            let mut found_op_code = (0, 0);
            let mut found = 0;
            for opcode in 1..=16 {
                if mapping.values().any(|x| *x == opcode) {
                    continue;
                }
                let op = Op::build(opcode, original_op[1], original_op[2], original_op[3]);
                let mut before = before.clone();
                op.apply(&mut before);
                if before == after {
                    found += 1;
                    found_op_code = (original_op[0], opcode);
                }
            }
            if found == 1 {
                mapping.insert(found_op_code.0, found_op_code.1);
            }
        }
    }
    assert_eq!(mapping.len(), 16);
    let mut register = vec![0, 0, 0, 0];
    for instruction in program {
        let (opcode, a, b, c) = (
            instruction[0],
            instruction[1],
            instruction[2],
            instruction[3],
        );
        let opcode = mapping[&opcode];
        let op = Op::build(opcode, a, b, c);
        op.apply(&mut register);
    }
    register[0]
}

pub fn part1(input: String) -> u32 {
    let (program, samples) = parse(input);

    program.len();
    let mut total_over_three = 0;
    for (before, op, after) in samples {
        let mut found = 0;
        for opcode in 1..=16 {
            let op = Op::build(opcode, op[1], op[2], op[3]);
            let mut before = before.clone();
            op.apply(&mut before);
            if before == after {
                found += 1;
            }
        }
        if found >= 3 {
            total_over_three += 1;
        }
    }
    total_over_three
}

#[allow(clippy::type_complexity)]
fn parse(input: String) -> (Vec<Vec<usize>>, Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>) {
    let (samples, program) = input.split_once("\n\n\n\n").unwrap();
    let samples = samples.split_terminator('\n');
    let samples: Vec<(Vec<_>, Vec<_>, Vec<_>)> = samples
        .clone()
        .zip(samples.clone().skip(1))
        .zip(samples.clone().skip(2))
        .step_by(4)
        .map(|((before, op), after)| {
            // Your scientists were so preoccupied with whether they could, they didn’t stop to think if they should
            (
                before
                    .split_once("Before: [")
                    .unwrap()
                    .1
                    .split_once(']')
                    .unwrap()
                    .0
                    .split(", ")
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>(),
                op.split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>(),
                after
                    .split_once("After:  [")
                    .unwrap()
                    .1
                    .split_once(']')
                    .unwrap()
                    .0
                    .split(", ")
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();
    let program = program
        .split_terminator('\n')
        .map(|line| {
            line.split_terminator(' ')
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    (program, samples)
}
