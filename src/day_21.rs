// --- Day 21: Dirac Dice ---

// There's not much to do as you slowly descend to the bottom of the ocean. The
// submarine computer challenges you to a nice game of Dirac Dice.

// This game consists of a single die, two pawns, and a game board with a
// circular track containing ten spaces marked 1 through 10 clockwise. Each
// player's starting space is chosen randomly (your puzzle input). Player 1 goes
// first.

// Players take turns moving. On each player's turn, the player rolls the die
// three times and adds up the results. Then, the player moves their pawn that
// many times forward around the track (that is, moving clockwise on spaces in
// order of increasing value, wrapping back around to 1 after 10). So, if a
// player is on space 7 and they roll 2, 2, and 1, they would move forward 5
// times, to spaces 8, 9, 10, 1, and finally stopping on 2.

// After each player moves, they increase their score by the value of the space
// their pawn stopped on. Players' scores start at 0. So, if the first player
// starts on space 7 and rolls a total of 5, they would stop on space 2 and add
// 2 to their score (for a total score of 2). The game immediately ends as a win
// for any player whose score reaches at least 1000.

// Since the first game is a practice game, the submarine opens a compartment
// labeled deterministic dice and a 100-sided die falls out. This die always
// rolls 1 first, then 2, then 3, and so on up to 100, after which it starts
// over at 1 again. Play using this die.

// For example, given these starting positions:

// Player 1 starting position: 4
// Player 2 starting position: 8

// This is how the game would go:

//     Player 1 rolls 1+2+3 and moves to space 10 for a total score of 10.
//     Player 2 rolls 4+5+6 and moves to space 3 for a total score of 3.
//     Player 1 rolls 7+8+9 and moves to space 4 for a total score of 14.
//     Player 2 rolls 10+11+12 and moves to space 6 for a total score of 9.
//     Player 1 rolls 13+14+15 and moves to space 6 for a total score of 20.
//     Player 2 rolls 16+17+18 and moves to space 7 for a total score of 16.
//     Player 1 rolls 19+20+21 and moves to space 6 for a total score of 26.
//     Player 2 rolls 22+23+24 and moves to space 6 for a total score of 22.

// ...after many turns...

//     Player 2 rolls 82+83+84 and moves to space 6 for a total score of 742.
//     Player 1 rolls 85+86+87 and moves to space 4 for a total score of 990.
//     Player 2 rolls 88+89+90 and moves to space 3 for a total score of 745.
//     Player 1 rolls 91+92+93 and moves to space 10 for a final score, 1000.

// Since player 1 has at least 1000 points, player 1 wins and the game ends. At
// this point, the losing player had 745 points and the die had been rolled a
// total of 993 times; 745 * 993 = 739785.

// Play a practice game using the deterministic 100-sided die. The moment either
// player wins, what do you get if you multiply the score of the losing player
// by the number of times the die was rolled during the game?

// --- Part Two ---

// Now that you're warmed up, it's time to play the real game.

// A second compartment opens, this time labeled Dirac dice. Out of it falls a
// single three-sided die.

// As you experiment with the die, you feel a little strange. An informational
// brochure in the compartment explains that this is a quantum die: when you
// roll it, the universe splits into multiple copies, one copy for each possible
// outcome of the die. In this case, rolling the die always splits the universe
// into three copies: one where the outcome of the roll was 1, one where it was
// 2, and one where it was 3.

// The game is played the same as before, although to prevent things from
// getting too far out of hand, the game now ends when either player's score
// reaches at least 21.

// Using the same starting positions as in the example above, player 1 wins in
// 444356092776315 universes, while player 2 merely wins in 341960390180808
// universes.

// Using your given starting positions, determine every possible outcome. Find
// the player that wins in more universes; in how many universes does that
// player win?

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeterministicDice(u8, usize);

impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice(0, 0)
    }

    fn roll(self) -> (DeterministicDice, usize) {
        let next_val = self.0 + 1;
        if next_val == 100 {
            (DeterministicDice(0, self.1 + 1), next_val as usize)
        } else {
            (DeterministicDice(next_val, self.1 + 1), next_val as usize)
        }
    }

    fn count(self) -> usize {
        self.1
    }
}

