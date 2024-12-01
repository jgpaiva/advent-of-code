use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_test_file(file!());
    assert_eq!(part1(&input), 2);
    assert_eq!(part2(&input), 6);
}

pub fn part1(input: &str) -> u32 {
    let (path, input) = input.split_once("\n\n").unwrap();
    let input = input.split('\n').collect::<Vec<&str>>();

    let g = parse_input(&input);
    let mut cur = "AAA";
    for (i, dir) in path.chars().cycle().enumerate() {
        let (l, r) = &g[cur];
        match dir {
            'L' => cur = l,
            'R' => cur = r,
            _ => unreachable!(),
        }
        //dbg!(&cur);
        if cur == "ZZZ" {
            return i as u32 + 1;
        }
    }
    unreachable!()
}

#[allow(unused)]
pub fn part2_slow(input: &str) -> u32 {
    let (path, input) = input.split_once("\n\n").unwrap();
    let input = input.split('\n').collect::<Vec<&str>>();

    let g = parse_input(&input);
    let mut cur: Vec<_> = g.keys().filter(|v| v.ends_with('A')).collect();
    for (i, dir) in path.chars().cycle().enumerate() {
        for n in cur.iter_mut() {
            let (l, r) = &g[*n];
            match dir {
                'L' => *n = l,
                'R' => *n = r,
                _ => unreachable!(),
            }
        }
        if cur.iter().all(|c| c.ends_with('Z')) {
            return i as u32 + 1;
        }
    }
    0
}

// this doesnt yet work
pub fn part2(input: &str) -> u32 {
    let (path, input) = input.split_once("\n\n").unwrap();
    let input = input.split('\n').collect::<Vec<&str>>();

    let g = parse_input(&input);
    let mut ends = HashMap::new();
    let jumps: HashMap<_, _> = g
        .keys()
        .map(|start| {
            let mut n = start;
            let mut is_end = HashSet::new();
            for (i, dir) in path.chars().enumerate() {
                let (l, r) = &g[n];
                match dir {
                    'L' => n = l,
                    'R' => n = r,
                    _ => unreachable!(),
                }
                if n.ends_with('Z') {
                    is_end.insert(i);
                }
            }
            ends.insert(start.to_string(), is_end);
            (start.to_string(), n.to_string())
        })
        .collect();

    let mut cur: Vec<_> = g.keys().filter(|v| v.ends_with('A')).cloned().collect();
    //let mut loops: Vec<_> = (0..cur.len()).map(|_| HashSet::<String>::new()).collect();
    for j in 0..1000 {
        let mut iter = cur.iter();
        let first = iter.next().unwrap();
        let intersection = iter.fold(ends.get(first.as_str()).unwrap().clone(), |acc, v| {
            acc.intersection(&ends[v.as_str()]).cloned().collect()
        });
        if let Some(v) = intersection.iter().min() {
            return (j * path.len() + v + 1) as u32;
        }
        cur = cur.iter().map(|v| jumps[v.as_str()].clone()).collect();
        /* for (i, c) in cur.iter().enumerate(){
            if !loops[i].insert(c.to_string()){
            if loops[i].insert("cleared".to_string()){


               loops[i].clear();
               loops[i].insert(c.to_string());
               loops[i].insert("cleared".to_string());
               } else
               if loops[i].insert("loop".to_string()){
                   dbg!((&i, loops.len(), &j, loops[i].len() -2));
               }
            }

        }*/
    }
    // unreachable!()
    // this one is unsolved
    0
}

fn parse_input(input: &[&str]) -> HashMap<String, (String, String)> {
    let mut ret = HashMap::new();
    for l in input.iter() {
        let (n, rest) = l.split_once(" = ").unwrap();
        let (l, r) = rest.split_once(", ").unwrap();

        ret.insert(
            n.to_string(),
            (l[1..].to_string(), r[..r.len() - 1].to_string()),
        );
    }
    ret
}
