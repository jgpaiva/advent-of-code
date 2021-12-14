mod year_2018;
mod year_2021;

mod utils;
use utils::*;

fn main() {
    let skip_slow = true;
    days_2021(skip_slow);
    days_2018(skip_slow);
}

fn days_2021(skip_slow: bool) {
    use year_2021::*;
    println!("2021");
    println!("day01 part 1: {}", day01::part1(&read_lines("2021/day01")));
    println!("day01 part 2: {}", day01::part2(&read_lines("2021/day01")));
    println!("day02 part 1: {}", day02::part1(&read_lines("2021/day02")));
    println!("day02 part 2: {}", day02::part2(&read_lines("2021/day02")));
    println!("day03 part 1: {}", day03::part1(&read_lines("2021/day03")));
    println!("day03 part 2: {}", day03::part2(&read_lines("2021/day03")));
    println!("day04 part 1: {}", day04::part1(read_file("2021/day04")));
    println!("day04 part 2: {}", day04::part2(read_file("2021/day04")));
    println!("day05 part 1: {}", day05::part1(read_file("2021/day05")));
    println!("day05 part 2: {}", day05::part2(read_file("2021/day05")));
    println!(
        "day06 part 1: {}",
        day06::part1(read_file("2021/day06"), 80)
    );
    println!(
        "day06 part 2: {}",
        day06::part2(read_file("2021/day06"), 256)
    );
    println!("day07 part 1: {}", day07::part1(read_file("2021/day07")));
    println!("day07 part 2: {}", day07::part2(read_file("2021/day07")));
    println!("day08 part 1: {}", day08::part1(read_file("2021/day08")));
    println!("day08 part 2: {}", day08::part2(read_file("2021/day08")));
    println!("day09 part 1: {}", day09::part1(read_file("2021/day09")));
    println!("day09 part 2: {}", day09::part2(read_file("2021/day09")));
    println!("day10 part 1: {}", day10::part1(read_file("2021/day10")));
    println!("day10 part 2: {}", day10::part2(read_file("2021/day10")));
    println!(
        "day11 part 1: {}",
        day11::part1(read_file("2021/day11"), 100)
    );
    println!(
        "day11 part 2: {}",
        day11::part2(read_file("2021/day11"), 1000)
    );
    println!("day12 part 1: {}", day12::part1(read_file("2021/day12")));
    if !skip_slow {
        println!("day12 part 2: {}", day12::part2(read_file("2021/day12")));
    }
    println!("day13 part 1: {}", day13::part1(read_file("2021/day13")));
    println!("day13 part 2: \n{}", day13::part2(read_file("2021/day13")));
    println!("day14 part 1: {}", day14::part1(read_file("2021/day14")));
    println!("day14 part 2: {}", day14::part2(read_file("2021/day14")));
}

fn days_2018(skip_slow: bool) {
    use year_2018::*;
    println!("2018");
    println!("day01: {}", day01::part2(read_lines("2018/day01")));
    println!("day02 part 1: {}", day02::part1(read_lines("2018/day02")));
    println!("day02 part 2: {}", day02::part2(read_lines("2018/day02")));
    println!("day03: {}", day03::day03(read_lines("2018/day03")));
    println!("day04: {}", day04::day04(read_lines("2018/day04")));
    println!("day05: {}", day05::day05(read_lines("2018/day05")));
    if !skip_slow {
        println!("day06 part 1: {}", day06::part1(read_lines("2018/day06")));
    }
    println!(
        "day06 part 2: {}",
        day06::part2(read_lines("2018/day06"), 10000)
    );
    println!("day07 part 1: {}", day07::part1(&read_lines("2018/day07")));
    println!(
        "day07 part 2: {}",
        day07::part2(&read_lines("2018/day07"), 5, 60)
    );
    println!("day08 part 1: {}", day08::part1(&read_lines("2018/day08")));
    println!("day08 part 2: {}", day08::part2(&read_lines("2018/day08")));
    // part 2 is the same as part 1, but faster
    println!("day09 part 1: {}", day09::part2(473, 70904));
    // skipped because it takes a while
    if !skip_slow {
        println!("day09 part 2: {}", day09::part2(473, 7090400));
        println!(
            "day10 part 1: \n{}",
            day10::part1(read_file("2018/day10"), 10304)
        );
    }
    println!("day11 part 1: {:?}", day11::part1(1788));
    if !skip_slow {
        println!("day11 part 2: {:?}", day11::part2(1788));
    }
    println!(
        "day12 part 1: {}",
        day12::part1(read_file("2018/day12"), 20)
    );
    if !skip_slow {
        println!(
            "day12 part 2: {}",
            day12::part2(read_file("2018/day12"), 500)
        );
    }
    println!("day13 part 1: {:?}", day13::part1(read_file("2018/day13")));
    println!("day13 part 2: {:?}", day13::part2(read_file("2018/day13")));
    println!("day14 part 1: {:?}", day14::part1(681901));
    if !skip_slow {
        println!("day14 part 2: {:?}", day14::part2("681901"));
        println!("day15 part 1: {:?}", day15::part1(read_file("2018/day15")));
        println!("day15 part 2: {:?}", day15::part2(read_file("2018/day15")));
    }
}
