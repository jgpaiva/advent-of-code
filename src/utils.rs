use std::fs;

#[allow(dead_code)]
pub fn to_vec(arr: &[&str]) -> Vec<String> {
    arr.iter().map(|x| x.to_string()).collect()
}

pub fn read_lines(file_name: &str) -> Vec<String> {
    let contents = Box::new(
        fs::read_to_string(format!("data/{}.txt", file_name)).expect("error reading file"),
    );
    contents.lines().map(|x| x.to_string()).collect()
}

pub fn read_file(file_name: &str) -> Box<String> {
    Box::new(fs::read_to_string(format!("data/{}.txt", file_name)).expect("error reading file"))
}
