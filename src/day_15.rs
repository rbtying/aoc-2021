// --- Day 15: Chiton ---

// You've almost reached the exit of the cave, but the walls are getting closer
// together. Your submarine can barely still fit, though; the main problem is
// that the walls of the cave are covered in chitons, and it would be best not
// to bump any of them.

// The cavern is large, but has a very low ceiling, restricting your motion to
// two dimensions. The shape of the cavern resembles a square; a quick scan of
// chiton density produces a map of risk level throughout the cave (your puzzle
// input). For example:

// 1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581

// You start in the top left position, your destination is the bottom right
// position, and you cannot move diagonally. The number at each position is its
// risk level; to determine the total risk of an entire path, add up the risk
// levels of each position you enter (that is, don't count the risk level of
// your starting position unless you enter it; leaving it adds no risk to your
// total).

// Your goal is to find a path with the lowest total risk. In this example, a
// path with the lowest total risk is highlighted here:

// 1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581

// The total risk of this path is 40 (the starting position is never entered, so
// its risk is not counted).

// What is the lowest total risk of any path from the top left to the bottom
// right?

// --- Part Two ---

// Now that you know how to find low-risk paths in the cave, you can try to find
// your way out.

// The entire cave is actually five times larger in both dimensions than you
// thought; the area you originally scanned is just one tile in a 5x5 tile area
// that forms the full map. Your original map tile repeats to the right and
// downward; each time the tile repeats to the right or downward, all of its
// risk levels are 1 higher than the tile immediately up or left of it. However,
// risk levels above 9 wrap back around to 1. So, if your original map had some
// position with a risk level of 8, then that same position on each of the 25
// total tiles would be as follows:

// 8 9 1 2 3
// 9 1 2 3 4
// 1 2 3 4 5
// 2 3 4 5 6
// 3 4 5 6 7

// Each single digit above corresponds to the example position with a value of 8
// on the top-left tile. Because the full map is actually five times larger in
// both dimensions, that position appears a total of 25 times, once in each
// duplicated tile, with the values shown above.

// Equipped with the full map, you can now find a path from the top left corner
// to the bottom right corner with the lowest total risk:

// The total risk of this path is 315 (the starting position is still never
// entered, so its risk is not counted).

// Using the full map, what is the lowest total risk of any path from the top
// left to the bottom right?

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

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
        .filter(move |(i_, j_)| *i_ == i || *j_ == j)
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

    let start = (0, 0);
    let end = (rows.len() - 1, rows[0].len() - 1);

    let mut dist = HashMap::new();

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: (usize, usize),
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            dist.insert((i, j), std::usize::MAX);
        }
    }
    dist.insert(start, 0);
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return cost;
        }

        if cost > dist[&position] {
            continue;
        }
        let (i, j) = position;

        for (i_, j_) in adjacent_points(i, j, rows.len(), rows[0].len()) {
            let next = State {
                cost: cost + rows[i_][j_] as usize,
                position: (i_, j_),
            };

            if next.cost < dist[&next.position] {
                heap.push(next);
                dist.insert(next.position, next.cost);
            }
        }
    }

    unreachable!()
}

pub fn part_2(s: &str) -> usize {
    // repeat s 5 times by 5 times
    let mut s_ = String::new();

    for i in 0..5 {
        for line in s.lines() {
            for j in 0..5 {
                for c in line.chars() {
                    let mut v = c.to_digit(10).unwrap() + i + j;
                    while v > 9 {
                        v -= 9;
                    }

                    s_ += &(v.to_string());
                }
            }
            s_ += "\n";
        }
    }

    part_1(&s_)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    #[test]
    fn test_day_15_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 40);
    }

    #[test]
    fn test_day_15_part_1() {
        assert_eq!(part_1(include_str!("input/day_15.txt")), 595);
    }

    #[test]
    fn test_day_15_example_part_2() {
        let answer = part_2(EXAMPLE);
        assert_eq!(answer, 315);
    }

    #[test]
    fn test_day_15_part_2() {
        let answer = part_2(include_str!("input/day_15.txt"));
        assert_eq!(answer, 2914);
    }
}
