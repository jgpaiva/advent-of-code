use std::fs;

#[allow(dead_code)]
pub fn to_vec(arr: &[&str]) -> Vec<String> {
    arr.iter().map(|x| x.to_string()).collect()
}

pub fn read_lines(file_name: &str) -> Vec<String> {
    let contents =
        fs::read_to_string(format!("data/{}.txt", file_name)).expect("error reading file");
    contents.lines().map(|x| x.to_string()).collect()
}

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(format!("data/{}.txt", file_name)).expect("error reading file")
}

#[allow(unused)]
pub fn read_test_file(source_file_name: &str) -> String {
    let day = source_file_name
        .split_once("day")
        .unwrap()
        .1
        .chars()
        .take(2)
        .collect::<String>();
    let year = source_file_name
        .split_once('_')
        .unwrap()
        .1
        .chars()
        .take(4)
        .collect::<String>();
    let file_name = &format!("{year}/test_day{day}");
    fs::read_to_string(format!("data/{}.txt", file_name)).expect("error reading file")
}

#[macro_export]
macro_rules! day {
    ($y:literal, $i:ident) => {
        let now = std::time::Instant::now();
        print!(
            "{} part 1: {}",
            stringify!($i),
            $i::part1(&read_file(format!("{}/{}", $y, stringify!($i)).as_str()))
        );
        println!("({} secs)", now.elapsed().as_secs());
        let now = std::time::Instant::now();
        print!(
            "{} part 2: {}",
            stringify!($i),
            $i::part2(&read_file(format!("{}/{}", $y, stringify!($i)).as_str()))
        );
        println!("({} secs)", now.elapsed().as_secs());
    };
}
