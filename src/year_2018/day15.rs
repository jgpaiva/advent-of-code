use std::collections::{HashSet, VecDeque};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2018/test_day15-1");
    assert_eq!(part1(input), (27730, vec![200, 131, 59, 200], false));
    let input = utils::read_file("2018/test_day15-2");
    assert_eq!(part1(input), (36334, vec![200, 197, 185, 200, 200], true));
    let input = utils::read_file("2018/test_day15-3");
    assert_eq!(part1(input), (39514, vec![164, 197, 200, 98, 200], true));
    let input = utils::read_file("2018/test_day15-4");
    assert_eq!(part1(input), (27755, vec![200, 98, 200, 95, 200], false));
    let input = utils::read_file("2018/test_day15-5");
    assert_eq!(part1(input), (28944, vec![200, 98, 38, 200], false));
    let input = utils::read_file("2018/test_day15-6");
    assert_eq!(part1(input), (18740, vec![137, 200, 200, 200, 200], false));
    let input = utils::read_file("2018/test_day15-1");
    assert_eq!(part2(input), (4988, vec![158, 14], 15));
}

pub fn part2(input: String) -> (u32, Vec<u8>, u8) {
    for i in 3..100 {
        let res = run_sim(input.clone(), i);
        if res.2 {
            return (res.0, res.1, i);
        }
    }
    unreachable!();
}

