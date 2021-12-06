use std::collections::HashMap;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2018/test_day05");
    assert_eq!(day05(input), "part 1: 10 part2: 4");
}

pub fn day05(lines: Vec<String>) -> String {
    let polymer = lines.into_iter().next().unwrap();
    let a_to_z = "abcdefghijklmnopqrstuvwxyz";
    let a_to_z_capital = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut mapping = HashMap::<char, char>::new();
    for (char1, char2) in a_to_z.chars().zip(a_to_z_capital.chars()) {
        mapping.insert(char1, char2);
        mapping.insert(char2, char1);
    }
    let mut min_size = None;
    let mut _min_polymer = None;
    for c in a_to_z.chars() {
        let other_c = *mapping.get(&c).unwrap();
        let new_polymer: String = polymer
            .chars()
            .filter(|x| *x != c && *x != other_c)
            .collect();
        let (size, polymer) = react(&mapping, new_polymer);
        match min_size {
            None => {
                min_size = Some(size);
                _min_polymer = Some(polymer);
            }
            Some(current_min_size) => {
                if current_min_size > size {
                    min_size = Some(size);
                    _min_polymer = Some(polymer);
                }
            }
        };
    }
    let (size, _final_polymer) = react(&mapping, polymer);
    format!("part 1: {} part2: {}", size, min_size.unwrap())
}

fn react(mapping: &HashMap<char, char>, polymer: String) -> (usize, String) {
    let mut queue: Vec<char> = vec![];
    for c in polymer.chars() {
        if let Some(top_of_queue) = queue.pop() {
            if *(mapping.get(&top_of_queue).unwrap()) == c {
                // two reacted, top one removed
                continue;
            }
            queue.push(top_of_queue);
        }
        queue.push(c);
    }
    let size = queue.len();
    let final_polymer: String = queue.iter().collect();
    (size, final_polymer)
}
