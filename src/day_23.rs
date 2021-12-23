// --- Day 23: Amphipod ---

// A group of amphipods notice your fancy submarine and flag you down. "With
// such an impressive shell," one amphipod says, "surely you can help us with a
// question that has stumped our best scientists."

// They go on to explain that a group of timid, stubborn amphipods live in a
// nearby burrow. Four types of amphipods live there: Amber (A), Bronze (B),
// Copper (C), and Desert (D). They live in a burrow that consists of a hallway
// and four side rooms. The side rooms are initially full of amphipods, and the
// hallway is initially empty.

// They give you a diagram of the situation (your puzzle input), including
// locations of each amphipod (A, B, C, or D, each of which is occupying an
// otherwise open space), walls (#), and open space (.).

// For example:

// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########

// The amphipods would like a method to organize every amphipod into side rooms
// so that each side room contains one type of amphipod and the types are sorted
// A-D going left to right, like this:

// #############
// #...........#
// ###A#B#C#D###
//   #A#B#C#D#
//   #########

// Amphipods can move up, down, left, or right so long as they are moving into
// an unoccupied open space. Each type of amphipod requires a different amount
// of energy to move one step: Amber amphipods require 1 energy per step, Bronze
// amphipods require 10 energy, Copper amphipods require 100, and Desert ones
// require 1000. The amphipods would like you to find a way to organize the
// amphipods that requires the least total energy.

// However, because they are timid and stubborn, the amphipods have some extra
// rules:

//     Amphipods will never stop on the space immediately outside any room. They
//     can move into that space so long as they immediately continue moving.
//     (Specifically, this refers to the four open spaces in the hallway that
//     are directly above an amphipod starting position.)
//     Amphipods will never move from the hallway into a room unless that room
//     is their destination room and that room contains no amphipods which do
//     not also have that room as their own destination. If an amphipod's
//     starting room is not its destination room, it can stay in that room until
//     it leaves the room. (For example, an Amber amphipod will not move from
//     the hallway into the right three rooms, and will only move into the
//     leftmost room if that room is empty or if it only contains other Amber
//     amphipods.)
//     Once an amphipod stops moving in the hallway, it will stay in that spot
//     until it can move into a room. (That is, once any amphipod starts moving,
//     any other amphipods currently in the hallway are locked in place and will
//     not move again until they can move fully into a room.)

// In the above example, the amphipods can be organized using a minimum of 12521
// energy. 

// What is the least energy required to organize the amphipods?

// --- Part Two ---

// As you prepare to give the amphipods your solution, you notice that the
// diagram they handed you was actually folded up. As you unfold it, you
// discover an extra part of the diagram.

// Between the first and second lines of text that contain amphipod starting
// positions, insert the following lines:

//   #D#C#B#A#
//   #D#B#A#C#

// So, the above example now becomes:

// #############
// #...........#
// ###B#C#B#D###
//   #D#C#B#A#
//   #D#B#A#C#
//   #A#D#C#A#
//   #########

// The amphipods still want to be organized into rooms similar to before:

// #############
// #...........#
// ###A#B#C#D###
//   #A#B#C#D#
//   #A#B#C#D#
//   #A#B#C#D#
//   #########

// In this updated example, the least energy required to organize these
// amphipods is 44169

// Using the initial configuration from the full diagram, what is the least
// energy required to organize the amphipods?


use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum NodeType {
    Hallway,
    Room,
    OutsideRoom,
}

fn print_map(
    nodes: &HashMap<(isize, isize), NodeType>,
    amphipods: &BTreeMap<(isize, isize), Amphipod>,
) {
    let min_i = nodes.keys().map(|(i, _)| *i).min().unwrap();
    let max_i = nodes.keys().map(|(i, _)| *i).max().unwrap();
    let min_j = nodes.keys().map(|(_, j)| *j).min().unwrap();
    let max_j = nodes.keys().map(|(_, j)| *j).max().unwrap();

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            match (nodes.get(&(i, j)), amphipods.get(&(i, j))) {
                (Some(NodeType::Hallway), None) | (Some(NodeType::Room), None) => {
                    eprint!(".")
                }
                (Some(NodeType::OutsideRoom), None) => eprint!("x"),
                (Some(NodeType::Hallway), Some(a))
                | (Some(NodeType::Room), Some(a))
                | (Some(NodeType::OutsideRoom), Some(a)) => eprint!("{:?}", a),
                (None, None) => eprint!(" "),
                _ => unreachable!(),
            }
        }
        eprintln!();
    }
}

