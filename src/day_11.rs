// --- Day 11: Dumbo Octopus ---

// You enter a large cavern full of rare bioluminescent dumbo octopuses! They
// seem to not like the Christmas lights on your submarine, so you turn them off
// for now.

// There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus
// slowly gains energy over time and flashes brightly for a moment when its
// energy is full. Although your lights are off, maybe you could navigate
// through the cave without disturbing the octopuses if you could predict when
// the flashes of light will happen.

// Each octopus has an energy level - your submarine can remotely measure the
// energy level of each octopus (your puzzle input). For example:

// 5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526

// The energy level of each octopus is a value between 0 and 9. Here, the
// top-left octopus has an energy level of 5, the bottom-right one has an energy
// level of 6, and so on.

// You can model the energy levels and flashes of light in steps. During a
// single step, the following occurs:

//     First, the energy level of each octopus increases by 1.
//     Then, any octopus with an energy level greater than 9 flashes. This
//     increases the energy level of all adjacent octopuses by 1, including
//     octopuses that are diagonally adjacent. If this causes an octopus to have
//     an energy level greater than 9, it also flashes. This process continues
//     as long as new octopuses keep having their energy level increased beyond
//     9. (An octopus can only flash at most once per step.)
//     Finally, any octopus that flashed during this step has its energy level
//     set to 0, as it used all of its energy to flash.

// Adjacent flashes can cause an octopus to flash on a step even if it begins
// that step with very little energy.

// [...]

// Given the starting energy levels of the dumbo octopuses in your cavern,
// simulate 100 steps. How many total flashes are there after 100 steps?

// It seems like the individual flashes aren't bright enough to navigate.
// However, you might have a better option: the flashes seem to be
// synchronizing!

// In the example above, the first time all octopuses flash simultaneously is
// step 195:

// If you can calculate the exact moments when the octopuses will all flash
// simultaneously, you should be able to navigate through the cavern. What is
// the first step during which all octopuses flash?

use std::collections::HashSet;

fn adjacent_points(
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (-1isize..=1)
        .flat_map(move |i_off| {
            (-1isize..=1).map(move |j_off| (i as isize + i_off, j as isize + j_off))
        })
        .filter(move |(i_, j_)| {
            *i_ >= 0
                && *i_ < max_i as isize
                && *j_ >= 0
                && *j_ < max_j as isize
                && (i as isize, j as isize) != (*i_, *j_)
        })
        .map(|(i_, j_)| (i_ as usize, j_ as usize))
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

    let mut num_flashes = 0;
    for _step in 0..100 {
        // Begining of step
        let mut queue = vec![];
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                rows[i][j] += 1;
                if rows[i][j] > 9 {
                    queue.push((i, j));
                }
            }
        }

        let mut has_flashed = HashSet::new();
        while let Some((i, j)) = queue.pop() {
            if has_flashed.contains(&(i, j)) {
                continue;
            }
            assert!(has_flashed.insert((i, j)));

            for (i_, j_) in adjacent_points(i, j, rows.len(), rows[0].len()) {
                rows[i_][j_] += 1;
                if rows[i_][j_] > 9 && !has_flashed.contains(&(i_, j_)) {
                    queue.push((i_, j_));
                }
            }
        }

        num_flashes += has_flashed.len();
        for (i, j) in has_flashed {
            rows[i][j] = 0;
        }
    }

    num_flashes
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

    let mut step = 1;
    loop {
        // Begining of step
        let mut queue = vec![];
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                rows[i][j] += 1;
                if rows[i][j] > 9 {
                    queue.push((i, j));
                }
            }
        }

        let mut has_flashed = HashSet::new();
        while let Some((i, j)) = queue.pop() {
            if has_flashed.contains(&(i, j)) {
                continue;
            }
            assert!(has_flashed.insert((i, j)));

            for (i_, j_) in adjacent_points(i, j, rows.len(), rows[0].len()) {
                rows[i_][j_] += 1;
                if rows[i_][j_] > 9 && !has_flashed.contains(&(i_, j_)) {
                    queue.push((i_, j_));
                }
            }
        }

        if has_flashed.len() == rows.len() * rows[0].len() {
            break;
        }

        for (i, j) in has_flashed {
            rows[i][j] = 0;
        }

        step += 1;
    }

    step
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn test_day_11_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 1656);
    }

    #[test]
    fn test_day_11_part_1() {
        assert_eq!(part_1(include_str!("input/day_11.txt")), 1655);
    }

    #[test]
    fn test_day_11_example_part_2() {
        assert_eq!(part_2(EXAMPLE), 195);
    }

    #[test]
    fn test_day_11_part_2() {
        assert_eq!(part_2(include_str!("input/day_11.txt")), 337);
    }
}
