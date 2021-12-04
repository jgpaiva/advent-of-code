#[path = "utils.rs"]
mod utils;

#[test]
fn test() {
    let input = utils::to_vec(&[
        "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
    ]);

    assert_eq!(part1(&input), 7);
    assert_eq!(part2(&input), 5);
}

pub fn part2(lines: &Vec<String>) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.parse())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    let lines = lines
        .iter()
        .zip(lines.iter().skip(1))
        .zip(lines.iter().skip(2))
        .map(|((v1, v2), v3)| v1 + v2 + v3)
        .collect::<Vec<i32>>();

    let mut prev = None;
    let mut res = 0;
    for i in lines {
        if let Some(prev) = prev.replace(i) {
            if prev < i {
                res = res + 1;
            }
        }
    }
    res
}

pub fn part1(lines: &Vec<String>) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.parse())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    let mut prev = None;
    let mut res = 0;
    for i in lines {
        if let Some(prev) = prev.replace(i) {
            if prev < i {
                res = res + 1;
            }
        }
    }
    res
}
