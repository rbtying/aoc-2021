// --- Day 5: Hydrothermal Venture ---

// You come across a field of hydrothermal vents on the ocean floor! These vents
// constantly produce large, opaque clouds, so it would be best to avoid them if
// possible.

// They tend to form in lines; the submarine helpfully produces a list of nearby
// lines of vents (your puzzle input) for you to review. For example:

// 0,9 -> 5,9
// 8,0 -> 0,8
// 9,4 -> 3,4
// 2,2 -> 2,1
// 7,0 -> 7,4
// 6,4 -> 2,0
// 0,9 -> 2,9
// 3,4 -> 1,4
// 0,0 -> 8,8
// 5,5 -> 8,2

// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where
// x1,y1 are the coordinates of one end the line segment and x2,y2 are the
// coordinates of the other end. These line segments include the points at both
// ends. In other words:

//     An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
//     An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.

// For now, only consider horizontal and vertical lines: lines where either x1 = x2
// or y1 = y2.

// So, the horizontal and vertical lines from the above list would produce the
// following diagram:

// .......1..
// ..1....1..
// ..1....1..
// .......1..
// .112111211
// ..........
// ..........
// ..........
// ..........
// 222111....

// In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9.
// Each position is shown as the number of lines which cover that point or . if no
// line covers that point. The top-left pair of 1s, for example, comes from 2,2 ->
// 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9
// -> 2,9.

// To avoid the most dangerous areas, you need to determine the number of points
// where at least two lines overlap. In the above example, this is anywhere in the
// diagram with a 2 or larger - a total of 5 points.

// Consider only horizontal and vertical lines. At how many points do at least two
// lines overlap?

// --- Part Two ---

// Unfortunately, considering only horizontal and vertical lines doesn't give you
// the full picture; you need to also consider diagonal lines.

// Because of the limits of the hydrothermal vent mapping system, the lines in your
// list will only ever be horizontal, vertical, or a diagonal line at exactly 45
// degrees. In other words:

//     An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
//     An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.

// Considering all lines from the above example would now produce the following
// diagram:

// 1.1....11.
// .111...2..
// ..2.1.111.
// ...1.2.2..
// .112313211
// ...1.2....
// ..1...1...
// .1.....1..
// 1.......1.
// 222111....

// You still need to determine the number of points where at least two lines
// overlap. In the above example, this is still anywhere in the diagram with a 2 or
// larger - now a total of 12 points.

// Consider all of the lines. At how many points do at least two lines overlap?

use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    pub fn from_str(s: &str) -> Line {
        let (start, end) = s.split_once("->").unwrap();
        let (start_x, start_y) = start.split_once(",").unwrap();
        let (end_x, end_y) = end.split_once(",").unwrap();
        Line {
            start: (
                start_x.trim().parse().unwrap(),
                start_y.trim().parse().unwrap(),
            ),
            end: (end_x.trim().parse().unwrap(), end_y.trim().parse().unwrap()),
        }
    }

    /// Returns min_x, max_x, min_y, max_y
    pub fn bounds(&self) -> ((usize, usize), (usize, usize)) {
        (
            (self.start.0.min(self.end.0), self.start.0.max(self.end.0)),
            (self.start.1.min(self.end.0), self.start.1.max(self.end.1)),
        )
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    pub fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub fn is_diagonal(&self) -> bool {
        (self.start.0.max(self.end.0) - self.start.0.min(self.end.0))
            == (self.start.1.max(self.end.1) - self.start.1.min(self.end.1))
    }

    pub fn points_iter(&self) -> Box<dyn Iterator<Item = (usize, usize)> + '_> {
        if self.is_horizontal() {
            if self.start.0 <= self.end.0 {
                Box::new((self.start.0..=self.end.0).map(move |x| (x, self.start.1)))
            } else {
                Box::new((self.end.0..=self.start.0).map(move |x| (x, self.start.1)))
            }
        } else if self.is_vertical() {
            if self.start.1 <= self.end.1 {
                Box::new((self.start.1..=self.end.1).map(move |y| (self.start.0, y)))
            } else {
                Box::new((self.end.1..=self.start.1).map(move |y| (self.start.0, y)))
            }
        } else if self.is_diagonal() {
            if self.start.0 < self.end.0 {
                if self.start.1 < self.end.1 {
                    // diagonal up-right at 45 degrees
                    Box::new(
                        (0..=self.end.0 - self.start.0)
                            .map(move |offset| (self.start.0 + offset, self.start.1 + offset)),
                    )
                } else {
                    // diagonal down-right at 45 degrees
                    Box::new(
                        (0..=self.end.0 - self.start.0)
                            .map(move |offset| (self.start.0 + offset, self.start.1 - offset)),
                    )
                }
            } else {
                if self.start.1 < self.end.1 {
                    // diagonal up-left at 45 degrees
                    Box::new(
                        (0..=self.start.0 - self.end.0)
                            .map(move |offset| (self.start.0 - offset, self.start.1 + offset)),
                    )
                } else {
                    // diagonal down-left at 45 degrees
                    Box::new(
                        (0..=self.start.0 - self.end.0)
                            .map(move |offset| (self.start.0 - offset, self.start.1 - offset)),
                    )
                }
            }
        } else {
            unimplemented!()
        }
    }
}

pub fn part_1(s: &str) -> usize {
    let lines = s.lines().map(|s_| Line::from_str(s_));
    let mut counts: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines {
        if line.is_horizontal() || line.is_vertical() {
            for point in line.points_iter() {
                *counts.entry(point).or_default() += 1;
            }
        }
    }

    counts.iter().filter(|(_, ct)| **ct >= 2).count()
}

pub fn part_2(s: &str) -> usize {
    let lines = s.lines().map(|s_| Line::from_str(s_));
    let mut counts: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines {
        for point in line.points_iter() {
            *counts.entry(point).or_default() += 1;
        }
    }

    counts.iter().filter(|(_, ct)| **ct >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn test_day_5_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 5);
    }

    #[test]
    fn test_day_5_part_1() {
        assert_eq!(part_1(include_str!("input/day_5.txt")), 4826);
    }

    #[test]
    fn test_day_5_example_part_2() {
        assert_eq!(part_2(EXAMPLE), 12);
    }

    #[test]
    fn test_day_5_part_2() {
        assert_eq!(part_2(include_str!("input/day_5.txt")), 16793);
    }
}
