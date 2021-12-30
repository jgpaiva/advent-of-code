#[cfg(test)]
use std::collections::HashSet;
use std::{collections::HashMap, fmt};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day23");
    assert_eq!(part1(input), 12521);
    // let input = utils::read_file("2021/test_day23");
    // assert_eq!(part2(input), 2758514936282235);
}

pub fn part2(_input: String) -> u64 {
    todo!()
}

#[derive(Default, Eq, PartialEq, Hash, Clone)]
struct Board {
    slot_1_2: u8,
    slot_1_1: u8,
    slot_2_2: u8,
    slot_2_1: u8,
    slot_3_2: u8,
    slot_3_1: u8,
    slot_4_2: u8,
    slot_4_1: u8,
    hallway_0: u8,
    hallway_1: u8,
    hallway_1_2: u8,
    hallway_2_3: u8,
    hallway_3_4: u8,
    hallway_2: u8,
    hallway_3: u8,
    depth: u8,
}

impl Board {
    fn get_slot(&self, slot_num: u8, pos: u8) -> u8 {
        match (slot_num, pos) {
            (1, 2) => self.slot_1_2,
            (1, 1) => self.slot_1_1,
            (2, 2) => self.slot_2_2,
            (2, 1) => self.slot_2_1,
            (3, 2) => self.slot_3_2,
            (3, 1) => self.slot_3_1,
            (4, 2) => self.slot_4_2,
            (4, 1) => self.slot_4_1,
            _ => unreachable!(),
        }
    }

    fn set_slot(&mut self, slot_num: u8, pos: u8, el: u8) {
        match (slot_num, pos) {
            (1, 1) => self.slot_1_1 = el,
            (1, 2) => self.slot_1_2 = el,
            (2, 1) => self.slot_2_1 = el,
            (2, 2) => self.slot_2_2 = el,
            (3, 1) => self.slot_3_1 = el,
            (3, 2) => self.slot_3_2 = el,
            (4, 1) => self.slot_4_1 = el,
            (4, 2) => self.slot_4_2 = el,
            _ => unreachable!(),
        }
    }

    fn clone_with_slot(&self, slot_num: u8, pos: u8, el: u8) -> Board {
        let mut b1 = self.clone();
        b1.set_slot(slot_num, pos, el);
        b1
    }

    fn final_board(&self) -> Board {
        let mut ret = Board::default();
        ret.depth = self.depth;
        for slot_num in 1..=4 {
            for pos in 1..=ret.depth {
                ret.set_slot(slot_num, pos, slot_num);
            }
        }
        ret
    }

    fn is_final_board(&self) -> bool {
        (1..=4).all(|slot_num| (1..=self.depth).all(|pos| self.get_slot(slot_num, pos) == slot_num))
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "|{}{} {}{} {} {}{} {} {}{} {} {}{} {}{}|",
            board_to_char(self.hallway_0),
            board_to_char(self.hallway_1),
            board_to_char(self.slot_1_1),
            board_to_char(self.slot_1_2),
            board_to_char(self.hallway_1_2),
            board_to_char(self.slot_2_1),
            board_to_char(self.slot_2_2),
            board_to_char(self.hallway_2_3),
            board_to_char(self.slot_3_1),
            board_to_char(self.slot_3_2),
            board_to_char(self.hallway_3_4),
            board_to_char(self.slot_4_1),
            board_to_char(self.slot_4_2),
            board_to_char(self.hallway_2),
            board_to_char(self.hallway_3)
        ))
    }
}

