#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2018/test_day8");
    assert_eq!(part1(&input), 138);
    assert_eq!(part2(&input), 66);
}

pub fn part2(lines: &[String]) -> usize {
    let input: Vec<i32> = lines[0]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (size, _, metadata) = metadata_for_sub_tree(0, &input);
    assert_eq!(size, input.len());
    metadata
}

pub fn part1(lines: &[String]) -> usize {
    let input: Vec<i32> = lines[0]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (size, metadata, _) = metadata_for_sub_tree(0, &input);
    assert_eq!(size, input.len());
    metadata
}

pub fn metadata_for_sub_tree(node: usize, input: &[i32]) -> (usize, usize, usize) {
    let n_children = input[node] as usize;
    let n_metadata = input[node + 1] as usize;
    if n_children == 0 {
        return (
            2 + n_metadata,
            (0..n_metadata).map(|i| input[i + 2 + node]).sum::<i32>() as usize,
            (0..n_metadata).map(|i| input[i + 2 + node]).sum::<i32>() as usize,
        );
    }
    let mut current_child = node + 2;
    let mut child_metadata = 0_usize;
    let mut child_metadata_v2 = vec![];
    let mut child_size = 0;
    for _ in 0..n_children {
        let (size, metadata, metadata_v2) = metadata_for_sub_tree(current_child, input);
        current_child += size;
        child_metadata += metadata as usize;
        child_size += size;
        child_metadata_v2.push(metadata_v2 as i32);
    }
    (
        2 + n_metadata + child_size,
        child_metadata
            + (0..n_metadata)
                .map(|i| input[2 + child_size + node + i])
                .sum::<i32>() as usize,
        (0..n_metadata)
            .map(|i| input[2 + child_size + node + i] as usize)
            .map(|i| child_metadata_v2.get(i - 1))
            .flatten()
            .sum::<i32>() as usize,
    )
}