pub fn part_1(s: &str) -> usize {
    let mut iter = s.lines();
    let mut p1_position: usize = iter
        .next()
        .unwrap()
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let mut p2_position: usize = iter
        .next()
        .unwrap()
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut dice = DeterministicDice::new();
    loop {
        let mut p1_roll = 0;
        for _ in 0..3 {
            let (dice_, v) = dice.roll();
            p1_roll += v;
            dice = dice_
        }

        p1_position += p1_roll;
        while p1_position > 10 {
            p1_position -= 10;
        }
        p1_score += p1_position;

        if p1_score >= 1000 {
            break;
        }

        let mut p2_roll = 0;
        for _ in 0..3 {
            let (dice_, v) = dice.roll();
            p2_roll += v;
            dice = dice_
        }

        p2_position += p2_roll;
        while p2_position > 10 {
            p2_position -= 10;
        }
        p2_score += p2_position;

        if p2_score >= 1000 {
            break;
        }
    }

    p1_score.min(p2_score) * dice.count()
}

pub fn part_2(s: &str) -> usize {
    let mut iter = s.lines();
    let p1_position: usize = iter
        .next()
        .unwrap()
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let p2_position: usize = iter
        .next()
        .unwrap()
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    #[derive(Copy, Clone)]
    enum Roll {
        P1Roll,
        P1Score(usize),
        P2Roll,
        P2Score(usize),
    }

    #[derive(Copy, Clone)]
    struct State {
        p1_score: usize,
        p2_score: usize,
        p1_pos: usize,
        p2_pos: usize,
        roll: Roll,
        win_multiplier: usize,
    }

    let mut stk = vec![State {
        p1_score: 0,
        p2_score: 0,
        p1_pos: p1_position,
        p2_pos: p2_position,
        roll: Roll::P1Roll,
        win_multiplier: 1,
    }];

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    let mut d = [0u8; 10];

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                d[i + j + k] += 1;
            }
        }
    }

    while let Some(s) = stk.pop() {
        match s.roll {
            Roll::P1Roll => {
                for (i, x) in d.iter().enumerate().skip(3) {
                    if *x > 0 {
                        stk.push(State {
                            roll: Roll::P1Score(i),
                            win_multiplier: s.win_multiplier * *x as usize,
                            ..s
                        });
                    }
                }
            }
            Roll::P1Score(v) => {
                let mut p1_position = s.p1_pos + v;
                while p1_position > 10 {
                    p1_position -= 10;
                }
                let p1_score = s.p1_score + p1_position;

                if p1_score >= 21 {
                    p1_wins += s.win_multiplier;
                } else {
                    stk.push(State {
                        p1_pos: p1_position,
                        p1_score,
                        roll: Roll::P2Roll,
                        ..s
                    })
                }
            }
            Roll::P2Roll => {
                for (i, x) in d.iter().enumerate().skip(3) {
                    if *x > 0 {
                        stk.push(State {
                            roll: Roll::P2Score(i),
                            win_multiplier: s.win_multiplier * *x as usize,
                            ..s
                        });
                    }
                }
            }
            Roll::P2Score(v) => {
                let mut p2_position = s.p2_pos + v;
                while p2_position > 10 {
                    p2_position -= 10;
                }
                let p2_score = s.p2_score + p2_position;

                if p2_score >= 21 {
                    p2_wins += s.win_multiplier;
                } else {
                    stk.push(State {
                        p2_pos: p2_position,
                        p2_score,
                        roll: Roll::P1Roll,
                        ..s
                    })
                }
            }
        }
    }

    p1_wins.max(p2_wins)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_day_21_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 739785);
    }

    #[test]
    fn test_day_21_part_1() {
        assert_eq!(part_1(include_str!("input/day_21.txt")), 713328);
    }

    #[test]
    fn test_day_21_example_part_2() {
        assert_eq!(part_2(EXAMPLE), 444356092776315);
    }

    #[test]
    fn test_day_21_part_2() {
        let answer = part_2(include_str!("input/day_21.txt"));
        assert_eq!(answer, 92399285032143);
    }

    const EXAMPLE: &str = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;
}
