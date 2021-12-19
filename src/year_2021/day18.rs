use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let num = "[[[[[9,8],1],2],3],4]";
    assert_eq!(print_number(parse_number(num)), num);
    println!("1");
    assert_eq!(
        print_number(explode_number(parse_number("[[1,2],[3,4]]"))),
        "[[1,2],[3,4]]"
    );
    println!("2");
    assert_eq!(
        print_number(explode_number(parse_number("[[[1,2],[3,4]],5]"))),
        "[[[1,2],[3,4]],5]"
    );
    println!("3");
    assert_eq!(
        print_number(explode_number(parse_number("[[[[1,2],[3,4]],5],6]"))),
        "[[[[1,2],[3,4]],5],6]"
    );
    println!("4");
    assert_eq!(
        print_number(explode_number(parse_number("[[[[[9,8],1],2],3],4]"))),
        "[[[[0,9],2],3],4]"
    );
    assert_eq!(
        traverse_number_right(parse_number("[[1,2],[[[[9,8],1],2],3]]")),
        vec![1, 2, 9, 8, 1, 2, 3]
    );
    assert_eq!(
        traverse_number_left(parse_number("[[1,2],[[[[9,8],1],2],3]]")),
        vec![3, 2, 1, 8, 9, 2, 1]
    );
    assert_eq!(
        print_number(explode_number(parse_number("[7,[6,[5,[4,[3,2]]]]]"))),
        "[7,[6,[5,[7,0]]]]"
    );
    assert_eq!(
        print_number(explode_number(parse_number("[[6,[5,[4,[3,2]]]],1]"))),
        "[[6,[5,[7,0]]],3]"
    );
    assert_eq!(
        print_number(explode_number(parse_number("[[1,2],[[[[1,5],2],3],4]]"))),
        "[[1,3],[[[0,7],3],4]]"
    );
    assert_eq!(
        print_number(explode_number(parse_number(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
        ))),
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    );
    assert_eq!(
        print_number(explode_number(parse_number(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        ))),
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
    );
    assert_eq!(
        print_number(split_number(explode_number(parse_number(
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"
        )))),
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
    );

    let input = utils::read_file("2021/test_day18");
    assert_eq!(part1(input.clone()), 4140);
    assert_eq!(part2(input.clone()), 3993);
}

pub fn part2(input: String) -> u32 {
    let input = input
        .split_terminator('\n')
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let input = input
        .clone()
        .into_iter()
        .flat_map(|n1| {
            input
                .clone()
                .into_iter()
                .map(move |n2| {
                    if n1 != n2 {
                        Some((n1.clone(), n2))
                    } else {
                        None
                    }
                })
                .flatten()
        })
        .map(|(n1, n2)| (n1.clone(), n2.clone(), add_numbers(n1, n2)));
    let max = input
        .max_by(|v1, v2| magnitude_recur(v1.2.clone()).cmp(&magnitude_recur(v2.2.clone())))
        .unwrap();
    dbg!(&max.0, &max.1, print_number(max.2.clone()));
    magnitude_recur(max.2)
}

pub fn part1(input: String) -> u32 {
    let input = input
        .split_terminator('\n')
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let input = input
        .clone()
        .into_iter()
        .skip(1)
        .fold(input[0].clone(), |acc, i| {
            let sum = add_numbers(acc, i);
            print_number(sum)
        });
    dbg!(&input);
    magnitude_recur(parse_number(input.as_ref()))
}

fn add_numbers(n1: String, n2: String) -> Rc<RefCell<Tree>> {
    let sum: String = vec!["[".to_string(), n1, ",".to_string(), n2, "]".to_string()]
        .into_iter()
        .collect();
    let sum: Num = parse_number(sum.as_ref());
    for i in 0..1000 {
        assert!(i != 999);
        let explode_res = explode_number_recur(sum.clone(), 0);
        if matches!(explode_res, OpResult::Done) {
            continue;
        }
        let split_res = split_number_recur(sum.clone());
        if matches!(split_res, OpResult::Noop) {
            break;
        }
    }
    sum
}

fn magnitude_recur(input: Num) -> u32 {
    match &*input.borrow() {
        Tree::Leaf { v, .. } => *v as u32,
        Tree::Inner { left, right } => {
            3 * magnitude_recur(left.clone()) + 2 * magnitude_recur(right.clone())
        }
    }
}