#[test]
fn test_next_moves() {
    let b = z("..[AB].[DC].[CB].[AD]..");
    assert_eq!(
        next_moves(&b, 0)
            .into_iter()
            .map(|x| x.0)
            .collect::<HashSet<_>>(),
        HashSet::from([
            z("A.[.B].[DC].[CB].[AD].."),
            z(".A[.B].[DC].[CB].[AD].."),
            z("..[.B]A[DC].[CB].[AD].."),
            z("..[.B].[DC]A[CB].[AD].."),
            z("..[.B].[DC].[CB]A[AD].."),
            z("..[.B].[DC].[CB].[AD]A."),
            z("..[.B].[DC].[CB].[AD].A"),
            z("D.[AB].[.C].[CB].[AD].."),
            z(".D[AB].[.C].[CB].[AD].."),
            z("..[AB]D[.C].[CB].[AD].."),
            z("..[AB].[.C]D[CB].[AD].."),
            z("..[AB].[.C].[CB]D[AD].."),
            z("..[AB].[.C].[CB].[AD]D."),
            z("..[AB].[.C].[CB].[AD].D"),
            z("C.[AB].[DC].[.B].[AD].."),
            z(".C[AB].[DC].[.B].[AD].."),
            z("..[AB]C[DC].[.B].[AD].."),
            z("..[AB].[DC]C[.B].[AD].."),
            z("..[AB].[DC].[.B]C[AD].."),
            z("..[AB].[DC].[.B].[AD]C."),
            z("..[AB].[DC].[.B].[AD].C"),
            z("A.[AB].[DC].[CB].[.D].."),
            z(".A[AB].[DC].[CB].[.D].."),
            z("..[AB]A[DC].[CB].[.D].."),
            z("..[AB].[DC]A[CB].[.D].."),
            z("..[AB].[DC].[CB]A[.D].."),
            z("..[AB].[DC].[CB].[.D]A."),
            z("..[AB].[DC].[CB].[.D].A"),
        ])
    );
    let b = z("..[.B].[.D].[.A].[.A]..");
    assert_eq!(
        next_moves(&b, 0)
            .into_iter()
            .map(|x| x.0)
            .collect::<HashSet<_>>(),
        HashSet::from([
            z("B.[..].[.D].[.A].[.A].."),
            z(".B[..].[.D].[.A].[.A].."),
            z("..[..]B[.D].[.A].[.A].."),
            z("..[..].[.D]B[.A].[.A].."),
            z("..[..].[.D].[.A]B[.A].."),
            z("..[..].[.D].[.A].[.A]B."),
            z("..[..].[.D].[.A].[.A].B"),
            z("D.[.B].[..].[.A].[.A].."),
            z(".D[.B].[..].[.A].[.A].."),
            z("..[.B]D[..].[.A].[.A].."),
            z("..[.B].[..]D[.A].[.A].."),
            z("..[.B].[..].[.A]D[.A].."),
            z("..[.B].[..].[.A].[.A]D."),
            z("..[.B].[..].[.A].[.A].D"),
            z("A.[.B].[.D].[..].[.A].."),
            z(".A[.B].[.D].[..].[.A].."),
            z("..[.B]A[.D].[..].[.A].."),
            z("..[.B].[.D]A[..].[.A].."),
            z("..[.B].[.D].[..]A[.A].."),
            z("..[.B].[.D].[..].[.A]A."),
            z("..[.B].[.D].[..].[.A].A"),
            z("A.[.B].[.D].[.A].[..].."),
            z(".A[.B].[.D].[.A].[..].."),
            z("..[.B]A[.D].[.A].[..].."),
            z("..[.B].[.D]A[.A].[..].."),
            z("..[.B].[.D].[.A]A[..].."),
            z("..[.B].[.D].[.A].[..]A."),
            z("..[.B].[.D].[.A].[..].A"),
        ])
    );
    let b = z("..[D.]D[..]A[..].[..]..");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([
            (z(".D[..]D[..]A[..].[..].."), 2 * 1000),
            (z("D.[..]D[..]A[..].[..].."), 3 * 1000),
        ])
    );
    let b = z("..[..]C[..]A[..].[A.]..");
    assert_eq!(
        next_moves(&b, 0)
            .into_iter()
            .map(|x| x.0)
            .collect::<HashSet<_>>(),
        HashSet::from([
            z("..[..]C[..]A[..].[..]A."),
            z("..[..]C[..]A[..].[..].A"),
            z("..[..]C[..]A[..]A[..].."),
        ])
    );
    let b = z("..[..]C[..]A[..].[..]D.");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([(z("..[..]C[..]A[..].[.D].."), 3 * 1000),])
    );
    let b = z("..[..]C[..]A[..].[.A]D.");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([(z("..[..]C[..]A[..]A[..]D."), 3 * 1),])
    );
    let b = z("..[..]D[..].[..]A[.A]D.");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([])
    );
    let b = z("..[..]C[..].[..]A[.A]D.");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([(z("..[..].[..].[.C]A[.A]D."), 5 * 100)])
    );
    let b = z("..[..]A[..]B[..]C[..]D.");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([
            (z("..[.A].[..]B[..]C[..]D."), 3 * 1),
            (z("..[..]A[.B].[..]C[..]D."), 3 * 10),
            (z("..[..]A[..]B[.C].[..]D."), 3 * 100),
            (z("..[..]A[..]B[..]C[.D].."), 3 * 1000),
        ])
    );
    let b = z("..[.A].[.B].[.C].[.D]..");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([])
    );
    let b = z("..[AA].[BB].[CC].[DD]..");
    assert_eq!(
        next_moves(&b, 0).into_iter().collect::<HashSet<_>>(),
        HashSet::from([])
    );
}

