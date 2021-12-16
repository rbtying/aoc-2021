// --- Day 12: Passage Pathing ---

// With your submarine's subterranean subsystems subsisting suboptimally, the
// only way you're getting out of this cave anytime soon is by finding a path
// yourself. Not just a path - the only way to know if you've found the best
// path is to find all of them.

// Fortunately, the sensors are still mostly working, and so you build a rough
// map of the remaining caves (your puzzle input). For example:

// start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end

// This is a list of how all of the caves are connected. You start in the cave
// named start, and your destination is the cave named end. An entry like b-d
// means that cave b is connected to cave d - that is, you can move between
// them.

// So, the above cave system looks roughly like this:

//     start
//     /   \
// c--A-----b--d
//     \   /
//      end

// Your goal is to find the number of distinct paths that start at start, end at
// end, and don't visit small caves more than once. There are two types of
// caves: big caves (written in uppercase, like A) and small caves (written in
// lowercase, like b). It would be a waste of time to visit any small cave more
// than once, but big caves are large enough that it might be worth visiting
// them multiple times. So, all paths you find should visit small caves at most
// once, and can visit big caves any number of times.

// Given these rules, there are 10 paths through this example cave system:

// start,A,b,A,c,A,end
// start,A,b,A,end
// start,A,b,end
// start,A,c,A,b,A,end
// start,A,c,A,b,end
// start,A,c,A,end
// start,A,end
// start,b,A,c,A,end
// start,b,A,end
// start,b,end

// (Each line in the above list corresponds to a single path; the caves visited
// by that path are listed in the order they are visited and separated by
// commas.)

// Note that in this cave system, cave d is never visited by any path: to do so,
// cave b would need to be visited twice (once on the way to cave d and a second
// time when returning from cave d), and since cave b is small, this is not
// allowed.

// Here is a slightly larger example:

// dc-end
// HN-start
// start-kj
// dc-start
// dc-HN
// LN-dc
// HN-end
// kj-sa
// kj-HN
// kj-dc

// The 19 paths through it are as follows:

// start,HN,dc,HN,end
// start,HN,dc,HN,kj,HN,end
// start,HN,dc,end
// start,HN,dc,kj,HN,end
// start,HN,end
// start,HN,kj,HN,dc,HN,end
// start,HN,kj,HN,dc,end
// start,HN,kj,HN,end
// start,HN,kj,dc,HN,end
// start,HN,kj,dc,end
// start,dc,HN,end
// start,dc,HN,kj,HN,end
// start,dc,end
// start,dc,kj,HN,end
// start,kj,HN,dc,HN,end
// start,kj,HN,dc,end
// start,kj,HN,end
// start,kj,dc,HN,end
// start,kj,dc,end

// Finally, this even larger example has 226 paths through it:

// fs-end
// he-DX
// fs-he
// start-DX
// pj-DX
// end-zg
// zg-sl
// zg-pj
// pj-he
// RW-he
// fs-DX
// pj-RW
// zg-RW
// start-pj
// he-WI
// zg-he
// pj-fs
// start-RW

// How many paths through this cave system are there that visit small caves at
// most once?

// --- Part Two ---

// After reviewing the available paths, you realize you might have time to visit
// a single small cave twice. Specifically, big caves can be visited any number
// of times, a single small cave can be visited at most twice, and the remaining
// small caves can be visited at most once. However, the caves named start and
// end can only be visited exactly once each: once you leave the start cave, you
// may not return to it, and once you reach the end cave, the path must end
// immediately.

// Now, the 36 possible paths through the first example above are:

// start,A,b,A,b,A,c,A,end
// start,A,b,A,b,A,end
// start,A,b,A,b,end
// start,A,b,A,c,A,b,A,end
// start,A,b,A,c,A,b,end
// start,A,b,A,c,A,c,A,end
// start,A,b,A,c,A,end
// start,A,b,A,end
// start,A,b,d,b,A,c,A,end
// start,A,b,d,b,A,end
// start,A,b,d,b,end
// start,A,b,end
// start,A,c,A,b,A,b,A,end
// start,A,c,A,b,A,b,end
// start,A,c,A,b,A,c,A,end
// start,A,c,A,b,A,end
// start,A,c,A,b,d,b,A,end
// start,A,c,A,b,d,b,end
// start,A,c,A,b,end
// start,A,c,A,c,A,b,A,end
// start,A,c,A,c,A,b,end
// start,A,c,A,c,A,end
// start,A,c,A,end
// start,A,end
// start,b,A,b,A,c,A,end
// start,b,A,b,A,end
// start,b,A,b,end
// start,b,A,c,A,b,A,end
// start,b,A,c,A,b,end
// start,b,A,c,A,c,A,end
// start,b,A,c,A,end
// start,b,A,end
// start,b,d,b,A,c,A,end
// start,b,d,b,A,end
// start,b,d,b,end
// start,b,end