enum OpResult {
    Done,
    Noop,
}

fn split_number_recur(input: Num) -> OpResult {
    let (input, v, left, right) = match &*input.borrow() {
        Tree::Leaf { v, left, right } => {
            if *v < 10 {
                return OpResult::Noop;
            } else {
                (input.clone(), *v, left.clone(), right.clone())
            }
        }
        Tree::Inner { left, right } => {
            let left_recur = split_number_recur(left.clone());
            if !matches!(left_recur, OpResult::Noop) {
                return left_recur;
            }
            return split_number_recur(right.clone());
        }
    };
    split(input, v, left, right);
    OpResult::Done
}

fn split(
    parent: Rc<RefCell<Tree>>,
    v: u8,
    parent_left: Weak<RefCell<Tree>>,
    parent_right: Weak<RefCell<Tree>>,
) {
    let left_v = v / 2;
    let right_v = v - left_v;
    let left_leaf = Tree::build_leaf(left_v);
    let right_leaf = Tree::build_leaf(right_v);
    let new_parent = Tree::Inner {
        left: left_leaf.clone(),
        right: right_leaf.clone(),
    };
    if let Tree::Leaf {
        ref mut right,
        ref mut left,
        ..
    } = *left_leaf.borrow_mut()
    {
        *right = Rc::downgrade(&right_leaf);
        if let Some(left_upgraded) = parent_left.upgrade() {
            if let Tree::Leaf { ref mut right, .. } = *left_upgraded.borrow_mut() {
                *right = Rc::downgrade(&left_leaf);
                *left = Rc::downgrade(&left_upgraded);
            } else {
                unreachable!()
            }
        }
    } else {
        unreachable!()
    };
    if let Tree::Leaf {
        ref mut right,
        ref mut left,
        ..
    } = *right_leaf.borrow_mut()
    {
        *left = Rc::downgrade(&left_leaf);
        if let Some(right_upgraded) = parent_right.upgrade() {
            if let Tree::Leaf { ref mut left, .. } = *right_upgraded.borrow_mut() {
                *left = Rc::downgrade(&right_leaf);
                *right = Rc::downgrade(&right_upgraded);
            } else {
                unreachable!()
            }
        }
    } else {
        unreachable!()
    };
    let mut b = parent.borrow_mut();
    *b = new_parent;
}

fn explode_number_recur(input: Num, level: usize) -> OpResult {
    let (left, right, input) = match &*input.borrow() {
        Tree::Leaf { .. } => return OpResult::Noop,
        Tree::Inner { left, right } => {
            if level == 4 {
                (left.clone(), right.clone(), input.clone())
            } else {
                let left_explosion = explode_number_recur(left.clone(), level + 1);
                if !matches!(left_explosion, OpResult::Noop) {
                    return left_explosion;
                }
                return explode_number_recur(right.clone(), level + 1);
            }
        }
    };
    explode(input, left, right);
    OpResult::Done
}

fn explode(parent: Rc<RefCell<Tree>>, left: Rc<RefCell<Tree>>, right: Rc<RefCell<Tree>>) {
    {
        let mut b = parent.borrow_mut();
        *b = Tree::Leaf {
            v: 0,
            right: Weak::new(),
            left: Weak::new(),
        };
    }
    if let Tree::Leaf { v, ref left, .. } = &*left.borrow() {
        let exploded_v = *v;
        if let Some(left_upgraded) = left.upgrade() {
            if let Tree::Leaf {
                ref mut right,
                ref mut v,
                ..
            } = *left_upgraded.borrow_mut()
            {
                *right = Rc::downgrade(&parent);
                *v += exploded_v;
                set_weak_left(&parent, &left_upgraded);
            } else {
                unreachable!()
            }
        }
    }
    if let Tree::Leaf { v, ref right, .. } = &*right.borrow() {
        let exploded_v = *v;
        if let Some(right_upgraded) = right.upgrade() {
            if let Tree::Leaf {
                ref mut left,
                ref mut v,
                ..
            } = *right_upgraded.borrow_mut()
            {
                *left = Rc::downgrade(&parent);
                *v += exploded_v;
                set_weak_right(&parent, &right_upgraded);
            } else {
                unreachable!()
            }
        }
    }
}

type Num = Rc<RefCell<Tree>>;

