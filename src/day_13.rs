// --- Day 13: Transparent Origami ---

// You reach another volcanically active part of the cave. It would be nice if
// you could do some kind of thermal imaging so you could tell ahead of time
// which caves are too hot to safely enter.

// Fortunately, the submarine seems to be equipped with a thermal camera! When
// you activate it, you are greeted with:

// Congratulations on your purchase! To activate this infrared thermal imaging
// camera system, please enter the code found on page 1 of the manual.

// Apparently, the Elves have never used this feature. To your surprise, you
// manage to find the manual; as you go to open it, page 1 falls out. It's a
// large sheet of transparent paper! The transparent paper is marked with random
// dots and includes instructions on how to fold it up (your puzzle input). For
// example:

// 6,10
// 0,14
// 9,10
// 0,3
// 10,4
// 4,11
// 6,0
// 6,12
// 4,1
// 0,13
// 10,12
// 3,4
// 3,0
// 8,4
// 1,10
// 2,14
// 8,10
// 9,0

// fold along y=7
// fold along x=5

// The first section is a list of dots on the transparent paper. 0,0 represents
// the top-left coordinate. The first value, x, increases to the right. The
// second value, y, increases downward. So, the coordinate 3,0 is to the right
// of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example
// form the following pattern, where # is a dot on the paper and . is an empty,
// unmarked position:

// ...#..#..#.
// ....#......
// ...........
// #..........
// ...#....#.#
// ...........
// ...........
// ...........
// ...........
// ...........
// .#....#.##.
// ....#......
// ......#...#
// #..........
// #.#........

// Then, there is a list of fold instructions. Each instruction indicates a line
// on the transparent paper and wants you to fold the paper up (for horizontal
// y=... lines) or left (for vertical x=... lines). In this example, the first
// fold instruction is fold along y=7, which designates the line formed by all
// of the positions where y is 7 (marked here with -):

// ...#..#..#.
// ....#......
// ...........
// #..........
// ...#....#.#
// ...........
// ...........
// -----------
// ...........
// ...........
// .#....#.##.
// ....#......
// ......#...#
// #..........
// #.#........

// Because this is a horizontal line, fold the bottom half up. Some of the dots
// might end up overlapping after the fold is complete, but dots will never
// appear exactly on a fold line. The result of doing this fold looks like this:

// #.##..#..#.
// #...#......
// ......#...#
// #...#......
// .#.#..#.###
// ...........
// ...........

// Now, only 17 dots are visible.

// Notice, for example, the two dots in the bottom left corner before the
// transparent paper is folded; after the fold is complete, those dots appear in
// the top left corner (at 0,0 and 0,1). Because the paper is transparent, the
// dot just below them in the result (at 0,3) remains visible, as it can be seen
// through the transparent paper.

// Also notice that some dots can end up overlapping; in this case, the dots
// merge together and become a single dot.

// The second fold instruction is fold along x=5, which indicates this line:

// #.##.|#..#.
// #...#|.....
// .....|#...#
// #...#|.....
// .#.#.|#.###
// .....|.....
// .....|.....

// Because this is a vertical line, fold left:

// #####
// #...#
// #...#
// #...#
// #####
// .....
// .....

// The instructions made a square!

// The transparent paper is pretty big, so for now, focus on just completing the
// first fold. After the first fold in the example above, 17 dots are visible -
// dots that end up overlapping after the fold is completed count as a single
// dot.

// How many dots are visible after completing just the first fold instruction on
// your transparent paper?

// --- Part Two ---

// Finish folding the transparent paper according to the instructions. The
// manual says the code is always eight capital letters.

// What code do you use to activate the infrared thermal imaging camera system?

use std::collections::HashSet;
use std::io::{Cursor, Write};

pub fn print_dots<W: Write>(dots: &HashSet<(usize, usize)>, io: &mut W) {
    let min_x = *dots.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *dots.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *dots.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *dots.iter().map(|(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&(x, y)) {
                write!(io, "#").unwrap();
            } else {
                write!(io, " ").unwrap();
            }
        }
        if y != max_y {
            writeln!(io).unwrap();
        }
    }
}

pub fn part_1(s: &str) -> usize {
    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    let mut iter = s.lines();
    for line in &mut iter {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        dots.insert((x.parse().unwrap(), y.parse().unwrap()));
    }

    let first_fold = iter.next().unwrap();
    let (axis, value) = first_fold
        .split_whitespace()
        .last()
        .unwrap()
        .split_once("=")
        .unwrap();
    let value: usize = value.parse().unwrap();

    let mut new_dots = HashSet::new();
    match axis {
        "x" => {
            // fold the paper left
            for (x, y) in dots {
                if x > value {
                    new_dots.insert((value - (x - value), y));
                } else {
                    new_dots.insert((x, y));
                }
            }
        }
        "y" => {
            // fold the paper up
            for (x, y) in dots {
                if y > value {
                    new_dots.insert((x, value - (y - value)));
                } else {
                    new_dots.insert((x, y));
                }
            }
        }
        _ => unreachable!(),
    }
    new_dots.len()
}

pub fn part_2(s: &str) -> String {
    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    let mut iter = s.lines();
    for line in &mut iter {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        dots.insert((x.parse().unwrap(), y.parse().unwrap()));
    }

    for fold in iter {
        let (axis, value) = fold
            .split_whitespace()
            .last()
            .unwrap()
            .split_once("=")
            .unwrap();
        let value: usize = value.parse().unwrap();

        let mut new_dots = HashSet::new();
        match axis {
            "x" => {
                // fold the paper left
                for (x, y) in dots {
                    if x > value {
                        new_dots.insert((value - (x - value), y));
                    } else {
                        new_dots.insert((x, y));
                    }
                }
            }
            "y" => {
                // fold the paper up
                for (x, y) in dots {
                    if y > value {
                        new_dots.insert((x, value - (y - value)));
                    } else {
                        new_dots.insert((x, y));
                    }
                }
            }
            _ => unreachable!(),
        }
        dots = new_dots;
    }
    let mut s = Vec::new();
    print_dots(&dots, &mut Cursor::new(&mut s));
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn test_day_13_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 17);
    }

    #[test]
    fn test_day_13_part_1() {
        assert_eq!(part_1(include_str!("input/day_13.txt")), 942);
    }

    #[test]
    fn test_day_13_example_part_2() {
        let answer = part_2(EXAMPLE);
        assert_eq!(
            answer,
            r#"
#####
#   #
#   #
#   #
#####"#
                .trim()
                .to_string()
        );
    }

    #[test]
    fn test_day_13_part_2() {
        let answer = part_2(include_str!("input/day_13.txt"));
        assert_eq!(
            answer,
            r#"  ## ####  ##  #  #  ##  ###  ###  ### 
   #    # #  # #  # #  # #  # #  # #  #
   #   #  #    #  # #  # #  # #  # ### 
   #  #   # ## #  # #### ###  ###  #  #
#  # #    #  # #  # #  # #    # #  #  #
 ##  ####  ###  ##  #  # #    #  # ### "#
                .to_string()
        );
    }
}
