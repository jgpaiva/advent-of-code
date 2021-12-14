use std::collections::{HashSet, VecDeque};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2018/test_day15");
    if false {
        assert_eq!(part1(input), 18740);
    }
}

#[allow(unused_variables)]
pub fn part2(input: String) -> u32 {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum El {
    Wall,
    Goblin(u8),
    Elf(u8),
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct BoardEl {
    p: (usize, usize),
    el: El,
}

struct Board {
    b: Vec<BoardEl>,
    height: usize,
}

impl Board {
    fn new(board: Vec<BoardEl>) -> Self {
        let height = board.iter().map(|el| el.p.1).max().unwrap() + 1;
        Board { b: board, height }
    }

    fn get(&self, (x, y): (usize, usize)) -> BoardEl {
        self.b[y * self.height + x]
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut BoardEl {
        &mut self.b[y * self.height + x]
    }

    fn adjacent_empty_positions(&self, p: (usize, usize)) -> Vec<BoardEl> {
        self.neighbours(p)
            .into_iter()
            .filter(|bel| matches!(bel.el, El::Empty))
            .collect()
    }

    fn is_enemy_adjacent(&self, unit: BoardEl) -> bool {
        self.neighbours(unit.p).iter().any(|bel| {
            matches!(
                (unit.el, bel.el),
                (El::Goblin(_), El::Elf(_)) | (El::Elf(_), El::Goblin(_))
            )
        })
    }

    fn distance(&self, bel1: BoardEl, bel2: BoardEl) -> Option<(usize, (usize, usize))> {
        self.neighbours(bel1.p)
            .into_iter()
            .map(|d| (self.distance_aux(bel1, bel2, d.p), d.p))
            .filter(|(d, _)| d.is_some())
            .map(|(d, p)| (d.unwrap(), p))
            .min_by(|(d1, _), (d2, _)| d1.cmp(d2))
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<BoardEl> {
        let left = self.get((x - 1, y));
        let right = self.get((x + 1, y));
        let up = self.get((x, y - 1));
        let down = self.get((x, y + 1));
        vec![up, left, right, down]
    }

    fn distance_aux(&self, from: BoardEl, to: BoardEl, start_at: (usize, usize)) -> Option<usize> {
        let mut seen: HashSet<(usize, usize)> = HashSet::from([from.p]);
        let mut next: VecDeque<(BoardEl, usize)> = VecDeque::from([(self.get(start_at), 1)]);
        while let Some((neighbour, distance)) = next.pop_front() {
            if neighbour.p == to.p {
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

    fn move_unit(&self, unit: BoardEl) -> Option<(usize, usize)> {
        let targets = self
            .b
            .iter()
            .filter(|bel| {
                matches!(
                    (unit.el, bel.el),
                    (El::Goblin(_), El::Elf(_)) | (El::Elf(_), El::Goblin(_))
                )
            })
            .cloned();
        // sort by distance and then by reading order
        // targets.sort_by(|e1, e2| (e1.0, e1.1.p.1, e1.1.p.0).cmp(&(e2.0, e2.1.p.1, e2.1.p.0)));
        if targets.clone().count() == 0 {
            panic!("no enemies");
        }
        if self.is_enemy_adjacent(unit) {
            return None;
        }
        let in_range = targets.flat_map(|bel| {
            self.adjacent_empty_positions(bel.p)
                .into_iter()
                .map(move |pos| (pos, bel))
        });
        in_range
            .map(|(space, bel)| (self.distance(unit, space), space, bel))
            .filter(|(d, _, _)| d.is_some())
            .map(|(d, s, b)| (d.unwrap(), s, b))
            .min_by(|(d1, s1, _), (d2, s2, _)| {
                (d1.0, s1.p.1, s1.p.0, d1.1 .1, d1.1 .0)
                    .cmp(&(d2.0, s2.p.1, s2.p.0, d2.1 .1, d2.1 .0))
            })
            .map(|(d, _, _)| d.1)
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
    let board = parse_board(input.to_string());
    let g = board.get((1, 1));
    assert_eq!(g.el, El::Goblin(200));
    let res = board.distance(board.get((1, 1)), board.get((4, 4)));
    assert_eq!(res.unwrap(), (6, (2, 1)));
    let res = board.distance(board.get((1, 1)), board.get((1, 2)));
    assert_eq!(res.unwrap(), (1, (1, 2)));
    let res = board.distance(board.get((1, 1)), board.get((2, 2)));
    assert_eq!(res.unwrap(), (2, (2, 1)));
    let res = board.distance(board.get((7, 1)), board.get((4, 4)));
    assert_eq!(res, None);
    let res = board.distance(board.get((7, 7)), board.get((4, 6)));
    assert_eq!(res.unwrap(), (6, (7, 6)));
    let res = board.distance(board.get((1, 7)), board.get((4, 4)));
    assert_eq!(res.unwrap(), (6, (2, 7)));
}

pub fn part1(input: String) -> u32 {
    let mut board = parse_board(input);

    for bel in board.b.clone().into_iter() {
        // get the unit's current state
        let unit = board.get(bel.p);
        // first try to move
        let move_to = match unit.el {
            El::Goblin(_) => board.move_unit(unit),
            El::Elf(_) => board.move_unit(unit),
            El::Wall | El::Empty => continue,
        };
        if let Some(move_to) = move_to {
            let move_to = board.get_mut(move_to);
            assert_eq!(move_to.el, El::Empty);
            move_to.el = unit.el;
            board.get_mut(unit.p).el = El::Empty;
        }
        // try to attack
        let neighbour_enemy = board
            .neighbours(unit.p)
            .into_iter()
            .filter(|bel| {
                matches!(
                    (unit.el, bel.el),
                    (El::Goblin(_), El::Elf(_)) | (El::Elf(_), El::Goblin(_))
                )
            })
            .min_by(|e1, e2| (e1.health(), e1.p.1, e1.p.0).cmp(&(e2.health(), e2.p.1, e2.p.0)));
        if let Some(neighbour_enemy) = neighbour_enemy {
            board.get_mut(neighbour_enemy.p).hit(3);
        }
    }

    // find targets - no targets, done, sym ended
    // find empty slots next to targets - all occupied and none by me, turn done
    // find empty slot to move to with fewest steps and with lowest reading order - can't find path to target, turn done
    // targets -> in range -> reachable -> nearest -> chosen
    // take one step on shortest path and shortest reading order
    // attack:

    todo!()
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
    let mut board = parse_board(input.to_string());
    let mut moves = vec![];
    for i in 0..100 {
        let mut moved = false;
        for unit in board.b.clone().into_iter() {
            let move_to = match unit.el {
                El::Goblin(_) => board.move_unit(unit),
                El::Elf(_) => board.move_unit(unit),
                El::Wall | El::Empty => continue,
            };
            if let Some(move_to) = move_to {
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

fn parse_board(input: String) -> Board {
    let board: Vec<BoardEl> = input
        .split_terminator('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let el = match c {
                    '#' => El::Wall,
                    '.' => El::Empty,
                    'G' => El::Goblin(200),
                    'E' => El::Elf(200),
                    v => unreachable!(v),
                };
                BoardEl { p: (x, y), el }
            })
        })
        .collect();
    Board::new(board)
}
impl BoardEl {
    fn health(&self) -> u8 {
        match self.el {
            El::Wall => unreachable!(),
            El::Goblin(h) => h,
            El::Elf(h) => h,
            El::Empty => unreachable!(),
        }
    }

    fn hit(&mut self, hit_points: u8) {
        let health = self.health();
        self.el = if health > 3 {
            match self.el {
                El::Wall => unreachable!(),
                El::Goblin(h) => El::Goblin(h - hit_points),
                El::Elf(h) => El::Elf(h - hit_points),
                El::Empty => unreachable!(),
            }
        } else {
            El::Empty
        }
    }
}
