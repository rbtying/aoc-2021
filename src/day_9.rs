// --- Day 9: Smoke Basin ---

// These caves seem to be lava tubes. Parts are even still volcanically active;
// small hydrothermal vents release smoke into the caves that slowly settles like
// rain.

// If you can model how the smoke flows through the caves, you might be able to
// avoid it and be that much safer. The submarine generates a heightmap of the
// floor of the nearby caves for you (your puzzle input).

// Smoke flows to the lowest point of the area it's in. For example, consider the
// following heightmap:

// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678

// Each number corresponds to the height of a particular location, where 9 is the
// highest and 0 is the lowest a location can be.

// Your first goal is to find the low points - the locations that are lower than
// any of its adjacent locations. Most locations have four adjacent locations (up,
// down, left, and right); locations on the edge or corner of the map have three or
// two adjacent locations, respectively. (Diagonal locations do not count as
// adjacent.)

// In the above example, there are four low points, all highlighted: two are in the
// first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom
// row (also a 5). All other locations on the heightmap have some lower adjacent
// location, and so are not low points.

// The risk level of a low point is 1 plus its height. In the above example, the
// risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of
// all low points in the heightmap is therefore 15.

// Find all of the low points on your heightmap. What is the sum of the risk levels
// of all low points on your heightmap?

// --- Part Two ---

// Next, you need to find the largest basins so you know what areas are most
// important to avoid.

// A basin is all locations that eventually flow downward to a single low point.
// Therefore, every low point has a basin, although some basins are very small.
// Locations of height 9 do not count as being in any basin, and all other
// locations will always be part of exactly one basin.

// The size of a basin is the number of locations within the basin, including the
// low point. The example above has four basins.

// The top-left basin, size 3:

// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678

// The top-right basin, size 9:

// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678

// The middle basin, size 14:

// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678

// The bottom-right basin, size 9:

// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678

// Find the three largest basins and multiply their sizes together. In the above
// example, this is 9 * 14 * 9 = 1134.

// What do you get if you multiply together the sizes of the three largest basins?

use std::collections::HashMap;

fn adjacent_points(i: usize, j: usize, max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    if i > 0 {
        res.push((i - 1, j));
    }
    if j > 0 {
        res.push((i, j - 1));
    }
    if i < max_i - 1 {
        res.push((i + 1, j));
    }
    if j < max_j - 1 {
        res.push((i, j + 1));
    }
    res
}

pub fn part_1(s: &str) -> usize {
    let mut rows: Vec<Vec<u8>> = vec![];
    for line in s.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        rows.push(row);
    }

    let mut sum_risk_levels = 0;
    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            let v = rows[i][j];
            if adjacent_points(i, j, rows.len(), rows[0].len())
                .into_iter()
                .all(|(ii, jj)| v < rows[ii][jj])
            {
                sum_risk_levels += (v as usize) + 1;
            }
        }
    }

    sum_risk_levels
}

pub fn part_2(s: &str) -> usize {
    let mut rows: Vec<Vec<u8>> = vec![];
    for line in s.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        rows.push(row);
    }

    let mut basins: Vec<Vec<usize>> = vec![];
    for _ in 0..rows.len() {
        basins.push((0..rows[0].len()).map(|_| 0).collect())
    }

    let mut basin_idx = 1;
    let mut low_points = vec![];
    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            let v = rows[i][j];
            if adjacent_points(i, j, rows.len(), rows[0].len())
                .into_iter()
                .all(|(ii, jj)| v < rows[ii][jj])
            {
                low_points.push((i, j));
                basins[i][j] = basin_idx;
                basin_idx += 1;
            }
        }
    }

    while let Some((i, j)) = low_points.pop() {
        for (n_i, n_j) in adjacent_points(i, j, rows.len(), rows[0].len()) {
            if rows[n_i][n_j] > rows[i][j] && rows[n_i][n_j] != 9 {
                // add it to the basin
                let v = basins[i][j];
                if basins[n_i][n_j] == 0 {
                    basins[n_i][n_j] = basins[i][j];
                    low_points.push((n_i, n_j));
                } else if basins[n_i][n_j] == v {
                    continue;
                } else {
                    unreachable!()
                }
            }
        }
    }

    let mut basin_counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..basins.len() {
        for j in 0..basins[0].len() {
            if basins[i][j] != 0 {
                *basin_counts.entry(basins[i][j]).or_default() += 1;
            }
        }
    }

    let mut basin_count_values: Vec<usize> = basin_counts.values().copied().collect();
    basin_count_values.sort();

    basin_count_values[basin_count_values.len() - 1]
        * basin_count_values[basin_count_values.len() - 2]
        * basin_count_values[basin_count_values.len() - 3]
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn test_day_9_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 15);
    }

    #[test]
    fn test_day_9_part_1() {
        assert_eq!(part_1(include_str!("input/day_9.txt")), 585);
    }

    #[test]
    fn test_day_9_example_part_2() {
        assert_eq!(part_2(EXAMPLE), 1134);
    }

    #[test]
    fn test_day_9_part_2() {
        assert_eq!(part_2(include_str!("input/day_9.txt")), 827904);
    }
}