fn next_moves(b: &Board, c: u32) -> Vec<(Board, u32)> {
    let mut ret = vec![];
    // move out of slots
    for slot_num in 1..=4 {
        if b.get_slot(slot_num, 1) > 0 {
            let el = b.get_slot(slot_num, 1);
            let b1 = b.clone_with_slot(slot_num, 1, 0);
            if el != slot_num || b.get_slot(slot_num, 2) != slot_num {
                move_out_of_slot_num(slot_num, b1, &mut ret, el, c);
            }
        } else if b.get_slot(slot_num, 2) > 0 {
            let el = b.get_slot(slot_num, 2);
            let b1 = b.clone_with_slot(slot_num, 2, 0);
            if el != slot_num {
                move_out_of_slot_num(slot_num, b1, &mut ret, el, c + cost(el));
            }
        }
    }
    // move into slots
    if b.hallway_0 > 0 && b.hallway_1 == 0 {
        let el = b.hallway_0;
        let b1 = Board { hallway_0: 0, ..*b };
        if el == 1 {
            move_into_slot(&b1, 1, el, &mut ret, c + 3 * cost(el));
        }
        if el == 2 && b1.hallway_1_2 == 0 {
            move_into_slot(&b1, 2, el, &mut ret, c + 5 * cost(el));
        }
        if el == 3 && b1.hallway_1_2 == 0 && b1.hallway_2_3 == 0 {
            move_into_slot(&b1, 3, el, &mut ret, c + 7 * cost(el));
        }
        if el == 4 && b1.hallway_1_2 == 0 && b1.hallway_2_3 == 0 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 4, el, &mut ret, c + 9 * cost(el));
        }
    }
    if b.hallway_1 > 0 {
        let el = b.hallway_1;
        let b1 = Board { hallway_1: 0, ..*b };
        if el == 1 {
            move_into_slot(&b1, 1, el, &mut ret, c + 2 * cost(el));
        }
        if el == 2 && b1.hallway_1_2 == 0 {
            move_into_slot(&b1, 2, el, &mut ret, c + 4 * cost(el));
        }
        if el == 3 && b1.hallway_1_2 == 0 && b1.hallway_2_3 == 0 {
            move_into_slot(&b1, 3, el, &mut ret, c + 6 * cost(el));
        }
        if el == 4 && b1.hallway_1_2 == 0 && b1.hallway_2_3 == 0 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 4, el, &mut ret, c + 8 * cost(el));
        }
    }
    if b.hallway_2 > 0 {
        let el = b.hallway_2;
        let b1 = Board { hallway_2: 0, ..*b };
        if el == 4 {
            move_into_slot(&b1, 4, el, &mut ret, c + 2 * cost(el));
        }
        if el == 3 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 3, el, &mut ret, c + 4 * cost(el));
        }
        if el == 2 && b1.hallway_2_3 == 0 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 2, el, &mut ret, c + 6 * cost(el));
        }
        if el == 1 && b1.hallway_1_2 == 0 && b1.hallway_2_3 == 0 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 1, el, &mut ret, c + 8 * cost(el));
        }
    }
    if b.hallway_3 > 0 && b.hallway_2 == 0 {
        let el = b.hallway_3;
        let b1 = Board { hallway_3: 0, ..*b };
        if el == 4 {
            move_into_slot(&b1, 4, el, &mut ret, c + 3 * cost(el));
        }
        if el == 3 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 3, el, &mut ret, c + 5 * cost(el));
        }
        if el == 2 && b1.hallway_2_3 == 0 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 2, el, &mut ret, c + 7 * cost(el));
        }
        if el == 1 && b1.hallway_1_2 == 0 && b1.hallway_2_3 == 0 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 1, el, &mut ret, c + 9 * cost(el));
        }
    }
    if b.hallway_1_2 > 0 {
        let el = b.hallway_1_2;
        let b1 = Board {
            hallway_1_2: 0,
            ..*b
        };
        if el == 1 {
            move_into_slot(&b1, 1, el, &mut ret, c + 2 * cost(el));
        }
        if el == 2 {
            move_into_slot(&b1, 2, el, &mut ret, c + 2 * cost(el));
        }
        if el == 3 && b1.hallway_2_3 == 0 {
            move_into_slot(&b1, 3, el, &mut ret, c + 4 * cost(el));
        }
        if el == 4 && b1.hallway_2_3 == 0 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 4, el, &mut ret, c + 6 * cost(el));
        }
    }
    if b.hallway_2_3 > 0 {
        let el = b.hallway_2_3;
        let b1 = Board {
            hallway_2_3: 0,
            ..*b
        };
        if el == 1 && b1.hallway_1_2 == 0 {
            move_into_slot(&b1, 1, el, &mut ret, c + 4 * cost(el));
        }
        if el == 2 {
            move_into_slot(&b1, 2, el, &mut ret, c + 2 * cost(el));
        }
        if el == 3 {
            move_into_slot(&b1, 3, el, &mut ret, c + 2 * cost(el));
        }
        if el == 4 && b1.hallway_3_4 == 0 {
            move_into_slot(&b1, 4, el, &mut ret, c + 4 * cost(el));
        }
    }
    if b.hallway_3_4 > 0 {
        let el = b.hallway_3_4;
        let b1 = Board {
            hallway_3_4: 0,
            ..*b
        };
        if el == 1 && b1.hallway_2_3 == 0 && b1.hallway_1_2 == 0 {
            move_into_slot(&b1, 1, el, &mut ret, c + 6 * cost(el));
        }
        if el == 2 && b1.hallway_2_3 == 0 {
            move_into_slot(&b1, 2, el, &mut ret, c + 4 * cost(el));
        }
        if el == 3 {
            move_into_slot(&b1, 3, el, &mut ret, c + 2 * cost(el));
        }
        if el == 4 {
            move_into_slot(&b1, 4, el, &mut ret, c + 2 * cost(el));
        }
    }
    ret
}

