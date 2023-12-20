use std::{collections::HashMap, str::FromStr};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 95437);
    assert_eq!(part2(&input), 24933642);
}

pub fn part2(lines: &str) -> u32 {
    let file_tree = create_file_tree(lines);
    let mut sizes = HashMap::new();
    let root_size = calc_sizes("/".to_string(), &file_tree, &mut sizes);
    let mut sizes: Vec<_> = sizes.values().cloned().collect();
    sizes.sort();
    for size in sizes.into_iter() {
        if root_size - size <= 40000000 {
            return size;
        }
    }
    unreachable!();
}

pub fn part1(lines: &str) -> u32 {
    let file_tree = create_file_tree(lines);
    let mut sizes = HashMap::new();
    calc_sizes("/".to_string(), &file_tree, &mut sizes);
    sizes.values().filter(|v| **v < 100000).sum()
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<LsEntry>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum LsEntry {
    Dir(String),
    File(String, u32),
}

impl FromStr for LsEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            s.split_once(' ')
                .ok_or(format!("could not split {s}"))
                .map(|(_, name)| LsEntry::Dir(name.to_string()))
        } else {
            s.split_once(' ')
                .ok_or(format!("could not split {s}"))
                .and_then(|(size, name)| {
                    size.parse()
                        .map_err(|e| format!("failed to parse {size}: {e}"))
                        .map(|size| LsEntry::File(name.to_string(), size))
                })
        }
    }
}

fn create_file_tree(lines: &str) -> HashMap<String, Vec<LsEntry>> {
    let mut current_ls = None;
    let mut commands = vec![];
    for line in lines.split_terminator('\n') {
        if line.starts_with("$ cd ") {
            if let Some(entries) = current_ls.take() {
                commands.push(Command::Ls(entries));
            }
            let (_, path) = line.split_once("$ cd ").unwrap();
            commands.push(Command::Cd(path.to_string()));
        } else if line.starts_with("$ ls") {
            if let Some(entries) = current_ls.take() {
                commands.push(Command::Ls(entries));
            }
            current_ls = Some(vec![]);
        } else {
            let mut entries = current_ls.unwrap();
            entries.push(
                line.parse::<LsEntry>()
                    .map_err(|err| format!("couldn't parse {line} as Ls Entry: {err}"))
                    .unwrap(),
            );
            current_ls = Some(entries);
        }
    }
    if let Some(entries) = current_ls.take() {
        commands.push(Command::Ls(entries));
    }

    let mut file_tree = HashMap::new();
    let mut current_path = vec![];
    for command in commands {
        match command {
            Command::Cd(path) if path == ".." => {
                current_path.pop();
            }
            Command::Cd(path) => {
                current_path.push(path);
            }
            Command::Ls(entries) => {
                let path: String = current_path.join("/");
                let path = path.replace("//", "/");
                file_tree.insert(path.clone(), entries.clone());
                for dir in entries.into_iter().filter_map(|e| {
                    if let LsEntry::Dir(dir) = e {
                        Some(dir)
                    } else {
                        None
                    }
                }) {
                    let dir_key = format!("{path}/{dir}");
                    let dir_key = dir_key.replace("//", "/");
                    file_tree.entry(dir_key).or_insert_with(Vec::new);
                }
            }
        }
    }
    file_tree
}

fn calc_sizes(
    current: String,
    file_tree: &HashMap<String, Vec<LsEntry>>,
    sizes: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(size) = sizes.get(&current) {
        return *size;
    }
    let mut size = 0;
    for entry in file_tree
        .get(&current)
        .unwrap_or_else(|| panic!("tried to get non-existent dir: {current}"))
    {
        let entry_size = match entry {
            LsEntry::Dir(dir) => {
                let dir_key = format!("{current}/{dir}");
                let dir_key = dir_key.replace("//", "/");
                calc_sizes(dir_key, file_tree, sizes)
            }
            LsEntry::File(_name, size) => *size,
        };
        size += entry_size;
    }
    sizes.insert(current, size);
    size
}
