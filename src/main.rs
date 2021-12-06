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
    println!("day1 part 1: {}", day01::part1(&read_lines("2021/day1")));
    println!("day1 part 2: {}", day01::part2(&read_lines("2021/day1")));
    println!("day2 part 1: {}", day02::part1(&read_lines("2021/day2")));
    println!("day2 part 2: {}", day02::part2(&read_lines("2021/day2")));
    println!("day3 part 1: {}", day03::part1(&read_lines("2021/day3")));
    println!("day3 part 2: {}", day03::part2(&read_lines("2021/day3")));
    println!("day4 part 1: {}", day04::part1(read_file("2021/day4")));
    println!("day4 part 2: {}", day04::part2(read_file("2021/day4")));
    println!("day5 part 1: {}", day05::part1(read_file("2021/day5")));
    println!("day5 part 2: {}", day05::part2(read_file("2021/day5")));
    println!("day6 part 1: {}", day06::part1(read_file("2021/day6"), 80));
    println!("day6 part 2: {}", day06::part2(read_file("2021/day6"), 256));
}

fn days_2018() {
    use year_2018::*;
    println!("2018");
    println!("day1: {}", day01::part2(read_lines("2018/day1")));
    println!("day2 part 1: {}", day02::part1(read_lines("2018/day2")));
    println!("day2 part 2: {}", day02::part2(read_lines("2018/day2")));
    println!("day3: {}", day03::day3(read_lines("2018/day3")));
    println!("day4: {}", day04::day4(read_lines("2018/day4")));
    println!("day5: {}", day05::day5(read_lines("2018/day5")));
    // skipped because it takes a while
    //    println!("day6 part 1: {}", day6::part1(read_lines("2018/day6")));
    println!(
        "day6 part 2: {}",
        day06::part2(read_lines("2018/day6"), 10000)
    );
    println!("day7 part 1: {}", day07::part1(&read_lines("2018/day7")));
    println!(
        "day7 part 2: {}",
        day07::part2(&read_lines("2018/day7"), 5, 60)
    );
    println!("day8 part 1: {}", day08::part1(&read_lines("2018/day8")));
    println!("day8 part 2: {}", day08::part2(&read_lines("2018/day8")));
    // part 2 is the same as part 1, but faster
    println!("day9 part 1: {}", day09::part2(473, 70904));
    // skipped because it takes a while
    // println!("day9 part 2: {}", day9::part2(473, 7090400));
    println!(
        "day10 part 1: \n{}",
        day10::part1(read_file("2018/day10"), 10304)
    );
    println!("day11 part 1: {:?}", day11::part1(1788));
    // skipped because it takes a while
    // println!("day11 part 2: {:?}", day11::part2(1788));
}