// The slightly larger example above now has 103 paths through it, and the even
// larger example now has 3509 paths through it.

// Given these new rules, how many paths through this cave system are there?

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn part_1(s: &str) -> usize {
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for line in s.lines() {
        let (from, to) = line.split_once("-").unwrap();
        edges
            .entry(from.to_owned())
            .or_default()
            .push(to.to_owned());
        edges
            .entry(to.to_owned())
            .or_default()
            .push(from.to_owned());
    }

    // Explore every path from start to end
    let mut stk = vec![vec!["start".to_owned()]];
    let mut num_paths = 0;
    while let Some(path) = stk.pop() {
        let last = path.last().unwrap();
        if last == "end" {
            num_paths += 1;
        }
        if let Some(edges_) = edges.get(last) {
            for to in edges_ {
                let mut path_ = path.clone();
                if !to.chars().next().unwrap().is_uppercase() && path_.contains(to) {
                    continue;
                }
                path_.push(to.to_owned());
                stk.push(path_);
            }
        }
    }

    num_paths
}

pub fn part_2(s: &str) -> usize {
    let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut nodes: HashMap<String, usize> = HashMap::new();
    let mut capitalized = vec![];
    for line in s.lines() {
        let (from, to) = line.split_once("-").unwrap();
        if !nodes.contains_key(from) {
            nodes.insert(from.to_string(), capitalized.len());
            capitalized.push(from.chars().next().unwrap().is_uppercase());
        }
        if !nodes.contains_key(to) {
            nodes.insert(to.to_string(), capitalized.len());
            capitalized.push(to.chars().next().unwrap().is_uppercase());
        }

        edges.entry(nodes[from]).or_default().push(nodes[to]);
        edges.entry(nodes[to]).or_default().push(nodes[from]);
    }

    #[derive(Clone)]
    struct Path {
        visited_twice: bool,
        visited: Rc<HashSet<usize>>,
        last: usize,
    }

    impl Path {
        pub fn new(s: usize) -> Self {
            Path {
                visited_twice: false,
                visited: Rc::new(vec![s].into_iter().collect()),
                last: s,
            }
        }

        pub fn extend(&self, s: usize, capital: bool) -> Self {
            let is_revisit = self.visited.contains(&s) && !capital;
            let visited_twice = self.visited_twice || is_revisit;

            if !capital && !is_revisit {
                let mut visited_: HashSet<_> = (*self.visited).clone();
                visited_.insert(s);
                Self {
                    visited: Rc::new(visited_),
                    last: s,
                    visited_twice,
                }
            } else {
                Self {
                    visited: Rc::clone(&self.visited),
                    last: s,
                    visited_twice,
                }
            }
        }
    }

    // Explore every path from start to end
    let end = nodes["end"];
    let start = nodes["start"];
    let mut stk = vec![Path::new(start)];
    let mut num_paths = 0;
    while let Some(path) = stk.pop() {
        if path.last == end {
            num_paths += 1;
        } else if let Some(edges_) = edges.get(&path.last) {
            for to in edges_ {
                if *to == start {
                    continue;
                }

                if !capitalized[*to] && path.visited_twice && path.visited.contains(to) {
                    continue;
                }

                stk.push(path.extend(*to, capitalized[*to]));
            }
        }
    }

    num_paths
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
    const EXAMPLE2: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
    const EXAMPLE3: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    #[test]
    fn test_day_12_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 10);
        assert_eq!(part_1(EXAMPLE2), 19);
    }

    #[test]
    fn test_day_12_part_1() {
        assert_eq!(part_1(include_str!("input/day_12.txt")), 3230);
    }

    #[test]
    fn test_day_12_example_part_2() {
        assert_eq!(part_2(EXAMPLE2), 103);
        assert_eq!(part_2(EXAMPLE3), 3509);
    }

    #[test]
    fn test_day_12_part_2() {
        assert_eq!(part_2(include_str!("input/day_12.txt")), 83475);
    }
}
