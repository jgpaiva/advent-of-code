use anyhow::{anyhow, Context};
use std::str::FromStr;

use regex::Regex;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 10605);
    assert_eq!(part2(&input), 2713310158);
}

pub fn part2(lines: &str) -> u64 {
    let monkeys = lines
        .split("\n\n")
        .map(|m| m.parse())
        .collect::<Result<Vec<Monkey>, _>>()
        .unwrap();
    run_monkeys(monkeys, 10000, 1)
}

pub fn part1(lines: &str) -> u64 {
    let monkeys = lines
        .split("\n\n")
        .map(|m| m.parse())
        .collect::<Result<Vec<Monkey>, _>>()
        .unwrap();
    run_monkeys(monkeys, 20, 3)
}

fn run_monkeys(mut monkeys: Vec<Monkey>, rounds: i32, divide_by: u64) -> u64 {
    let mut inspections_per_monkey = vec![0; monkeys.len()];
    let working_modulus: u64 = monkeys.iter().map(|m| m.test).product();
    for _ in 0..rounds {
        for m_index in 0..monkeys.len() {
            let items: Vec<_> = {
                let monkey = &mut monkeys[m_index];
                monkey.items.drain(0..).collect()
            };
            let Monkey {
                op,
                test,
                outcome_true,
                outcome_false,
                ..
            } = monkeys[m_index];
            for item in items {
                inspections_per_monkey[m_index] += 1;
                let item = op.apply(item);
                let item = if divide_by != 1 {
                    item / divide_by
                } else {
                    item % working_modulus
                };
                if item % test == 0 {
                    monkeys[outcome_true].items.push(item);
                } else {
                    monkeys[outcome_false].items.push(item);
                }
            }
        }
    }
    inspections_per_monkey.sort();
    inspections_per_monkey[inspections_per_monkey.len() - 2..inspections_per_monkey.len()]
        .iter()
        .product()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    test: u64,
    outcome_true: usize,
    outcome_false: usize,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    TimesConst(u32),
    PlusConst(u32),
    Times,
}

impl Op {
    fn apply(&self, v: u64) -> u64 {
        match self {
            Op::TimesConst(c) => v * *c as u64,
            Op::PlusConst(c) => v + *c as u64,
            Op::Times => v * v,
        }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let re = Regex::new(r"^Operation: new = old (.) (\S+)$").unwrap();
        let mut captures_iter = re.captures_iter(s);
        if let Some(captures) = captures_iter.next() {
            if captures.len() != 3 {
                return Err(anyhow!("Could not find all captures in {s}"));
            }
            let op = &captures[1];
            let v = &captures[2];
            match op {
                "*" if v == "old" => Ok(Self::Times),
                "*" => Ok(Self::TimesConst(
                    v.parse().context(format!("while parsing value: {v}"))?,
                )),
                "+" => Ok(Self::PlusConst(
                    v.parse().context(format!("while parsing value: {v}"))?,
                )),
                _ => Err(anyhow!("unknown operation: {op}")),
            }
        } else {
            Err(anyhow!("input structure didn't match regex: {s}"))
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let re = Regex::new(r"^.*Monkey (\d+):\n\s+Starting items: ((?:\d+,? ?)+)\n.*(Operation.*)\n.*by (\d+)\n.*monkey (\d+)\n.*monkey (\d+)$").unwrap();
        if !re.is_match(s) {
            return Err(anyhow!("output didn't match regex: {s}"));
        };
        let mut captures_iter = re.captures_iter(s);
        if let Some(captures) = captures_iter.next() {
            if captures.len() != 7 {
                return Err(anyhow!("Could not find all captures in {s}"));
            }

            let id = &captures[1];
            let items = &captures[2];
            let op = &captures[3];
            let test = &captures[4];
            let outcome_true = &captures[5];
            let outcome_false = &captures[6];

            let _id: u32 = id.parse().context(format!("while parsing id: {id}"))?;
            let items = items
                .split(", ")
                .map(|v| v.parse().context(format!("while parsing item: {v}")))
                .collect::<Result<Vec<_>, _>>()
                .context(format!("while parsing items: {items}"))?;
            let op: Op = op.parse().context(format!("while parsing op: {op}"))?;
            let test = test
                .parse()
                .context(format!("while parsing test: {test}"))?;
            let outcome_true = outcome_true
                .parse()
                .context(format!("while parsing outcome true: {outcome_true}"))?;
            let outcome_false = outcome_false
                .parse()
                .context(format!("while parsing outcome false: {outcome_false}"))?;

            Ok(Self {
                items,
                op,
                test,
                outcome_true,
                outcome_false,
            })
        } else {
            Err(anyhow!("Could not find captures in {s}"))
        }
    }
}
