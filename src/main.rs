use std::fs;

mod day1_2021;
mod day2_2021;
mod day3_2021;
mod day4_2021;

mod days_2018;

mod utils;

fn main() {
    days_2021();
    days_2018();
}

fn days_2021() {
    println!("2021");
    println!(
        "day1 part 1: {}",
        day1_2021::part1(&read_lines("2021/day1"))
    );
    println!(
        "day1 part 2: {}",
        day1_2021::part2(&read_lines("2021/day1"))
    );
    println!(
        "day2 part 1: {}",
        day2_2021::part1(&read_lines("2021/day2"))
    );
    println!(
        "day2 part 2: {}",
        day2_2021::part2(&read_lines("2021/day2"))
    );
    println!(
        "day3 part 1: {}",
        day3_2021::part1(&read_lines("2021/day3"))
    );
    println!(
        "day3 part 2: {}",
        day3_2021::part2(&read_lines("2021/day3"))
    );
    println!(
        "day4 part 1: {}",
        day4_2021::part1(read_file("2021/day4").as_str())
    );
    println!(
        "day4 part 2: {}",
        day4_2021::part2(read_file("2021/day4").as_str())
    );
}

fn days_2018() {
    println!("2018");
    println!("day1: {}", days_2018::day1_2018(read_lines("2018/day1")));
    println!("day2: {}", days_2018::day2_2018(read_lines("2018/day2")));
    println!(
        "day2 part 2: {}",
        days_2018::day2_2018_part2(read_lines("2018/day2"))
    );
    println!("day3: {}", days_2018::day3_2018(read_lines("2018/day3")));
    println!("day4: {}", days_2018::day4_2018(read_lines("2018/day4")));
    println!("day5: {}", days_2018::day5_2018(read_lines("2018/day5")));
    println!("day6: {}", days_2018::day6_2018(read_lines("2018/day6")));
    println!(
        "day6 part 2: {}",
        days_2018::day6_2018_part2(read_lines("2018/day6"), 10000)
    );
    println!("day7: {}", days_2018::day7_2018(read_lines("2018/day7")));
}

fn read_file(file_name: &str) -> Box<String> {
    Box::new(fs::read_to_string(format!("data/{}.txt", file_name)).expect("error reading file"))
}

fn read_lines(file_name: &str) -> Vec<String> {
    let contents = Box::new(
        fs::read_to_string(format!("data/{}.txt", file_name)).expect("error reading file"),
    );
    contents.lines().map(|x| x.to_string()).collect()
}
