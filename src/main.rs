mod year_2018;
mod year_2021;

mod utils;
use utils::*;

fn main() {
    days_2021();
    days_2018();
}

fn days_2021() {
    use year_2021::*;
    println!("2021");
    println!("day1 part 1: {}", day1::part1(&read_lines("2021/day1")));
    println!("day1 part 2: {}", day1::part2(&read_lines("2021/day1")));
    println!("day2 part 1: {}", day2::part1(&read_lines("2021/day2")));
    println!("day2 part 2: {}", day2::part2(&read_lines("2021/day2")));
    println!("day3 part 1: {}", day3::part1(&read_lines("2021/day3")));
    println!("day3 part 2: {}", day3::part2(&read_lines("2021/day3")));
    println!("day4 part 1: {}", day4::part1(read_file("2021/day4")));
    println!("day4 part 2: {}", day4::part2(read_file("2021/day4")));
}

fn days_2018() {
    use year_2018::days_1_to_7::*;
    println!("2018");
    println!("day1: {}", day1_2018(read_lines("2018/day1")));
    println!("day2: {}", day2_2018(read_lines("2018/day2")));
    println!("day2 part 2: {}", day2_2018_part2(read_lines("2018/day2")));
    println!("day3: {}", day3_2018(read_lines("2018/day3")));
    println!("day4: {}", day4_2018(read_lines("2018/day4")));
    println!("day5: {}", day5_2018(read_lines("2018/day5")));
    println!("day6: {}", day6_2018(read_lines("2018/day6")));
    println!(
        "day6 part 2: {}",
        day6_2018_part2(read_lines("2018/day6"), 10000)
    );
    println!("day7: {}", day7_2018(read_lines("2018/day7")));
}