const fn cost(a: Amphipod) -> isize {
    match a {
        Amphipod::A => 1,
        Amphipod::B => 10,
        Amphipod::C => 100,
        Amphipod::D => 1000,
    }
}

const fn correct_room_col(a: Amphipod) -> isize {
    match a {
        Amphipod::A => 3,
        Amphipod::B => 5,
        Amphipod::C => 7,
        Amphipod::D => 9,
    }
}

fn in_correct_room(amphipod_loc: (isize, isize), a: Amphipod) -> bool {
    amphipod_loc.1 == correct_room_col(a) && amphipod_loc.0 >= 2
}

fn search(
    nodes: &HashMap<(isize, isize), NodeType>,
    edges: &HashMap<(isize, isize), Vec<(isize, isize)>>,
    amphipods: &BTreeMap<(isize, isize), Amphipod>,
    depth: usize,
    cache: &mut HashMap<BTreeMap<(isize, isize), Amphipod>, Option<isize>>,
) -> Option<isize> {
    if let Some(v) = cache.get(amphipods) {
        return *v;
    }

    let out_of_place = out_of_place_amphipods(amphipods);

    if out_of_place.is_empty() {
        return Some(0);
    }

    let in_hallway = out_of_place
        .iter()
        .copied()
        .filter(|pos| nodes[pos] == NodeType::Hallway)
        .collect::<Vec<_>>();
    let in_room = out_of_place
        .iter()
        .copied()
        .filter(|pos| nodes[pos] == NodeType::Room)
        .collect::<Vec<_>>();
    assert_eq!(in_hallway.len() + in_room.len(), out_of_place.len());

    let mut subcosts = vec![];

    for p in in_room {
        for (h, hc) in _move_amphipod_to_hallway(nodes, edges, amphipods, p) {
            let mut aa = amphipods.clone();
            let v = aa.remove(&p).unwrap();
            aa.insert(h, v);
            assert_eq!(amphipods.len(), aa.len());

            if let Some(c) = search(nodes, edges, &aa, depth + 1, cache) {
                subcosts.push(hc + c);
            }
        }
    }

    for p in in_hallway {
        for (h, hc) in _move_amphipod_to_room(nodes, edges, amphipods, p) {
            let mut aa = amphipods.clone();
            let v = aa.remove(&p).unwrap();
            aa.insert(h, v);
            assert_eq!(amphipods.len(), aa.len());

            if let Some(c) = search(nodes, edges, &aa, depth + 1, cache) {
                subcosts.push(hc + c);
            }
        }
    }

    let v = subcosts.iter().min().copied();
    cache.insert(amphipods.clone(), v);
    v
}

fn _move_amphipod_to_hallway(
    nodes: &HashMap<(isize, isize), NodeType>,
    edges: &HashMap<(isize, isize), Vec<(isize, isize)>>,
    amphipods: &BTreeMap<(isize, isize), Amphipod>,
    amphipod_loc: (isize, isize),
) -> Vec<((isize, isize), isize)> {
    assert_eq!(nodes[&amphipod_loc], NodeType::Room);

    let amphipod = amphipods[&amphipod_loc];
    if _no_bad_in_correct_room(amphipods, amphipod) && in_correct_room(amphipod_loc, amphipod) {
        return vec![];
    }

    // Attempt to move this amphipod to each reachable hallway position.
    let mut viable_outputs = vec![];
    let mut visited = HashSet::new();

    let mut stk = vec![(amphipod_loc, 0)];

    while let Some((pos, c)) = stk.pop() {
        visited.insert(pos);
        if nodes[&pos] == NodeType::Hallway {
            viable_outputs.push((pos, c));
        }
        for n in &edges[&pos] {
            if !visited.contains(n) && !amphipods.contains_key(n) {
                stk.push((*n, c + cost(amphipod)));
            }
        }
    }

    viable_outputs
}

fn _no_bad_in_correct_room(amphipods: &BTreeMap<(isize, isize), Amphipod>, a: Amphipod) -> bool {
    for i in [2, 3, 4, 5] {
        if let Some(aa) = amphipods.get(&(i, correct_room_col(a))) {
            if *aa != a {
                return false;
            }
        }
    }
    true
}

