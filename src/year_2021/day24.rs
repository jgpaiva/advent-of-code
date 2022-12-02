use std::io;
use std::io::Write;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    //let input = utils::read_file("2021/day24");
    //assert_eq!(part1(input), 12521);
    // let input = utils::read_file("2021/test_day23-2");
    // assert_eq!(part2(input), 44169);
}

pub fn part2(_input: String) -> u32 {
    todo!()
}

pub fn part1(input: String) -> u32 {
    let ops = input
        .split_terminator('\n')
        .map(|line| parse(line))
        .collect::<Vec<_>>();

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    loop {
        let mut registers = vec![0; 4];
        print!("input code > ");
        stdout.flush().unwrap();
        let mut code = String::new();
        stdin.read_line(&mut code).unwrap();
        let code: Result<Vec<i64>, _> = code
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_string().parse())
            .collect();
        match code {
            Ok(mut code) => {
                for op in ops.clone() {
                    let ret = op.apply(&mut registers, &mut code);
                    if ret.is_err() {
                        break;
                    }
                    println!("{:?}    {:?}", op, registers);
                }
            }
            _ => (),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Op {
    Inp,
    Addr(usize, usize),
    Addi(usize, i64),
    Mulr(usize, usize),
    Muli(usize, i64),
    Divr(usize, usize),
    Divi(usize, i64),
    Modr(usize, usize),
    Modi(usize, i64),
    Eqlr(usize, usize),
    Eqli(usize, i64),
    Setr(usize, usize),
    Seti(usize, i64),
}

#[test]
fn test_parse() {
    let input = "inp w";
    assert_eq!(parse(input), Op::Inp);
    let input = "add x y";
    assert_eq!(parse(input), Op::Addr(0, 1));
    let input = "add x -3";
    assert_eq!(parse(input), Op::Addi(0, -3));
    let input = "mul x y";
    assert_eq!(parse(input), Op::Mulr(0, 1));
    let input = "mul y -3";
    assert_eq!(parse(input), Op::Muli(1, -3));
    let input = "div x y";
    assert_eq!(parse(input), Op::Divr(0, 1));
    let input = "div z -3";
    assert_eq!(parse(input), Op::Divi(2, -3));
    let input = "mod x y";
    assert_eq!(parse(input), Op::Modr(0, 1));
    let input = "mod z -3";
    assert_eq!(parse(input), Op::Modi(2, -3));
    let input = "eql x y";
    assert_eq!(parse(input), Op::Eqlr(0, 1));
    let input = "eql z -3";
    assert_eq!(parse(input), Op::Eqli(2, -3));
    let input = "set z -3";
    assert_eq!(parse(input), Op::Seti(2, -3));
    let input = "set z x";
    assert_eq!(parse(input), Op::Setr(2, 0));
}

fn parse(line: &str) -> Op {
    let (op, args) = line.split_once(' ').unwrap();
    if op.starts_with("inp") {
        return Op::Inp;
    }
    let (arg1, arg2) = args.split_once(' ').unwrap();
    let arg1 = match arg1 {
        "x" => 0,
        "y" => 1,
        "z" => 2,
        "w" => 3,
        _ => unreachable!(),
    };
    match arg2 {
        "x" => parse_op(op, arg1, Some(0), None),
        "y" => parse_op(op, arg1, Some(1), None),
        "z" => parse_op(op, arg1, Some(2), None),
        "w" => parse_op(op, arg1, Some(3), None),
        v => parse_op(op, arg1, None, Some(v.parse().unwrap())),
    }
}

fn parse_op(op: &str, r1: usize, r2: Option<usize>, i: Option<i64>) -> Op {
    match (op, r2, i) {
        ("add", Some(r2), _) => Op::Addr(r1, r2),
        ("add", _, Some(i)) => Op::Addi(r1, i),
        ("mul", Some(r2), _) => Op::Mulr(r1, r2),
        ("mul", _, Some(i)) => Op::Muli(r1, i),
        ("div", Some(r2), _) => Op::Divr(r1, r2),
        ("div", _, Some(i)) => Op::Divi(r1, i),
        ("mod", Some(r2), _) => Op::Modr(r1, r2),
        ("mod", _, Some(i)) => Op::Modi(r1, i),
        ("eql", Some(r2), _) => Op::Eqlr(r1, r2),
        ("eql", _, Some(i)) => Op::Eqli(r1, i),
        ("set", Some(r2), _) => Op::Setr(r1, r2),
        ("set", _, Some(i)) => Op::Seti(r1, i),
        _ => unreachable!(),
    }
}

#[test]
fn test_apply() -> Result<(), ()> {
    let mut reg = vec![0; 4];
    let mut code = vec![1, 2, 3];
    Op::Inp.apply(&mut reg, &mut code)?;
    assert_eq!(reg, [0, 0, 0, 1]);
    assert_eq!(code, [2, 3]);
    Op::Addr(0, 3).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [1, 0, 0, 1]);
    Op::Addi(1, -1).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [1, -1, 0, 1]);
    Op::Mulr(1, 2).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [1, 0, 0, 1]);
    Op::Muli(0, 2).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 0, 0, 1]);
    Op::Divr(0, 3).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 0, 0, 1]);
    Op::Divi(0, 1).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 0, 0, 1]);
    Op::Addi(1, 15).apply(&mut reg, &mut code)?;
    Op::Modi(1, 10).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 5, 0, 1]);
    Op::Modr(1, 0).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 1, 0, 1]);
    Op::Eqlr(1, 3).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 1, 0, 1]);
    Op::Eqli(1, 3).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 0, 0, 1]);
    Op::Seti(2, 3).apply(&mut reg, &mut code)?;
    assert_eq!(reg, [2, 0, 3, 1]);
    assert_eq!(code, [2, 3]);
    Ok(())
}

impl Op {
    fn apply(&self, registers: &mut [i64], code: &mut Vec<i64>) -> Result<(), ()> {
        match *self {
            Op::Inp => {
                if code.len() == 0 {
                    return Err(());
                }
                registers[3] = code[0];
                code.remove(0);
            }
            Op::Addr(r1, r2) => {
                registers[r1] += registers[r2];
            }
            Op::Addi(r1, i) => {
                registers[r1] += i;
            }
            Op::Mulr(r1, r2) => {
                registers[r1] *= registers[r2];
            }
            Op::Muli(r1, i) => {
                registers[r1] *= i;
            }
            Op::Divr(r1, r2) => {
                registers[r1] /= registers[r2];
            }
            Op::Divi(r1, i) => {
                registers[r1] /= i;
            }
            Op::Modr(r1, r2) => {
                registers[r1] = registers[r1] % registers[r2];
            }
            Op::Modi(r1, i) => {
                registers[r1] = registers[r1] % i;
            }
            Op::Eqlr(r1, r2) => {
                registers[r1] = if registers[r1] == registers[r2] { 1 } else { 0 };
            }
            Op::Eqli(r1, i) => {
                registers[r1] = if registers[r1] == i { 1 } else { 0 };
            }
            Op::Setr(r1, r2) => {
                registers[r1] = registers[r2];
            }
            Op::Seti(r1, i) => {
                registers[r1] = i;
            }
        }
        Ok(())
    }
}
