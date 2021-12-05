#[test]
fn test() {
    assert_eq!(part1(18), (33, 45));
    assert_eq!(part1(42), (21, 61));
    // skipped because it takes a while
    // assert_eq!(part2(18), ((90, 269), 16));
}

#[allow(dead_code)]
pub fn part2(serial: i32) -> ((usize, usize), usize) {
    let cells: Vec<Vec<i64>> = (1..=300)
        .map(|x| {
            (1..=300)
                .map(|y| fuel_cell((x, y), serial) as i64)
                .collect()
        })
        .collect();
    let mut partial_sums: Vec<Vec<i64>> = (1..=300)
        .map(|x| {
            (1..=300)
                .map(|y| fuel_cell((x, y), serial) as i64)
                .collect()
        })
        .collect();
    let v = (1..=300)
        .map(|size| {
            let for_size = (1..=(300 - size + 1))
                .flat_map(|x| {
                    (1..=(300 - size + 1))
                        .map(|y| {
                            let value = (x - 1..x - 1 + size)
                                .map(|x| partial_sums[x][y - 1])
                                .sum::<i64>();
                            ((x, y), value)
                        })
                        .collect::<Vec<_>>()
                })
                .max_by(|x1, x2| x1.1.cmp(&x2.1))
                .unwrap();
            for x in 0..300 {
                for y in 0..300 - size {
                    partial_sums[x][y] += cells[x][y + size];
                }
            }
            (for_size.0, for_size.1, size)
        })
        .max_by(|x1, x2| x1.1.cmp(&x2.1))
        .unwrap();
    (v.0, v.2)
}

pub fn part1(serial: i32) -> (usize, usize) {
    let cells: Vec<Vec<i32>> = (1..=300)
        .map(|x| (1..=300).map(|y| fuel_cell((x, y), serial)).collect())
        .collect();
    calculate(&cells, 3).0
}

fn calculate(cells: &[Vec<i32>], grid_size: usize) -> ((usize, usize), i32) {
    (1..=(300 - grid_size - 1))
        .flat_map(|x| {
            (1..=(300 - grid_size - 1))
                .map(|y| {
                    let value = (x - 1..x - 1 + grid_size)
                        .map(|x| (y - 1..y - 1 + grid_size).map(|y| cells[x][y]).sum::<i32>())
                        .sum::<i32>();
                    ((x, y), value)
                })
                .collect::<Vec<_>>()
        })
        .max_by(|x1, x2| x1.1.cmp(&x2.1))
        .unwrap()
}

#[test]
fn test_fuel_cell() {
    assert_eq!(fuel_cell((3, 5), 8), 4);
    assert_eq!(fuel_cell((122, 79), 57), -5);
    assert_eq!(fuel_cell((217, 196), 39), 0);
    assert_eq!(fuel_cell((101, 153), 71), 4);
}

pub fn fuel_cell(p: (i32, i32), serial: i32) -> i32 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = p.0 + 10;
    // Begin with a power level of the rack ID times the Y coordinate.
    let power_level = rack_id * p.1;
    // Increase the power level by the value of the grid serial number (your puzzle input).
    let power_level = power_level + serial;
    // Set the power level to itself multiplied by the rack ID.
    let power_level = power_level * rack_id;
    // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    let power_level = (power_level / 100) % 10;
    // Subtract 5 from the power level.
    power_level - 5
}
