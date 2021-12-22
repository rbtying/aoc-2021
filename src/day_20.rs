// --- Day 20: Trench Map ---

// With the scanners fully deployed, you turn their attention to mapping the
// floor of the ocean trench.

// When you get back the image from the scanners, it seems to just be random
// noise. Perhaps you can combine an image enhancement algorithm and the input
// image (your puzzle input) to clean it up a little.

// For example:

// ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
// #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
// .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
// .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
// .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
// ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
// ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

// #..#.
// #....
// ##..#
// ..#..
// ..###

// The first section is the image enhancement algorithm. It is normally given on
// a single line, but it has been wrapped to multiple lines in this example for
// legibility. The second section is the input image, a two-dimensional grid of
// light pixels (#) and dark pixels (.).

// The image enhancement algorithm describes how to enhance an image by
// simultaneously converting all pixels in the input image into an output image.
// Each pixel of the output image is determined by looking at a 3x3 square of
// pixels centered on the corresponding input image pixel. So, to determine the
// value of the pixel at (5,10) in the output image, nine pixels from the input
// image need to be considered: (4,9), (4,10), (4,11), (5,9), (5,10), (5,11),
// (6,9), (6,10), and (6,11). These nine input pixels are combined into a single
// binary number that is used as an index in the image enhancement algorithm
// string.

// For example, to determine the output pixel that corresponds to the very
// middle pixel of the input image, the nine pixels marked by [...] would need
// to be considered:

// # . . # .
// #[. . .].
// #[# . .]#
// .[. # .].
// . . # # #

// Starting from the top-left and reading across each row, these pixels are ...,
// then #.., then .#.; combining these forms ...#...#.. By turning dark pixels
// (.) into 0 and light pixels (#) into 1, the binary number 000100010 can be
// formed, which is 34 in decimal.

// The image enhancement algorithm string is exactly 512 characters long, enough
// to match every possible 9-bit binary number. The first few characters of the
// string (numbered starting from zero) are as follows:

// 0         10        20        30  34    40        50        60        70
// |         |         |         |   |     |         |         |         |
// ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##

// In the middle of this first group of characters, the character at index 34
// can be found: #. So, the output pixel in the center of the output image
// should be #, a light pixel.

// This process can then be repeated to calculate every pixel of the output
// image.

// Through advances in imaging technology, the images being operated on here are
// infinite in size. Every pixel of the infinite output image needs to be
// calculated exactly based on the relevant pixels of the input image. The small
// input image you have is only a small region of the actual infinite input
// image; the rest of the input image consists of dark pixels (.). For the
// purposes of the example, to save on space, only a portion of the
// infinite-sized input and output images will be shown.

// The starting input image, therefore, looks something like this, with more
// dark pixels (.) extending forever in every direction not shown here:

// ...............
// ...............
// ...............
// ...............
// ...............
// .....#..#......
// .....#.........
// .....##..#.....
// .......#.......
// .......###.....
// ...............
// ...............
// ...............
// ...............
// ...............

// By applying the image enhancement algorithm to every pixel simultaneously,
// the following output image can be obtained:

// ...............
// ...............
// ...............
// ...............
// .....##.##.....
// ....#..#.#.....
// ....##.#..#....
// ....####..#....
// .....#..##.....
// ......##..#....
// .......#.#.....
// ...............
// ...............
// ...............
// ...............

// Through further advances in imaging technology, the above output image can
// also be used as an input image! This allows it to be enhanced a second time:

// ...............
// ...............
// ...............
// ..........#....
// ....#..#.#.....
// ...#.#...###...
// ...#...##.#....
// ...#.....#.#...
// ....#.#####....
// .....#.#####...
// ......##.##....
// .......###.....
// ...............
// ...............
// ...............

// Truly incredible - now the small details are really starting to come through.
// After enhancing the original input image twice, 35 pixels are lit.

// Start with the original input image and apply the image enhancement algorithm
// twice, being careful to account for the infinite size of the images. How many
// pixels are lit in the resulting image?