fn _move_amphipod_to_room(
    nodes: &HashMap<(isize, isize), NodeType>,
    edges: &HashMap<(isize, isize), Vec<(isize, isize)>>,
    amphipods: &BTreeMap<(isize, isize), Amphipod>,
    amphipod_loc: (isize, isize),
) -> Vec<((isize, isize), isize)> {
    let amphipod = amphipods[&amphipod_loc];
    if !_no_bad_in_correct_room(amphipods, amphipod) {
        return vec![];
    }

    // Attempt to move this amphipod to each viable room position
    let mut viable_outputs = vec![];
    let mut visited = HashSet::new();

    assert_eq!(nodes[&amphipod_loc], NodeType::Hallway);

    let mut stk = vec![(amphipod_loc, 0)];

    while let Some((pos, c)) = stk.pop() {
        visited.insert(pos);
        if nodes[&pos] == NodeType::Room {
            viable_outputs.push((pos, c));
        }
        for n in &edges[&pos] {
            if visited.contains(n) {
                continue;
            }
            if nodes[n] == NodeType::Room && !in_correct_room(*n, amphipod) {
                continue;
            } else if !amphipods.contains_key(n) {
                stk.push((*n, c + cost(amphipod)));
            }
        }
    }

    viable_outputs
}

fn out_of_place_amphipods(amphipods: &BTreeMap<(isize, isize), Amphipod>) -> Vec<(isize, isize)> {
    let mut out_of_place = vec![];
    for (pos, amphipod) in amphipods {
        if !in_correct_room(*pos, *amphipod) || !_no_bad_in_correct_room(amphipods, *amphipod) {
            out_of_place.push(*pos);
        }
    }
    out_of_place
}

pub fn part_1(s: &str) -> isize {
    let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    let mut nodes = HashMap::new();
    let mut amphipods = BTreeMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let i = i as isize;
            let j = j as isize;
            match c {
                '.' => {
                    nodes.insert((i, j), NodeType::Hallway);
                }
                'A' => {
                    nodes.insert((i, j), NodeType::Room);
                    amphipods.insert((i, j), Amphipod::A);
                }
                'B' => {
                    nodes.insert((i, j), NodeType::Room);
                    amphipods.insert((i, j), Amphipod::B);
                }
                'C' => {
                    nodes.insert((i, j), NodeType::Room);
                    amphipods.insert((i, j), Amphipod::C);
                }
                'D' => {
                    nodes.insert((i, j), NodeType::Room);
                    amphipods.insert((i, j), Amphipod::D);
                }
                ' ' | '#' => (),
                _ => unreachable!(),
            }
        }
    }

    let mut edges = HashMap::new();
    let keys = nodes.keys().copied().collect::<Vec<_>>();
    for (i, j) in keys {
        let mut e = vec![];
        let n = nodes[&(i, j)];
        for (ii, jj) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            if let Some(v) = nodes.get(&(ii, jj)) {
                if let (NodeType::Hallway, NodeType::Room) = (n, v) {
                    nodes.insert((i, j), NodeType::OutsideRoom);
                }
                e.push((ii, jj));
            }
        }
        edges.insert((i, j), e);
    }

    print_map(&nodes, &amphipods);

    // Explore every possible action

    let mut cache = HashMap::new();
    search(&nodes, &edges, &amphipods, 0, &mut cache).unwrap()
}

pub fn part_2(s: &str) -> isize {
    part_1(s)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_day_23_example_part_1() {
        assert_eq!(part_1(EXAMPLE4), 9015);
        assert_eq!(part_1(EXAMPLE), 12521);
    }

    #[test]
    fn test_day_23_part_1() {
        assert_eq!(part_1(include_str!("input/day_23.txt")), 10411);
    }

    #[test]
    fn test_day_23_example_part_2() {
        assert_eq!(part_2(EXAMPLE2), 44169);
    }

    #[test]
    fn test_day_23_part_2() {
        let answer = part_2(include_str!("input/day_23_2.txt"));
        assert_eq!(answer, 46721);
    }

    const EXAMPLE: &str = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;
    const EXAMPLE2: &str = r#"#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########"#;
    const EXAMPLE4: &str = r#"#############
#...........#
###A#B#C#A###
  #D#B#C#D#
  #########"#;
}
