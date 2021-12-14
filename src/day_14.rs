// --- Day 14: Extended Polymerization ---

// The incredible pressures at this depth are starting to put a strain on your
// submarine. The submarine has polymerization equipment that would produce
// suitable materials to reinforce the submarine, and the nearby
// volcanically-active caves should even have the necessary input elements in
// sufficient quantities.

// The submarine manual contains instructions for finding the optimal polymer
// formula; specifically, it offers a polymer template and a list of pair
// insertion rules (your puzzle input). You just need to work out what polymer
// would result after repeating the pair insertion process a few times.

// For example:

// NNCB

// CH -> B
// HH -> N
// CB -> H
// NH -> C
// HB -> C
// HC -> B
// HN -> C
// NN -> C
// BH -> H
// NC -> B
// NB -> B
// BN -> B
// BB -> N
// BC -> B
// CC -> N
// CN -> C

// The first line is the polymer template - this is the starting point of the
// process.

// The following section defines the pair insertion rules. A rule like AB -> C
// means that when elements A and B are immediately adjacent, element C should
// be inserted between them. These insertions all happen simultaneously.

// So, starting with the polymer template NNCB, the first step simultaneously
// considers all three pairs:

//     The first pair (NN) matches the rule NN -> C, so element C is inserted
//     between the first N and the second N.
//     The second pair (NC) matches the rule NC -> B, so element B is inserted
//     between the N and the C.
//     The third pair (CB) matches the rule CB -> H, so element H is inserted
//     between the C and the B.

// Note that these pairs overlap: the second element of one pair is the first
// element of the next pair. Also, because all pairs are considered
// simultaneously, inserted elements are not considered to be part of a pair
// until the next step.

// After the first step of this process, the polymer becomes NCNBCHB.

// Here are the results of a few steps using the above rules:

// Template:     NNCB
// After step 1: NCNBCHB
// After step 2: NBCCNBBBCBHCB
// After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
// After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB

// This polymer grows quickly. After step 5, it has length 97; After step 10, it
// has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H
// occurs 161 times, and N occurs 865 times; taking the quantity of the most
// common element (B, 1749) and subtracting the quantity of the least common
// element (H, 161) produces 1749 - 161 = 1588.

// Apply 10 steps of pair insertion to the polymer template and find the most
// and least common elements in the result. What do you get if you take the
// quantity of the most common element and subtract the quantity of the least
// common element?

// --- Part Two ---

// The resulting polymer isn't nearly strong enough to reinforce the submarine.
// You'll need to run more steps of the pair insertion process; a total of 40
// steps should do it.

// In the above example, the most common element is B (occurring 2192039569602
// times) and the least common element is H (occurring 3849876073 times);
// subtracting these produces 2188189693529.

// Apply 40 steps of pair insertion to the polymer template and find the most
// and least common elements in the result. What do you get if you take the
// quantity of the most common element and subtract the quantity of the least
// common element?

use std::collections::HashMap;

pub fn part_1(s: &str) -> usize {
    let mut iter = s.lines();
    let mut template = iter.next().unwrap().chars().collect::<Vec<_>>();
    iter.next().unwrap();

    let replacements = iter
        .map(|s| {
            let (a, b) = s.split_once("->").unwrap();
            let mut a_ = a.trim().chars();
            let aa = a_.next().unwrap();
            let ab = a_.next().unwrap();
            let b = b.trim().chars().next().unwrap();
            (aa, ab, b)
        })
        .collect::<Vec<_>>();

    for _ in 0..10 {
        let mut rewritten = vec![];
        let mut templ_iter = template.iter();
        let mut templ_iter_2 = template.iter();
        rewritten.push(*templ_iter_2.next().unwrap());

        while let (Some(aa_), Some(ab_)) = (templ_iter.next(), templ_iter_2.next()) {
            for (aa, ab, c) in &replacements {
                if aa_ == aa && ab_ == ab {
                    rewritten.push(*c);
                }
            }
            rewritten.push(*ab_);
        }
        template = rewritten;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in template {
        *counts.entry(c).or_default() += 1;
    }

    let (_, min) = counts.iter().min_by_key(|(_, v)| *v).unwrap();
    let (_, max) = counts.iter().max_by_key(|(_, v)| *v).unwrap();

    *max - *min
}

pub fn part_2(s: &str) -> usize {
    let mut iter = s.lines();
    let template = iter.next().unwrap().chars().collect::<Vec<_>>();
    iter.next().unwrap();

    let replacements = iter.map(|s| {
        let (a, b) = s.split_once("->").unwrap();
        let mut a_ = a.trim().chars();
        let aa = a_.next().unwrap();
        let ab = a_.next().unwrap();
        let b = b.trim().chars().next().unwrap();
        (aa, ab, b)
    });
    let mut indexed_replacements: HashMap<(char, char), char> = HashMap::new();
    for (a, b, c) in replacements {
        indexed_replacements.insert((a, b), c);
    }

    let mut char_counts: HashMap<char, usize> = HashMap::new();
    let mut pair_char_counts: HashMap<(char, char), usize> = HashMap::new();

    let mut iter = template.iter().copied();
    let mut iter_ = template.iter().skip(1).copied();

    for c in &template {
        *char_counts.entry(*c).or_default() += 1;
    }

    while let (Some(a), Some(b)) = (iter.next(), iter_.next()) {
        *pair_char_counts.entry((a, b)).or_default() += 1;
    }

    for _ in 0..40 {
        let mut pair_char_counts_ = HashMap::new();
        for ((a, b), ct) in pair_char_counts {
            if let Some(c) = indexed_replacements.get(&(a, b)) {
                *char_counts.entry(*c).or_default() += ct;
                *pair_char_counts_.entry((a, *c)).or_default() += ct;
                *pair_char_counts_.entry((*c, b)).or_default() += ct;
            } else {
                *pair_char_counts_.entry((a, b)).or_default() += ct;
            }
        }
        pair_char_counts = pair_char_counts_;
    }

    let min = *char_counts.values().min().unwrap();
    let max = *char_counts.values().max().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn test_day_14_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 1588);
    }

    #[test]
    fn test_day_14_part_1() {
        assert_eq!(part_1(include_str!("input/day_14.txt")), 2194);
    }

    #[test]
    fn test_day_14_example_part_2() {
        let answer = part_2(EXAMPLE);
        assert_eq!(answer, 2188189693529);
    }

    #[test]
    fn test_day_14_part_2() {
        let answer = part_2(include_str!("input/day_14.txt"));
        assert_eq!(answer, 2360298895777);
    }
}