#[test]
fn test_cost() {
    assert_eq!(cost(1), 1);
    assert_eq!(cost(4), 1000)
}

fn cost(el: u8) -> u32 {
    10u32.pow(el as u32 - 1)
}

fn move_into_slot(b1: &Board, slot_num: u8, el: u8, ret: &mut Vec<(Board, u32)>, c: u32) {
    if b1.get_slot(slot_num, 1) == 0 && slot_num == el {
        if b1.get_slot(slot_num, 2) == 0 {
            ret.push((b1.clone_with_slot(slot_num, 2, el), c + cost(el)));
        } else if b1.get_slot(slot_num, 2) == el {
            ret.push((b1.clone_with_slot(slot_num, 1, el), c));
        }
    }
}

fn move_out_of_slot_num(slot_num: u8, b1: Board, ret: &mut Vec<(Board, u32)>, el: u8, c: u32) {
    match slot_num {
        1 => move_out_of_slot_1(b1, ret, el, c),
        2 => move_out_of_slot_2(b1, ret, el, c),
        3 => move_out_of_slot_3(b1, ret, el, c),
        4 => move_out_of_slot_4(b1, ret, el, c),
        _ => unreachable!(),
    }
}

fn move_out_of_slot_1(b1: Board, ret: &mut Vec<(Board, u32)>, el: u8, c: u32) {
    move_to_hallway_low(&b1, ret, el, c + 2 * cost(el));
    if b1.hallway_1_2 == 0 {
        ret.push((
            Board {
                hallway_1_2: el,
                ..b1
            },
            c + 2 * cost(el),
        ));
        if b1.hallway_2_3 == 0 {
            ret.push((
                Board {
                    hallway_2_3: el,
                    ..b1
                },
                c + 4 * cost(el),
            ));
            if b1.hallway_3_4 == 0 {
                ret.push((
                    Board {
                        hallway_3_4: el,
                        ..b1
                    },
                    c + 6 * cost(el),
                ));
                move_to_hallway_high(b1, ret, el, c + 8 * cost(el));
            }
        }
    }
}

