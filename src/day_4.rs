use std::fmt;
///
/// --- Day 4: Giant Squid ---
// You're already almost 1.5km (almost a mile) below the surface of the ocean,
// already so deep that you can't see any sunlight. What you can see, however,
// is a giant squid that has attached itself to the outside of your submarine.

// Maybe it wants to play bingo?

// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
// Numbers are chosen at random, and the chosen number is marked on all boards
// on which it appears. (Numbers may not appear on all boards.) If all numbers
// in any row or any column of a board are marked, that board wins. (Diagonals
// don't count.)

// The submarine has a bingo subsystem to help passengers (currently, you and
// the giant squid) pass the time. It automatically generates a random order in
// which to draw numbers and a random set of boards (your puzzle input). For
// example:

// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

// 22 13 17 11  0
//  8  2 23  4 24
// 21  9 14 16  7
//  6 10  3 18  5
//  1 12 20 15 19

//  3 15  0  2 22
//  9 18 13 17  5
// 19  8  7 25 23
// 20 11 10 24  4
// 14 21 16 12  6

// 14 21 17 24  4
// 10 16 15  9 19
// 18  8 23 26 20
// 22 11 13  6  5
//  2  0 12  3  7

// After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no
// winners, but the boards are marked as follows (shown here adjacent to each
// other to save space):

// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

// After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are
// still no winners:

// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

// Finally, 24 is drawn:

// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7

// At this point, the third board wins because it has at least one complete row
// or column of marked numbers (in this case, the entire top row is marked: 14
// 21 17 24 4).

// The score of the winning board can now be calculated. Start by finding the
// sum of all unmarked numbers on that board; in this case, the sum is 188.
// Then, multiply that sum by the number that was just called when the board
// won, 24, to get the final score, 188 * 24 = 4512.

// To guarantee victory against the giant squid, figure out which board will win
// first. What will your final score be if you choose that board?

// --- Part Two ---

// On the other hand, it might be wise to try a different strategy: let the
// giant squid win.

// You aren't sure how many bingo boards a giant squid could play at once, so
// rather than waste time counting its arms, the safe thing to do is to figure
// out which board will win last and choose that one. That way, no matter which
// boards it picks, it will win for sure.

// In the above example, the second board is the last to win, which happens
// after 13 is eventually called and its middle column is completely marked. If
// you were to keep playing until this point, the second board would have a sum
// of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.

// Figure out which board will win last. Once it wins, what would its final
// score be?
use std::str::FromStr;

pub fn parse_board<'a, I: Iterator<Item = &'a str>>(mut s: I) -> (I, Board) {
    let mut board = [[0u8; 5]; 5];

    for i in 0..5 {
        let l = s.next().unwrap();
        let mut l_it = l.split_whitespace();

        for j in 0..5 {
            board[i][j] = u8::from_str(l_it.next().unwrap()).unwrap();
        }
    }

    (s, Board::new(board))
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board {
    board: [[u8; 5]; 5],
    hits: u32,
}

impl Board {
    pub fn new(board: [[u8; 5]; 5]) -> Self {
        Self { board, hits: 0 }
    }

    pub fn hit(&mut self, v: u8) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == v {
                    let offset = i * 5 + j;
                    self.hits |= 1 << offset;
                    return true;
                }
            }
        }
        return false;
    }

    pub fn sum_unmarked(&self) -> u64 {
        let mut sum = 0u64;

        for i in 0..5 {
            for j in 0..5 {
                let offset = i * 5 + j;
                if self.hits & (1 << offset) == 0 {
                    sum += self.board[i][j] as u64;
                }
            }
        }

        sum
    }

    pub fn is_bingo(&self) -> bool {
        // Horizontals
        (0..5).any(|v| self.hits & (0b11111 << v * 5) == (0b11111 << v * 5)) ||
        // Verticals
        (0..5).any(|v| self.hits & (0b100001000010000100001 << v) == (0b100001000010000100001 << v))
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for i in 0..5 {
            for j in 0..5 {
                let hit = self.hits & 1 << (i * 5 + j) != 0;
                if hit {
                    write!(f, "(")?;
                } else {
                    write!(f, " ")?;
                }
                write!(f, "{:>3}", self.board[i][j])?;
                if hit {
                    write!(f, ")")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse(s: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = s.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|v| u8::from_str(v).unwrap())
        .collect::<Vec<_>>();
    let mut boards = vec![];

    while lines.next().is_some() {
        let (lines_, board) = parse_board(lines);
        lines = lines_;
        boards.push(board);
    }

    (numbers, boards)
}

pub fn part_1(numbers: Vec<u8>, mut boards: Vec<Board>) -> u64 {
    for number in numbers {
        for board in boards.iter_mut() {
            if board.hit(number) {
                if board.is_bingo() {
                    return number as u64 * board.sum_unmarked();
                }
            }
        }
    }
    unimplemented!()
}

pub fn part_2(numbers: Vec<u8>, mut boards: Vec<Board>) -> u64 {
    let mut completion_sequence = vec![0; boards.len()];
    let mut seq_start = 1;
    for number in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            if completion_sequence[idx] == 0 {
                assert!(!board.is_bingo());
                if board.hit(number) {
                    if board.is_bingo() {
                        completion_sequence[idx] = seq_start;
                        seq_start += 1;
                    }
                }
            }
        }
        if seq_start == boards.len() + 1 {
            return number as u64
                * boards[(0..completion_sequence.len())
                    .find(|idx| completion_sequence[*idx] == seq_start - 1)
                    .unwrap()]
                .sum_unmarked();
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{parse, part_1, part_2};

    const EXAMPLE_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn test_day_4_example_part_1() {
        let (numbers, boards) = parse(EXAMPLE_INPUT);
        assert_eq!(part_1(numbers, boards), 4512);
    }

    #[test]
    fn test_day_4_part_1() {
        let (numbers, boards) = parse(include_str!("input/day_4.txt"));
        assert_eq!(part_1(numbers, boards), 23177);
    }

    #[test]
    fn test_day_4_example_part_2() {
        let (numbers, boards) = parse(EXAMPLE_INPUT);
        assert_eq!(part_2(numbers, boards), 1924);
    }

    #[test]
    fn test_day_4_part_2() {
        let (numbers, boards) = parse(include_str!("input/day_4.txt"));
        assert_eq!(part_2(numbers, boards), 6804);
    }
}
