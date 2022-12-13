use anyhow::{anyhow, Context};
use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use std::cmp::Ordering;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    use Msg::{Const, List};
    let input = utils::read_test_file(file!());
    assert_eq!(parse_msg_const("119"), Ok(("", Const(119))));
    assert_eq!(parse_msg_list("[]"), Ok(("", List(vec![]))));
    assert_eq!(parse_msg_list("[1]"), Ok(("", List(vec![Const(1)]))));
    assert_eq!(
        parse_msg_list("[1,[[2,[]]],4]"),
        Ok((
            "",
            List(vec![
                Const(1),
                List(vec![List(vec![Const(2), List(vec![])])]),
                Const(4)
            ])
        ))
    );
    assert_eq!(part1(&input), 13);
    assert_eq!(part2(&input), 140);
}

fn parse_msg_const(input: &str) -> IResult<&str, Msg> {
    map(map_res(digit1, str::parse), Msg::Const)(input)
}

fn parse_msg_list(input: &str) -> IResult<&str, Msg> {
    map(
        tuple((
            tag("["),
            separated_list0(tag(","), alt((parse_msg_const, parse_msg_list))),
            tag("]"),
        )),
        |(_, v, _)| Msg::List(v),
    )(input)
}

pub fn part2(lines: &str) -> usize {
    let marker1 = "[[6]]";
    let marker2 = "[[2]]";
    let lines = format!("{lines}\n\n{marker1}\n{marker2}");
    let input = parse_input(&lines).unwrap();
    let mut input: Vec<_> = input
        .into_iter()
        .flat_map(|(left, right)| [left, right].into_iter())
        .collect();
    input.sort();
    let (_, marker1) = parse_msg_list(marker1).unwrap();
    let (_, marker2) = parse_msg_list(marker2).unwrap();
    input
        .into_iter()
        .enumerate()
        .filter(|(_, v)| v.eq(&marker1) || v.eq(&marker2))
        .map(|(i, _)| i + 1)
        .product()
}

pub fn part1(lines: &str) -> usize {
    let input = parse_input(lines).unwrap();
    input
        .into_iter()
        .enumerate()
        .flat_map(|(v, (left, right))| matches!(left.cmp(&right), Ordering::Less).then(|| v + 1))
        .sum()
}

impl Ord for Msg {
    fn cmp(&self, right: &Self) -> Ordering {
        match (self, right) {
            (Msg::Const(c_left), Msg::Const(c_right)) => c_left.cmp(c_right),
            (Msg::List(l_left), Msg::List(l_right)) => l_left
                .iter()
                .zip_longest(l_right)
                .map(|cmp| match cmp {
                    EitherOrBoth::Both(left, right) => left.cmp(right),
                    EitherOrBoth::Left(_) => Ordering::Greater,
                    EitherOrBoth::Right(_) => Ordering::Less,
                })
                .find(|ordering| !matches!(ordering, Ordering::Equal))
                .unwrap_or(Ordering::Equal),
            (_, Msg::Const(c_right)) => self.cmp(&Msg::List(vec![Msg::Const(*c_right)])),
            (Msg::Const(c_left), _) => Msg::List(vec![Msg::Const(*c_left)]).cmp(right),
        }
    }
}

impl PartialOrd for Msg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(lines: &str) -> anyhow::Result<Vec<(Msg, Msg)>> {
    lines
        .split("\n\n")
        .map(|l| l.split('\n').collect::<Vec<_>>())
        .map(|lines| {
            let (left, right) = lines
                .iter()
                .map(|line| {
                    let (leftover, msg) = parse_msg_list(line)
                        .map_err(|e| anyhow!("Parsing error in line {line}: {e}"))?;
                    if !leftover.is_empty() {
                        return Err(anyhow!(
                            "Didn't parse full input in line {line}. Leftover: {leftover}",
                        ));
                    }
                    Ok(msg)
                })
                .collect_tuple()
                .context(anyhow!(
                    "A set of lines didn't have exactly 2 lines: {lines:?}"
                ))?;
            let left = left?;
            let right = right?;
            Ok((left, right))
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Msg {
    List(Vec<Msg>),
    Const(u32),
}
