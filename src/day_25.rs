// --- Day 25: Sea Cucumber ---
// This is it: the bottom of the ocean trench, the last place the sleigh keys could be. Your
// submarine's experimental antenna still isn't boosted enough to detect the keys, but they must be
// here. All you need to do is reach the seafloor and find them.

// At least, you'd touch down on the seafloor if you could; unfortunately, it's completely covered
// by two large herds of sea cucumbers, and there isn't an open space large enough for your
// submarine.

// You suspect that the Elves must have done this before, because just then you discover the phone
// number of a deep-sea marine biologist on a handwritten note taped to the wall of the submarine's
// cockpit.

// "Sea cucumbers? Yeah, they're probably hunting for food. But don't worry, they're predictable
// critters: they move in perfectly straight lines, only moving forward when there's space to do
// so. They're actually quite polite!"

// You explain that you'd like to predict when you could land your submarine.

// "Oh that's easy, they'll eventually pile up and leave enough space for-- wait, did you say
// submarine? And the only place with that many sea cucumbers would be at the very bottom of the
// Mariana--" You hang up the phone.

// There are two herds of sea cucumbers sharing the same region; one always moves east (>), while
// the other always moves south (v). Each location can contain at most one sea cucumber; the
// remaining locations are empty (.). The submarine helpfully generates a map of the situation
// (your puzzle input). For example:

// v...>>.vv>
// .vv>>.vv..
// >>.>v>...v
// >>v>>.>.v.
// v>v.vv.v..
// >.>>..v...
// .vv..>.>v.
// v.v..>>v.v
// ....v..v.>
//
// Every step, the sea cucumbers in the east-facing herd attempt to move forward one location, then
// the sea cucumbers in the south-facing herd attempt to move forward one location. When a herd
// moves forward, every sea cucumber in the herd first simultaneously considers whether there is a
// sea cucumber in the adjacent location it's facing (even another sea cucumber facing the same
// direction), and then every sea cucumber facing an empty location simultaneously moves into that
// location.

// So, in a situation like this:

// ...>>>>>...
// After one step, only the rightmost sea cucumber would have moved:

// ...>>>>.>..
// After the next step, two sea cucumbers move:

// ...>>>.>.>.
// During a single step, the east-facing herd moves first, then the south-facing herd moves. So,
// given this situation:

// ..........
// .>v....v..
// .......>..
// ..........
// After a single step, of the sea cucumbers on the left, only the south-facing sea cucumber has
// moved (as it wasn't out of the way in time for the east-facing cucumber on the left to move),
// but both sea cucumbers on the right have moved (as the east-facing sea cucumber moved out of the
// way of the south-facing sea cucumber):

// ..........
// .>........
// ..v....v>.
// ..........
// Due to strong water currents in the area, sea cucumbers that move off the right edge of the map
// appear on the left edge, and sea cucumbers that move off the bottom edge of the map appear on
// the top edge. Sea cucumbers always check whether their destination location is empty before
// moving, even if that destination is on the opposite side of the map:

// Initial state:
// ...>...
// .......
// ......>
// v.....>
// ......>
// .......
// ..vvv..

// After 1 step:
// ..vv>..
// .......
// >......
// v.....>
// >......
// .......
// ....v..

// After 2 steps:
// ....v>.
// ..vv...
// .>.....
// ......>
// v>.....
// .......
// .......

// After 3 steps:
// ......>
// ..v.v..
// ..>v...
// >......
// ..>....
// v......
// .......

// After 4 steps:
// >......
// ..v....
// ..>.v..
// .>.v...
// ...>...
// .......
// v......
// To find a safe place to land your submarine, the sea cucumbers need to stop moving. Again
// consider the first example:

// Initial state:
// v...>>.vv>
// .vv>>.vv..
// >>.>v>...v
// >>v>>.>.v.
// v>v.vv.v..
// >.>>..v...
// .vv..>.>v.
// v.v..>>v.v
// ....v..v.>

// After 1 step:
// ....>.>v.>
// v.v>.>v.v.
// >v>>..>v..
// >>v>v>.>.v
// .>v.v...v.
// v>>.>vvv..
// ..v...>>..
// vv...>>vv.
// >.v.v..v.v

// After 2 steps:
// >.v.v>>..v
// v.v.>>vv..
// >v>.>.>.v.
// >>v>v.>v>.
// .>..v....v
// .>v>>.v.v.
// v....v>v>.
// .vv..>>v..
// v>.....vv.

// After 3 steps:
// v>v.v>.>v.
// v...>>.v.v
// >vv>.>v>..
// >>v>v.>.v>
// ..>....v..
// .>.>v>v..v
// ..v..v>vv>
// v.v..>>v..
// .v>....v..

