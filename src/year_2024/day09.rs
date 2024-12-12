use std::collections::{BTreeSet, HashMap};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 1928);
    assert_eq!(part2(&input), 2858);
}

const SPACE: i32 = -1;

pub fn part1(input: &str) -> u64 {
    let input = parse_input(input);
    let mut disk = Vec::new();
    let mut free_space = false;
    let mut file_index: i32 = 0;
    for i in input {
        if free_space {
            disk.resize(disk.len() + i as usize, SPACE);
        } else {
            for _ in 0..i {
                disk.push(file_index);
            }
            file_index += 1;
        }
        free_space = !free_space
    }
    let mut i = 0;
    let mut j = disk.len() - 1;
    loop {
        if i == j {
            break;
        }
        if disk[i] != SPACE {
            i += 1;
            continue;
        }
        if disk[j] == SPACE {
            j -= 1;
            continue;
        }
        disk.swap(i, j);
    }
    let mut retval: u64 = 0;
    for (i, v) in disk.into_iter().enumerate() {
        if v == SPACE {
            continue;
        }
        retval += i as u64 * v as u64;
    }
    retval
}

fn parse_input(input: &str) -> Vec<u8> {
    let input: Vec<u8> = input
        .trim()
        .chars()
        .map(|v| v as u8 - b'0')
        .inspect(|v| {
            assert!(*v < 10, "{}", v);
        })
        .collect();
    input
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum FileOrSpaceEnum {
    File(u32),
    Space,
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct FileOrSpace {
    pos: usize,
    length: u8,
    file_or_space: FileOrSpaceEnum,
}

fn file(pos: usize, length: u8, id: u32) -> FileOrSpace {
    FileOrSpace {
        pos,
        length,
        file_or_space: FileOrSpaceEnum::File(id),
    }
}

fn space(pos: usize, length: u8) -> FileOrSpace {
    FileOrSpace {
        pos,
        length,
        file_or_space: FileOrSpaceEnum::Space,
    }
}

pub fn part2(input: &str) -> u64 {
    let dbg_mode = false;
    let input = parse_input(input);
    let mut free_space = false;
    let mut pos = 0;
    let mut id = 0;
    let mut space_by_size: HashMap<u8, BTreeSet<FileOrSpace>> = HashMap::new();
    let mut numbers = Vec::new();
    for v in input {
        if free_space {
            let space = space(pos, v);
            let spaces = space_by_size.entry(space.length).or_default();
            spaces.insert(space);
        } else {
            let f = file(pos, v, id);
            numbers.push(f);
            id += 1;
        }
        pos += v as usize;
        free_space = !free_space;
    }

    dbg_state(
        numbers.iter().cloned(),
        space_by_size.values().flatten().cloned(),
        dbg_mode,
    );
    for i in (0..numbers.len()).rev() {
        let n = &mut numbers[i];
        let space = (n.length..10)
            .filter_map(|n| space_by_size.get(&n).and_then(|v| v.first()))
            .min_by(|v1, v2| v1.pos.cmp(&v2.pos))
            .cloned();
        let Some(space) = space else {
            continue; // can't be moved, no space available
        };
        if space.pos > n.pos {
            continue; // not worth moving
        }
        // old space is going away
        let spaces = space_by_size.get_mut(&space.length);
        let spaces = spaces.expect("must exist, I just got it");
        assert!(spaces.remove(&space));

        if space.length > n.length {
            let new_space = FileOrSpace {
                pos: space.pos + n.length as usize,
                length: space.length - n.length,
                file_or_space: FileOrSpaceEnum::Space,
            };
            let spaces = space_by_size.entry(new_space.length).or_default();
            spaces.insert(new_space);
        }
        let new_space = FileOrSpace {
            pos: n.pos,
            length: n.length,
            file_or_space: FileOrSpaceEnum::Space,
        };
        // move the file
        n.pos = space.pos;
        // replace the file with space
        let spaces = space_by_size.entry(new_space.length).or_default();
        spaces.insert(new_space);

        dbg_state(
            numbers.clone().iter().cloned(),
            space_by_size.values().flatten().cloned(),
            dbg_mode,
        );
    }
    dbg_state(
        numbers.iter().cloned(),
        space_by_size.values().flatten().cloned(),
        dbg_mode,
    );
    let final_state: BTreeSet<FileOrSpace> = numbers
        .into_iter()
        .chain(space_by_size.values().flatten().cloned())
        .collect();
    let mut retval = 0;
    for v in final_state {
        for i in 0..v.length {
            match v.file_or_space {
                FileOrSpaceEnum::File(id) => {
                    retval += id as u64 * (v.pos + i as usize) as u64;
                }
                FileOrSpaceEnum::Space => {}
            }
        }
    }
    retval
}

fn dbg_state(
    numbers: impl Iterator<Item = FileOrSpace>,
    spaces: impl Iterator<Item = FileOrSpace>,
    dbg_mode: bool,
) {
    if !dbg_mode {
        return;
    }
    let mut dbg = String::new();
    for v in numbers.into_iter().chain(spaces).collect::<BTreeSet<_>>() {
        for _ in 0..v.length {
            match v.file_or_space {
                FileOrSpaceEnum::File(id) => {
                    dbg.push_str(&format!("{}", id));
                }
                FileOrSpaceEnum::Space => {
                    dbg.push('.');
                }
            }
        }
    }
    dbg!(dbg);
}