pub fn part1(input: String) -> (u32, Vec<u8>, bool) {
    run_sim(input, 3)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum El {
    Wall,
    Goblin(u8, u8),
    Elf(u8, u8),
    Empty,
}

#[derive(Debug, Clone)]
struct BoardEl {
    p: (usize, usize),
    el: El,
}

struct Board {
    b: Vec<BoardEl>,
    height: usize,
    width: usize,
    elf_attack_power: u8,
}

enum MoveOutcome {
    CombatEnd,
    EnemyAdjacent,
    MoveTo(Option<(usize, usize)>),
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut stats = vec![];
        let board = self
            .b
            .iter()
            .map(|b| match b.el {
                El::Wall => '#',
                El::Goblin(h, _) => {
                    stats.push(format!("G{}", h));
                    'G'
                }
                El::Elf(h, _) => {
                    stats.push(format!("E{}", h));
                    'E'
                }
                El::Empty => '.',
            })
            .collect::<Vec<_>>();
        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| board[y * self.width + x])
                    .collect::<String>()
            })
            .chain([stats.join(" ")].into_iter())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Board {
    fn new(board: Vec<BoardEl>, elf_attack_power: u8) -> Self {
        let height = board.iter().map(|el| el.p.1).max().unwrap() + 1;
        let width = board.iter().map(|el| el.p.0).max().unwrap() + 1;
        Board {
            b: board,
            height,
            width,
            elf_attack_power,
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> &BoardEl {
        &self.b[y * self.width + x]
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut BoardEl {
        &mut self.b[y * self.width + x]
    }

    fn adjacent_empty_positions(&self, p: (usize, usize)) -> Vec<&BoardEl> {
        self.neighbours(p)
            .into_iter()
            .filter(|bel| matches!(bel.el, El::Empty))
            .collect()
    }

    fn is_enemy_adjacent(&self, unit: &BoardEl) -> bool {
        self.neighbours(unit.p).iter().any(|bel| {
            matches!(
                (unit.el.clone(), bel.el.clone()),
                (El::Goblin(_, _), El::Elf(_, _)) | (El::Elf(_, _), El::Goblin(_, _))
            )
        })
    }

    fn distance(&self, p1: (usize, usize), p2: (usize, usize)) -> Option<(usize, (usize, usize))> {
        self.neighbours(p1)
            .into_iter()
            .map(|d| (self.distance_aux(p1, p2, d.p), d.p))
            .filter(|(d, _)| d.is_some())
            .map(|(d, p)| (d.unwrap(), p))
            .min_by(|(d1, _), (d2, _)| d1.cmp(d2))
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<&BoardEl> {
        let left = self.get((x - 1, y));
        let right = self.get((x + 1, y));
        let up = self.get((x, y - 1));
        let down = self.get((x, y + 1));
        vec![up, left, right, down]
    }

    fn distance_aux(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        start_at: (usize, usize),
    ) -> Option<usize> {
        let mut seen: HashSet<(usize, usize)> = HashSet::from([from]);
        let mut next: VecDeque<(&BoardEl, usize)> = VecDeque::from([(self.get(start_at), 1)]);
        while let Some((neighbour, distance)) = next.pop_front() {
            if neighbour.p == to {
                return Some(distance);
            }
            if !matches!(neighbour.el, El::Empty) {
                continue;
            }
            if !seen.insert(neighbour.p) {
                continue;
            }
            for neighbour in self.neighbours(neighbour.p) {
                next.push_back((neighbour, distance + 1));
            }
        }
        None
    }

    fn move_unit(&self, unit: &BoardEl) -> MoveOutcome {
        let targets = self
            .b
            .iter()
            .filter(|bel| {
                matches!(
                    (unit.el.clone(), bel.el.clone()),
                    (El::Goblin(_, _), El::Elf(_, _)) | (El::Elf(_, _), El::Goblin(_, _))
                )
            })
            .cloned();
        if targets.clone().count() == 0 {
            return MoveOutcome::CombatEnd;
        }
        if self.is_enemy_adjacent(unit) {
            return MoveOutcome::EnemyAdjacent;
        }
        let in_range = targets.flat_map(|bel| self.adjacent_empty_positions(bel.p).into_iter());
        MoveOutcome::MoveTo(
            in_range
                .map(|space| (self.distance(unit.p, space.p), space))
                .filter(|(d, _)| d.is_some())
                .map(|(d, s)| (d.unwrap(), s))
                .min_by(|(d1, s1), (d2, s2)| (d1.0, s1.p.1, s1.p.0).cmp(&(d2.0, s2.p.1, s2.p.0)))
                .map(|(d, _)| d.1),
        )
    }
}

#[test]
fn test_distance() {
    let input = r#"#########
#G..G.#G#
#E.....##
#.......#
#G..E..G#
#.#.....#
#.#..E#.#
#G..G.#G#
#########"#;
    let board = parse_board(input.to_string(), 3);
    let g = board.get((1, 1));
    assert_eq!(g.el, El::Goblin(200, 1));
    let res = board.distance((1, 1), (4, 4));
    assert_eq!(res.unwrap(), (6, (2, 1)));
    let res = board.distance((1, 1), (1, 2));
    assert_eq!(res.unwrap(), (1, (1, 2)));
    let res = board.distance((1, 1), (2, 2));
    assert_eq!(res.unwrap(), (2, (2, 1)));
    let res = board.distance((7, 1), (4, 4));
    assert_eq!(res, None);
    let res = board.distance((7, 7), (4, 6));
    assert_eq!(res.unwrap(), (6, (7, 6)));
    let res = board.distance((1, 7), (4, 4));
    assert_eq!(res.unwrap(), (6, (2, 7)));
}

fn run_sim(input: String, elf_attack_power: u8) -> (u32, Vec<u8>, bool) {
    let mut board = parse_board(input, elf_attack_power);
    for i in 0..100 {
        let mut combat_end = false;
        let mut elf_died = false;
        for bel in board
            .b
            .clone()
            .into_iter()
            .filter(|bel| matches!(bel.el, El::Elf(_, _) | El::Goblin(_, _)))
        {
            // get the unit's current state
            let unit = board.get(bel.p).clone();
            match (bel.el, unit.el.clone()) {
                (El::Goblin(_, id1), El::Goblin(_, id2)) if id1 == id2 => (),
                (El::Elf(_, id1), El::Elf(_, id2)) if id1 == id2 => (),
                _ => continue,
            }
            // first try to move
            let move_to = board.move_unit(&unit);
            if let MoveOutcome::CombatEnd = move_to {
                combat_end = true;
                break;
            }
            let p = if let MoveOutcome::MoveTo(Some(move_to_p)) = move_to {
                let move_to = board.get_mut(move_to_p);
                assert_eq!(move_to.el, El::Empty);
                move_to.el = unit.el.clone();
                board.get_mut(unit.p).el = El::Empty;
                move_to_p
            } else {
                unit.p
            };
            // try to attack
            let neighbour_enemy = board
                .neighbours(p)
                .into_iter()
                .filter(|bel| {
                    matches!(
                        (unit.el.clone(), bel.el.clone()),
                        (El::Goblin(_, _), El::Elf(_, _)) | (El::Elf(_, _), El::Goblin(_, _))
                    )
                })
                .min_by(|e1, e2| (e1.health(), e1.p.1, e1.p.0).cmp(&(e2.health(), e2.p.1, e2.p.0)));
            if let Some(neighbour_enemy) = neighbour_enemy {
                let neighbour_enemy = neighbour_enemy.clone();
                let elf_attack_power = board.elf_attack_power;
                elf_died = board.get_mut(neighbour_enemy.p).hit(elf_attack_power) || elf_died;
            }
        }
        //println!("{}", board.to_string());
        if combat_end || (elf_attack_power > 3 && elf_died) {
            return (
                i * board
                    .b
                    .iter()
                    .map(|b| match b.el {
                        El::Wall => 0,
                        El::Goblin(h, _) => h,
                        El::Elf(h, _) => h,
                        El::Empty => 0,
                    } as u32)
                    .sum::<u32>(),
                board
                    .b
                    .iter()
                    .map(|b| match b.el {
                        El::Wall => None,
                        El::Goblin(h, _) => Some(h),
                        El::Elf(h, _) => Some(h),
                        El::Empty => None,
                    })
                    .flatten()
                    .collect(),
                !elf_died
                    && board
                        .b
                        .iter()
                        .map(|b| match b.el {
                            El::Wall => None,
                            El::Goblin(_, _) => Some(false),
                            El::Elf(_, _) => Some(true),
                            El::Empty => None,
                        })
                        .flatten()
                        .next()
                        .unwrap(),
            );
        }
    }
    unreachable!();
}

#[test]
fn test_move() {
    let input = r#"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#;
    let mut board = parse_board(input.to_string(), 3);
    let mut moves = vec![];
    for i in 0..100 {
        let mut moved = false;
        for unit in board.b.clone().into_iter() {
            let move_to = match unit.el {
                El::Goblin(_, _) => board.move_unit(&unit),
                El::Elf(_, _) => board.move_unit(&unit),
                El::Wall | El::Empty => continue,
            };
            if let MoveOutcome::MoveTo(Some(move_to)) = move_to {
                moves.push((unit.p, move_to));
                moved = true;
                assert_eq!(board.get(move_to).el, El::Empty);
                board.get_mut(move_to).el = unit.el;
                board.get_mut(unit.p).el = El::Empty;
            }
        }
        if !moved {
            assert_eq!(i, 3);
            assert_eq!(
                moves,
                vec![
                    ((1, 1), (2, 1)),
                    ((4, 1), (4, 2)),
                    ((7, 1), (6, 1)),
                    ((1, 4), (2, 4)),
                    ((4, 4), (4, 3)),
                    ((7, 4), (7, 3)),
                    ((1, 7), (1, 6)),
                    ((4, 7), (4, 6)),
                    ((7, 7), (7, 6)),
                    ((2, 1), (3, 1)),
                    ((6, 1), (5, 1)),
                    ((7, 3), (6, 3)),
                    ((2, 4), (2, 3)),
                    ((1, 6), (1, 5)),
                    ((4, 6), (4, 5)),
                    ((7, 6), (7, 5)),
                    ((3, 1), (3, 2)),
                    ((5, 1), (5, 2)),
                    ((2, 3), (3, 3)),
                    ((6, 3), (5, 3)),
                    ((1, 5), (1, 4)),
                    ((4, 5), (4, 4))
                ]
            );
            break;
        }
    }
}

fn parse_board(input: String, elf_attack_power: u8) -> Board {
    let mut counter = 0;
    let board: Vec<BoardEl> = input
        .split_terminator('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let el = match c {
                    '#' => El::Wall,
                    '.' => El::Empty,
                    'G' => El::Goblin(200, {
                        counter += 1;
                        counter
                    }),
                    'E' => El::Elf(200, {
                        counter += 1;
                        counter
                    }),
                    v => unreachable!(v),
                };
                BoardEl { p: (x, y), el }
            })
        })
        .collect();
    Board::new(board, elf_attack_power)
}
impl BoardEl {
    fn health(&self) -> u8 {
        match self.el {
            El::Wall => unreachable!(),
            El::Goblin(h, _) => h,
            El::Elf(h, _) => h,
            El::Empty => unreachable!(),
        }
    }

    fn hit(&mut self, mut hit_points: u8) -> bool {
        if let El::Elf(_, _) = self.el {
            hit_points = 3;
        }
        let health = self.health();
        let mut elf_died = false;
        self.el = if health > hit_points {
            match self.el {
                El::Goblin(h, id) => El::Goblin(h - hit_points, id),
                El::Elf(h, id) => El::Elf(h - hit_points, id),
                _ => unreachable!(),
            }
        } else {
            if let El::Elf(_, _) = self.el {
                elf_died = true;
            }
            El::Empty
        };
        elf_died
    }
}