// After 4 steps:
// v>..v.>>..
// v.v.>.>.v.
// >vv.>>.v>v
// >>.>..v>.>
// ..v>v...v.
// ..>>.>vv..
// >.v.vv>v.v
// .....>>vv.
// vvv>...v..

// After 5 steps:
// vv>...>v>.
// v.v.v>.>v.
// >.v.>.>.>v
// >v>.>..v>>
// ..v>v.v...
// ..>.>>vvv.
// .>...v>v..
// ..v.v>>v.v
// v.v.>...v.

// ...

// After 10 steps:
// ..>..>>vv.
// v.....>>.v
// ..v.v>>>v>
// v>.>v.>>>.
// ..v>v.vv.v
// .v.>>>.v..
// v.v..>v>..
// ..v...>v.>
// .vv..v>vv.

// ...

// After 20 steps:
// v>.....>>.
// >vv>.....v
// .>v>v.vv>>
// v>>>v.>v.>
// ....vv>v..
// .v.>>>vvv.
// ..v..>>vv.
// v.v...>>.v
// ..v.....v>

// ...

// After 30 steps:
// .vv.v..>>>
// v>...v...>
// >.v>.>vv.>
// >v>.>.>v.>
// .>..v.vv..
// ..v>..>>v.
// ....v>..>v
// v.v...>vv>
// v.v...>vvv

// ...

// After 40 steps:
// >>v>v..v..
// ..>>v..vv.
// ..>>>v.>.v
// ..>>>>vvv>
// v.....>...
// v.v...>v>>
// >vv.....v>
// .>v...v.>v
// vvv.v..v.>

// ...

// After 50 steps:
// ..>>v>vv.v
// ..v.>>vv..
// v.>>v>>v..
// ..>>>>>vv.
// vvv....>vv
// ..v....>>>
// v>.......>
// .vv>....v>
// .>v.vv.v..

// ...

// After 55 steps:
// ..>>v>vv..
// ..v.>>vv..
// ..>>v>>vv.
// ..>>>>>vv.
// v......>vv
// v>v....>>v
// vvv...>..>
// >vv.....>.
// .>v.vv.v..

// After 56 steps:
// ..>>v>vv..
// ..v.>>vv..
// ..>>v>>vv.
// ..>>>>>vv.
// v......>vv
// v>v....>>v
// vvv....>.>
// >vv......>
// .>v.vv.v..

// After 57 steps:
// ..>>v>vv..
// ..v.>>vv..
// ..>>v>>vv.
// ..>>>>>vv.
// v......>vv
// v>v....>>v
// vvv.....>>
// >vv......>
// .>v.vv.v..

// After 58 steps:
// ..>>v>vv..
// ..v.>>vv..
// ..>>v>>vv.
// ..>>>>>vv.
// v......>vv
// v>v....>>v
// vvv.....>>
// >vv......>
// .>v.vv.v..
// In this example, the sea cucumbers stop moving after 58 steps.

// Find somewhere safe to land your submarine. What is the first step on which no sea cucumbers
// move?

pub fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        for c in row {
            eprint!("{}", c);
        }
        eprintln!();
    }
}

pub fn part_1(s: &str) -> usize {
    let mut grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    let h = grid.len();
    let w = grid[0].len();

    for step in 0.. {
        let mut moves = vec![];

        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                match c {
                    '.' | 'v' => (),
                    '>' => {
                        let next_pos = (i, (j + 1) % w);
                        if grid[next_pos.0][next_pos.1] == '.' {
                            moves.push(((i, j), next_pos, '>'));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        let mut num_moves = moves.len();

        // update the grid
        for (prev, _, _) in &moves {
            grid[prev.0][prev.1] = '.';
        }
        for (_, next, c) in moves {
            grid[next.0][next.1] = c;
        }
        let mut moves = vec![];

        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                match c {
                    '.' | '>' => (),
                    'v' => {
                        let next_pos = ((i + 1) % h, j);
                        if grid[next_pos.0][next_pos.1] == '.' {
                            moves.push(((i, j), next_pos, 'v'));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        num_moves += moves.len();

        // update the grid
        for (prev, _, _) in &moves {
            grid[prev.0][prev.1] = '.';
        }
        for (_, next, c) in moves {
            grid[next.0][next.1] = c;
        }

        if num_moves == 0 {
            return step + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::part_1;

    #[test]
    fn test_day_25_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 58);
    }

    #[test]
    fn test_day_25_part_1() {
        assert_eq!(part_1(include_str!("input/day_25.txt")), 557);
    }

    const EXAMPLE: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;
}
