#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_file("2021/test_day20");
    assert_eq!(part1(input.clone()), 35);
    assert_eq!(part2(input), 3351);
}

pub fn part2(input: String) -> usize {
    let (algo, image) = input.split_once("\n\n").unwrap();
    let algo = algo
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut image = to_vec_bool(image);

    for _i in 0..50 {
        image = run_iter(image, &algo);
    }

    image
        .into_iter()
        .flat_map(|line| line.into_iter())
        .filter(|i| *i)
        .count()
}

pub fn part1(input: String) -> usize {
    let (algo, image) = input.split_once("\n\n").unwrap();
    let algo = algo
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let image = to_vec_bool(image);

    let image_out = run_iter(image, &algo);
    let image_out = run_iter(image_out, &algo);

    image_out
        .into_iter()
        .flat_map(|line| line.into_iter())
        .filter(|i| *i)
        .count()
}

#[allow(clippy::needless_range_loop)]
fn run_iter(image: Vec<Vec<bool>>, algo: &[bool]) -> Vec<Vec<bool>> {
    let border_value = image[0].iter().all(|v| *v);
    let image = add_border(image, border_value);
    let image_len_i = image.len();
    let image_len_j = image[0].len();
    let mut image_out: Vec<Vec<bool>> = (0..image_len_i)
        .map(|_| (0..image_len_j).map(|_| false).collect())
        .collect();
    for i in 1..image_len_i - 1 {
        for j in 1..image_len_j - 1 {
            let bit_slice = image[i - 1][j - 1..=j + 1]
                .iter()
                .chain(image[i][j - 1..=j + 1].iter())
                .chain(image[i + 1][j - 1..=j + 1].iter());
            let value: usize = bit_slice
                .rev()
                .enumerate()
                .map(|(i, v)| if *v { 1 << i } else { 0 })
                .sum();
            image_out[i][j] = algo[value];
        }
    }
    if (1..image_len_i - 1).all(|i| image_out[i][1]) {
        for i in 0..image_len_i {
            image_out[i][0] = true;
        }
    }
    if (1..image_len_i - 1).all(|i| image_out[i][image_len_j - 2]) {
        for i in 0..image_len_i {
            image_out[i][image_len_j - 1] = true;
        }
    }
    if (1..image_len_j - 1).all(|j| image_out[1][j]) {
        for j in 0..image_len_j {
            image_out[0][j] = true;
        }
    }
    if (1..image_len_j - 1).all(|j| image_out[image_len_i - 2][j]) {
        for j in 0..image_len_j {
            image_out[image_len_i - 1][j] = true;
        }
    }
    image_out
}

fn to_vec_bool(image: &str) -> Vec<Vec<bool>> {
    image
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
fn to_string(image: Vec<Vec<bool>>) -> String {
    image
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|v| if v { '#' } else { '.' })
                .map(|c| c.to_string())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn add_border(image: Vec<Vec<bool>>, value: bool) -> Vec<Vec<bool>> {
    let image_len_i = image.len();
    let image_len_j = image[0].len();
    if (0..3)
        .flat_map(|i| {
            let image = &image;
            (0..image_len_j).map(move |j| image[i][j])
        })
        .all(|v| v)
        && (0..3)
            .flat_map(|j| {
                let image = &image;
                (0..image_len_i).map(move |i| image[i][j])
            })
            .all(|v| v)
    {
        return image;
    }
    (0..3)
        .map(|_| vec![value; image_len_j])
        .chain(image.into_iter())
        .chain((0..3).map(|_| vec![value; image_len_j]))
        .map(|line| {
            (0..3)
                .map(|_| value)
                .chain(line)
                .chain((0..3).map(|_| value))
                .collect()
        })
        .collect()
}