fn move_out_of_slot_2(b1: Board, ret: &mut Vec<(Board, u32)>, el: u8, c: u32) {
    if b1.hallway_1_2 == 0 {
        ret.push((
            Board {
                hallway_1_2: el,
                ..b1
            },
            c + 2 * cost(el),
        ));
        move_to_hallway_low(&b1, ret, el, c + 4 * cost(el));
    }
    if b1.hallway_2_3 == 0 {
        ret.push((
            Board {
                hallway_2_3: el,
                ..b1
            },
            c + 2 * cost(el),
        ));
        if b1.hallway_3_4 == 0 {
            ret.push((
                Board {
                    hallway_3_4: el,
                    ..b1
                },
                c + 4 * cost(el),
            ));
            move_to_hallway_high(b1, ret, el, c + 6 * cost(el));
        }
    }
}

fn move_out_of_slot_3(b1: Board, ret: &mut Vec<(Board, u32)>, el: u8, c: u32) {
    if b1.hallway_2_3 == 0 {
        ret.push((
            Board {
                hallway_2_3: el,
                ..b1
            },
            c + 2 * cost(el),
        ));
        if b1.hallway_1_2 == 0 {
            ret.push((
                Board {
                    hallway_1_2: el,
                    ..b1
                },
                c + 4 * cost(el),
            ));
            move_to_hallway_low(&b1, ret, el, c + 6 * cost(el));
        }
    }
    if b1.hallway_3_4 == 0 {
        ret.push((
            Board {
                hallway_3_4: el,
                ..b1
            },
            c + 2 * cost(el),
        ));
        move_to_hallway_high(b1, ret, el, c + 4 * cost(el));
    }
}

fn move_out_of_slot_4(b1: Board, ret: &mut Vec<(Board, u32)>, el: u8, c: u32) {
    if b1.hallway_3_4 == 0 {
        ret.push((
            Board {
                hallway_3_4: el,
                ..b1
            },
            c + 2 * cost(el),
        ));
        if b1.hallway_2_3 == 0 {
            ret.push((
                Board {
                    hallway_2_3: el,
                    ..b1
                },
                c + 4 * cost(el),
            ));
            if b1.hallway_1_2 == 0 {
                ret.push((
                    Board {
                        hallway_1_2: el,
                        ..b1
                    },
                    c + 6 * cost(el),
                ));
                move_to_hallway_low(&b1, ret, el, c + 8 * cost(el));
            }
        }
    }
    move_to_hallway_high(b1, ret, el, c + 2 * cost(el));
}

fn move_to_hallway_high(b1: Board, ret: &mut Vec<(Board, u32)>, el: u8, c: u32) {
    if b1.hallway_2 == 0 {
        ret.push((
            Board {
                hallway_2: el,
                ..b1
            },
            c,
        ));
        if b1.hallway_3 == 0 {
            ret.push((
                Board {
                    hallway_3: el,
                    ..b1
                },
                c + cost(el),
            ));
        }
    }
}

fn move_to_hallway_low(b1: &Board, ret: &mut Vec<(Board, u32)>, el: u8, c: u32) {
    if b1.hallway_1 == 0 {
        ret.push((
            Board {
                hallway_1: el,
                ..*b1
            },
            c,
        ));
        if b1.hallway_0 == 0 {
            ret.push((
                Board {
                    hallway_0: el,
                    ..*b1
                },
                c + cost(el),
            ));
        }
    }
}

