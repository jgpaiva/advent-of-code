mod year_2018 {
    automod::dir!(pub "src/year_2018");
}
mod year_2021 {
    automod::dir!(pub "src/year_2021");
}
mod year_2022 {
    automod::dir!(pub "src/year_2022");
}
mod year_2023 {
    automod::dir!(pub "src/year_2023");
}
mod year_2024 {
    automod::dir!(pub "src/year_2024");
}

mod utils;
use utils::*;

fn main() {
    let skip_slow = true;
    let skip_old_years = true;
    days_2024(skip_slow);
    if !skip_old_years {
        days_2022(skip_slow);
        days_2021(skip_slow);
        days_2018(skip_slow);
        days_2023(skip_slow);
    }
}

fn days_2024(_skip_slow: bool) {
    use year_2024::*;
    println!("2024");
    day!(2024, day01);
    day!(2024, day02);
    day!(2024, day03);
}

fn days_2023(_skip_slow: bool) {
    use year_2023::*;
    println!("2023");
    day!(2023, day01);
    day!(2023, day02);
    day!(2023, day03);
    day!(2023, day04);
    day!(2023, day05);
    day!(2023, day06);
    day!(2023, day07);
    day!(2023, day08);
}

fn days_2022(skip_slow: bool) {
    use year_2022::*;
    println!("2022");
    day!(2022, day01);
    day!(2022, day02);
    day!(2022, day03);
    day!(2022, day04);
    day!(2022, day05);
    day!(2022, day06);
    day!(2022, day07);
    day!(2022, day08);
    day!(2022, day09);
    day!(2022, day10);
    day!(2022, day11);
    day!(2022, day12);
    day!(2022, day13);
    day!(2022, day14);
    if !skip_slow {
        day!(2022, day15);
    }
    day!(2022, day16);
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
    println!(
        "day14 part 2: {}",
        day14::part2(read_file("2021/day14"), 40)
    );
    println!("day15 part 1: {}", day15::part1(read_file("2021/day15")));
    println!("day15 part 2: {}", day15::part2(read_file("2021/day15")));
    println!("day16 part 1: {}", day16::part1(read_file("2021/day16")));
    println!("day16 part 2: {}", day16::part2(read_file("2021/day16")));
    println!("day17 part 1: {:?}", day17::part1(read_file("2021/day17")));
    println!("day17 part 2: {:?}", day17::part2(read_file("2021/day17")));
    println!("day18 part 1: {:?}", day18::part1(read_file("2021/day18")));
    println!("day18 part 2: {:?}", day18::part2(read_file("2021/day18")));
    if !skip_slow {
        println!("day19 part 1: {:?}", day19::part1(read_file("2021/day19")));
        println!("day19 part 2: {:?}", day19::part2(read_file("2021/day19")));
    }
    println!("day20 part 1: {:?}", day20::part1(read_file("2021/day20")));
    println!("day20 part 2: {:?}", day20::part2(read_file("2021/day20")));
    println!("day21 part 1: {:?}", day21::part1(2, 1));
    println!("day21 part 2: {:?}", day21::part2(2, 1));
    println!("day22 part 1: {:?}", day22::part1(read_file("2021/day22")));
    println!("day22 part 2: {:?}", day22::part2(read_file("2021/day22")));
    if !skip_slow {
        println!(
            "day23 part 1: {:?}",
            day23::part1(read_file("2021/day23-1"))
        );
        println!(
            "day23 part 2: {:?}",
            day23::part2(read_file("2021/day23-2"))
        );
    }
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
        println!("day15 part 2: {:?}", day15::part2(read_file("2018/day15")));
        println!("day15 part 1: {:?}", day15::part1(read_file("2018/day15")));
        println!("day15 part 2: {:?}", day15::part2(read_file("2018/day15")));
    }
    println!("day16 part 1: {:?}", day16::part1(read_file("2018/day16")));
    println!("day16 part 2: {:?}", day16::part2(read_file("2018/day16")));
    if !skip_slow {
        // still wip
        println!("day17 part 1: {:?}", day17::part1(read_file("2018/day17")));
        println!("day17 part 2: {:?}", day17::part2(read_file("2018/day17")));
    }
}