#[derive(Clone)]
enum Tree {
    Leaf {
        v: u8,
        left: Weak<RefCell<Tree>>,
        right: Weak<RefCell<Tree>>,
    },
    Inner {
        left: Num,
        right: Num,
    },
}

fn parse_number(input: &str) -> Num {
    let chars: Vec<char> = input.chars().collect();
    parse_number_aux(&chars, None).1
}

fn parse_number_aux(chars: &[char], last_num: Option<Num>) -> (usize, Num, (Num, Num)) {
    assert_eq!(chars[0], '[');
    let (size_left, left, (left_first_num, left_last_num)) = match chars[1] {
        '[' => parse_number_aux(&chars[1..], last_num.clone()),
        v => {
            let v: u8 = v.to_string().parse().unwrap();
            let leaf = Tree::build_leaf(v);
            (1, leaf.clone(), (leaf.clone(), leaf))
        }
    };
    if let Some(last_num) = last_num {
        set_weak_right(&last_num, &left_first_num);
        set_weak_left(&left_first_num, &last_num);
    }
    assert_eq!(chars[size_left + 1], ',');
    let (size_right, right, (right_first_num, right_last_num)) = match chars[size_left + 2] {
        '[' => parse_number_aux(&chars[size_left + 2..], Some(left_last_num.clone())),
        v => {
            let v: u8 = v.to_string().parse().unwrap();
            let leaf = Tree::build_leaf(v);
            (1, leaf.clone(), (leaf.clone(), leaf))
        }
    };
    set_weak_right(&left_last_num, &right_first_num);
    set_weak_left(&right_first_num, &left_last_num);
    assert_eq!(chars[size_left + size_right + 2], ']');
    (
        size_left + size_right + 3,
        Tree::build_inner(left, right),
        (left_first_num, right_last_num),
    )
}

fn set_weak_right(leaf: &Rc<RefCell<Tree>>, set_to: &Rc<RefCell<Tree>>) {
    let mut b = (*leaf).borrow_mut();
    match *b {
        Tree::Leaf { ref mut right, .. } => {
            *right = Rc::downgrade(set_to);
        }
        _ => unreachable!(),
    }
}
fn set_weak_left(leaf: &Rc<RefCell<Tree>>, set_to: &Rc<RefCell<Tree>>) {
    let mut b = (*leaf).borrow_mut();
    match *b {
        Tree::Leaf { ref mut left, .. } => {
            *left = Rc::downgrade(set_to);
        }
        _ => unreachable!(),
    }
}

fn print_number(input: Num) -> String {
    match &*input.borrow() {
        Tree::Leaf { v, .. } => v.to_string(),
        Tree::Inner { left, right } => vec![
            "[".to_string(),
            print_number(left.clone()),
            ",".to_string(),
            print_number(right.clone()),
            "]".to_string(),
        ]
        .into_iter()
        .collect(),
    }
}

impl Tree {
    fn build_leaf(v: u8) -> Rc<RefCell<Tree>> {
        Rc::new(RefCell::new(Tree::Leaf {
            v,
            left: Weak::new(),
            right: Weak::new(),
        }))
    }

    fn build_inner(left: Rc<RefCell<Tree>>, right: Rc<RefCell<Tree>>) -> Rc<RefCell<Tree>> {
        Rc::new(RefCell::new(Tree::Inner { left, right }))
    }
}

#[cfg(test)]
fn explode_number(input: Num) -> Num {
    explode_number_recur(input.clone(), 0);
    input
}

#[cfg(test)]
fn split_number(input: Num) -> Num {
    split_number_recur(input.clone());
    input
}

#[cfg(test)]
fn traverse_number_right(input: Num) -> Vec<u8> {
    match &*input.borrow() {
        Tree::Leaf { v, right, .. } => {
            let mut ret = vec![*v];
            if let Some(right) = right.upgrade() {
                ret.append(&mut traverse_number_right(right.clone()));
            }
            ret
        }
        Tree::Inner { left, .. } => return traverse_number_right(left.clone()),
    }
}

#[cfg(test)]
fn traverse_number_left(input: Num) -> Vec<u8> {
    match &*input.borrow() {
        Tree::Leaf { v, left, .. } => {
            let mut ret = vec![*v];
            if let Some(left) = left.upgrade() {
                ret.append(&mut traverse_number_left(left.clone()));
            }
            ret
        }
        Tree::Inner { right, .. } => return traverse_number_left(right.clone()),
    }
}
