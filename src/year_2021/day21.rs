use std::collections::HashMap;

#[test]
fn test() {
    assert_eq!(part1(4, 8), 739785);
    assert_eq!(part2(4, 8), 444356092776315);
}

pub fn part2(p1: u32, p2: u32) -> u64 {
    let mut known_results = HashMap::new();
    let res = calculate_recur(p1, p2, 0, 0, true, &mut known_results);
    res.0.max(res.1)
}

type Memoize = HashMap<(u32, u32, u32, u32, bool), (u64, u64)>;

fn calculate_recur(
    p1: u32,
    p2: u32,
    score_1: u32,
    score_2: u32,
    one_to_play: bool,
    known_results: &mut Memoize,
) -> (u64, u64) {
    if let Some(res) = known_results.get(&(p1, p2, score_1, score_2, one_to_play)) {
        return *res;
    }
    if score_1 >= 21 {
        known_results.insert((p1, p2, score_1, score_2, one_to_play), (1, 0));
        return (1, 0);
    }
    if score_2 >= 21 {
        known_results.insert((p1, p2, score_1, score_2, one_to_play), (0, 1));
        return (0, 1);
    }
    let res = (1..=3)
        .flat_map(|i| (1..=3).flat_map(move |j| (1..=3).map(move |k| (i, j, k))))
        .map(|(i, j, k)| {
            if one_to_play {
                let p1 = ((p1 - 1) + (i + j + k)) % 10 + 1;
                let score_1 = score_1 + p1;
                calculate_recur(p1, p2, score_1, score_2, !one_to_play, known_results)
            } else {
                let p2 = ((p2 - 1) + (i + j + k)) % 10 + 1;
                let score_2 = score_2 + p2;
                calculate_recur(p1, p2, score_1, score_2, !one_to_play, known_results)
            }
        })
        .fold((0, 0), |(p1_acc, p2_acc), (p1, p2)| {
            (p1_acc + p1, p2_acc + p2)
        });
    known_results.insert((p1, p2, score_1, score_2, one_to_play), res);
    res
}

pub fn part1(mut p1: u32, mut p2: u32) -> u32 {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut dice = 1;
    let mut rolls = 0;
    loop {
        run_once(&mut dice, &mut p1);
        rolls += 3;
        p1_score += p1;
        if p1_score >= 1000 {
            break;
        }
        run_once(&mut dice, &mut p2);
        rolls += 3;
        p2_score += p2;
        if p2_score >= 1000 {
            break;
        }
    }
    if p1_score >= 1000 {
        rolls * p2_score
    } else {
        rolls * p1_score
    }
}

fn run_once(dice: &mut i32, p: &mut u32) {
    let roll1 = *dice;
    *dice = ((*dice - 1) + 1) % 100 + 1;
    let roll2 = *dice;
    *dice = ((*dice - 1) + 1) % 100 + 1;
    let roll3 = *dice;
    *dice = ((*dice - 1) + 1) % 100 + 1;
    let next_move: u32 = roll1 as u32 + roll2 as u32 + roll3 as u32;
    *p = ((*p - 1) + next_move) % 10 + 1;
}