// --- Part Two ---

// You still can't quite make out the details in the image. Maybe you just
// didn't enhance it enough.

// If you enhance the starting input image in the above example a total of 50
// times, 3351 pixels are lit in the final output image.

// Start again with the original input image and apply the image enhancement
// algorithm 50 times. How many pixels are lit in the resulting image?

use std::collections::HashSet;

pub fn print_img(img: &HashSet<(isize, isize)>) {
    let min_x = img.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = img.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = img.iter().map(|(y, _)| *y).min().unwrap();
    let max_y = img.iter().map(|(y, _)| *y).max().unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if img.contains(&(x, y)) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
}

pub fn get_pix(
    img: &HashSet<(isize, isize)>,
    coord: (isize, isize),
    bounds: (isize, isize, isize, isize),
    border_val: bool,
) -> usize {
    let mut v = 0;

    for offset_x in [-1, 0, 1] {
        for offset_y in [-1, 0, 1] {
            let pos = (coord.0 + offset_x, coord.1 + offset_y);
            let in_bounds = (pos.0 >= bounds.0 && pos.0 <= bounds.1)
                && (pos.1 >= bounds.2 && pos.1 <= bounds.3);
            if img.contains(&pos) || (!in_bounds && border_val) {
                v |= 1;
            }
            v <<= 1;
        }
    }

    v >> 1
}

#[allow(clippy::type_complexity)]
fn enhance_img(
    img: &HashSet<(isize, isize)>,
    algo: &[bool],
    bounds: (isize, isize, isize, isize),
    border_val: bool,
) -> (HashSet<(isize, isize)>, bool, (isize, isize, isize, isize)) {
    let (min_x, max_x, min_y, max_y) = bounds;

    let mut out = HashSet::new();

    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            let idx = get_pix(img, (x, y), bounds, border_val);
            if algo[idx] {
                out.insert((x, y));
            }
        }
    }

    (
        out,
        algo[get_pix(img, (min_x - 1, min_y - 1), bounds, border_val)],
        (min_x - 1, max_x + 1, min_y - 1, max_y + 1),
    )
}

pub fn part_1(s: &str) -> usize {
    let mut iter = s.lines();
    let algo = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect::<Vec<bool>>();
    assert_eq!(algo.len(), 512);

    let _ = iter.next().unwrap();

    let mut lit: HashSet<(isize, isize)> = HashSet::new();

    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in iter.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                lit.insert((row as isize, col as isize));
            }
            max_col = max_col.max(col);
        }
        max_row = row;
    }

    let (e1, e1b, e1bv) = enhance_img(
        &lit,
        &algo,
        (0, max_row as isize, 0, max_col as isize),
        false,
    );
    let (e2, _, _) = enhance_img(&e1, &algo, e1bv, e1b);

    e2.len()
}

pub fn part_2(s: &str) -> usize {
    let mut iter = s.lines();
    let algo = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect::<Vec<bool>>();
    assert_eq!(algo.len(), 512);

    let _ = iter.next().unwrap();

    let mut lit: HashSet<(isize, isize)> = HashSet::new();

    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in iter.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                lit.insert((row as isize, col as isize));
            }
            max_col = max_col.max(col);
        }
        max_row = row;
    }

    let mut e = (lit, false, (0, max_row as isize, 0, max_col as isize));
    for _ in 0..50 {
        e = enhance_img(&e.0, &algo, e.2, e.1);
    }
    print_img(&e.0);

    e.0.len()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_day_20_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 35);
    }

    #[test]
    fn test_day_20_part_1() {
        assert_eq!(part_1(include_str!("input/day_20.txt")), 5291);
    }

    #[test]
    fn test_day_20_example_part_2() {
        assert_eq!(part_2(EXAMPLE), 3351);
    }

    #[test]
    fn test_day_20_part_2() {
        let answer = part_2(include_str!("input/day_20.txt"));
        assert_eq!(answer, 16665);
    }

    const EXAMPLE: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;
}
