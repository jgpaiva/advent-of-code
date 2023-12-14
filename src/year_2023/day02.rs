use regex;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 8);
    assert_eq!(part2(&input), 2286);
}

pub fn part1(input: &str) -> u32 {
    let games = parse_input(input);
    games
        .iter()
        .filter(|game| {
            game.sets.iter().all(|set| {
                set.results.iter().all(|(count, color)| match color {
                    Color::Red => *count <= 12,
                    Color::Green => *count <= 13,
                    Color::Blue => *count <= 14,
                })
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let games = parse_input(input);
    games
        .iter()
        .map(|game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for set in &game.sets {
                for (count, color) in &set.results {
                    match color {
                        Color::Red => min_red = min_red.max(*count),
                        Color::Green => min_green = min_green.max(*count),
                        Color::Blue => min_blue = min_blue.max(*count),
                    }
                }
            }
            min_red * min_green * min_blue
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Game> {
    let input = input
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let game_re = regex::Regex::new(r"Game (.*)").unwrap();
    let set_re = regex::Regex::new(r"(\w+) (\w+)").unwrap();

    input
        .into_iter()
        .map(|line| {
            let (game, rest) = line.split_once(':').unwrap();

            let c = game_re.captures(game).unwrap();
            let id: u32 = c[1].to_string().parse().unwrap();
            let sets: Vec<&str> = rest.split(';').collect();

            let sets = sets
                .into_iter()
                .map(|set| Set {
                    results: set_re
                        .captures_iter(set)
                        .map(|c| {
                            let (_, [v, color]) = c.extract();
                            let v: u32 = v.parse().unwrap();
                            let color = Color::from_str(color);
                            (v, color)
                        })
                        .collect(),
                })
                .collect();
            Game { id, sets }
        })
        .collect()
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    results: Vec<(u32, Color)>,
}
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}
impl Color {
    fn from_str(s: &str) -> Color {
        match s {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => unreachable!(),
        }
    }
}