/// #############
/// #...........#
/// ###B#C#B#D###
///   #A#D#C#A#
///   #########
pub fn part1(input: String) -> u32 {
    let board = parse(input.as_ref());
    let ret = explore_recur(board, 0, &mut HashMap::new()).unwrap();
    dbg!(ret.1);
    ret.0
}

fn explore_recur(
    b: Board,
    c: u32,
    memoize: &mut HashMap<Board, u32>,
) -> Option<(u32, Vec<(Board, u32)>)> {
    if b.is_final_board() {
        return Some((c, vec![(b, c)]));
    }

    let final_distance = memoize.get(&b.final_board());
    if let Some(&final_distance) = final_distance {
        if final_distance < c {
            return None;
        }
    }

    let min_distance = memoize.get(&b);
    if let Some(&min_distance) = min_distance {
        if min_distance < c {
            return None;
        }
    }
    memoize.insert(b.clone(), c);

    let mut moves = next_moves(&b, c);
    moves.sort_by(|m1, m2| m1.1.cmp(&m2.1));
    moves
        .into_iter()
        .map(|(m, c)| explore_recur(m, c, memoize))
        .flatten()
        .min_by(|r1, r2| r1.0.cmp(&r2.0))
        .map(|(d, path)| {
            let mut ret = vec![(b, c)];
            ret.extend(path);
            (d, ret)
        })
}

fn parse(input: &str) -> Board {
    let mut board = Board::default();
    board.depth = 2;
    for (i, line) in input
        .split_terminator('\n')
        .skip(1)
        .enumerate()
        .take(board.depth as usize + 1)
    {
        if i == 0 {
            let mut c = line.chars().skip(1);
            board.hallway_0 = parse_char(c.next().unwrap());
            board.hallway_1 = parse_char(c.next().unwrap());
            let mut c = c.skip(1);
            board.hallway_1_2 = parse_char(c.next().unwrap());
            let mut c = c.skip(1);
            board.hallway_2_3 = parse_char(c.next().unwrap());
            let mut c = c.skip(1);
            board.hallway_3_4 = parse_char(c.next().unwrap());
            let mut c = c.skip(1);
            board.hallway_2 = parse_char(c.next().unwrap());
            board.hallway_3 = parse_char(c.next().unwrap());
        } else {
            let c: Vec<_> = line.chars().skip(3).collect();
            let pos = i as u8;
            for i in 0..4 {
                let slot_num = i + 1;
                board.set_slot(slot_num, pos, parse_char(c[i as usize * 2]));
            }
        }
    }
    board
}

#[cfg(test)]
fn z(b: &str) -> Board {
    let mut board = Board::default();
    board.depth = 2;
    let mut c = b.chars();
    board.hallway_0 = parse_char(c.next().unwrap());
    board.hallway_1 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), '[');
    board.slot_1_1 = parse_char(c.next().unwrap());
    board.slot_1_2 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), ']');
    board.hallway_1_2 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), '[');
    board.slot_2_1 = parse_char(c.next().unwrap());
    board.slot_2_2 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), ']');
    board.hallway_2_3 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), '[');
    board.slot_3_1 = parse_char(c.next().unwrap());
    board.slot_3_2 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), ']');
    board.hallway_3_4 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), '[');
    board.slot_4_1 = parse_char(c.next().unwrap());
    board.slot_4_2 = parse_char(c.next().unwrap());
    assert_eq!(c.next().unwrap(), ']');
    board.hallway_2 = parse_char(c.next().unwrap());
    board.hallway_3 = parse_char(c.next().unwrap());
    assert_eq!(c.next(), None);
    board
}

fn board_to_char(c: u8) -> char {
    match c {
        1 => 'A',
        2 => 'B',
        3 => 'C',
        4 => 'D',
        0 => '.',
        _ => unreachable!(),
    }
}

fn parse_char(c: char) -> u8 {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        '.' => 0,
        _ => unreachable!(),
    }
}
