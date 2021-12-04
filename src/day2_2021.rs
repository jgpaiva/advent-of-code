#[path = "utils.rs"]
mod utils;

#[test]
fn test_day2() {
    let input = utils::to_vec(&[
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ]);
    assert_eq!(part2(&input), 900);
    assert_eq!(part1(&input), 150);
}

pub fn part2(lines: &Vec<String>) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(c, v)| match c {
            "forward" => Day5Commands::Forward(v.parse().unwrap()),
            "down" => Day5Commands::Down(v.parse().unwrap()),
            "up" => Day5Commands::Up(v.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for i in lines {
        match i {
            Day5Commands::Forward(v) => {
                horizontal += v;
                vertical += aim * v
            }
            Day5Commands::Down(v) => aim += v,
            Day5Commands::Up(v) => aim -= v,
        }
    }
    horizontal * vertical
}

enum Day5Commands {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn part1(lines: &Vec<String>) -> i32 {
    let lines = lines
        .iter()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(c, v)| match c {
            "forward" => Day5Commands::Forward(v.parse().unwrap()),
            "down" => Day5Commands::Down(v.parse().unwrap()),
            "up" => Day5Commands::Up(v.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut horizontal = 0;
    let mut vertical = 0;
    for i in lines {
        match i {
            Day5Commands::Forward(v) => horizontal += v,
            Day5Commands::Down(v) => vertical += v,
            Day5Commands::Up(v) => vertical -= v,
        }
    }
    horizontal * vertical
}
