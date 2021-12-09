// --- Day 8: Seven Segment Search ---

// You barely reach the safety of the cave when the whale smashes into the cave
// mouth, collapsing it. Sensors indicate another exit to this cave at a much
// greater depth, so you have no choice but to press on.

// As your submarine slowly makes its way through the cave system, you notice
// that the four-digit seven-segment displays in your submarine are
// malfunctioning; they must have been damaged during the escape. You'll be in a
// lot of trouble without them, so you'd better figure out what's wrong.

// Each digit of a seven-segment display is rendered by turning on or off any of
// seven segments named a through g:

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// So, to render a 1, only segments c and f would be turned on; the rest would
// be off. To render a 7, only segments a, c, and f would be turned on.

// The problem is that the signals which control the segments have been mixed up
// on each display. The submarine is still trying to display numbers by
// producing output on signal wires a through g, but those wires are connected
// to segments randomly. Worse, the wire/segment connections are mixed up
// separately for each four-digit display! (All of the digits within a display
// use the same connections, though.)

// So, you might know that only signal wires b and g are turned on, but that
// doesn't mean segments b and g are turned on: the only digit that uses two
// segments is 1, so it must mean segments c and f are meant to be on. With just
// that information, you still can't tell which wire (b/g) goes to which segment
// (c/f). For that, you'll need to collect more information.

// For each display, you watch the changing signals for a while, make a note of
// all ten unique signal patterns you see, and then write down a single four
// digit output value (your puzzle input). Using the signal patterns, you should
// be able to work out which pattern corresponds to which digit.

// For example, here is what you might see in a single entry in your notes:

// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
// cdfeb fcadb cdfeb cdbaf

// (The entry is wrapped here to two lines so it fits; in your notes, it will
// all be on a single line.)

// Each entry consists of ten unique signal patterns, a | delimiter, and finally
// the four digit output value. Within an entry, the same wire/segment
// connections are used (but you don't know what the connections actually are).
// The unique signal patterns correspond to the ten different ways the submarine
// tries to render a digit using the current wire/segment connections. Because 7
// is the only digit that uses three segments, dab in the above example means
// that to render a 7, signal lines d, a, and b are on. Because 4 is the only
// digit that uses four segments, eafb means that to render a 4, signal lines e,
// a, f, and b are on.

// Using this information, you should be able to work out which combination of
// signal wires corresponds to each of the ten digits. Then, you can decode the
// four digit output value. Unfortunately, in the above example, all of the
// digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and
// are more difficult to deduce.

// For now, focus on the easy digits. Consider this larger example:

// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
// fdgacbe cefdb cefbgd gcbe
// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
// fcgedb cgb dgebacf gc
// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
// cg cg fdcagb cbg
// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
// efabcd cedba gadfec cb
// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
// gecf egdcabf bgf bfgea
// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
// gebdcfa ecba ca fadegcb
// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
// cefg dcbef fcge gbcadfe
// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
// ed bcgafe cdgba cbgef
// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
// gbdfcae bgc cg cgb
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
// fgae cfgab fg bagce

// Because the digits 1, 4, 7, and 8 each use a unique number of segments, you
// should be able to tell which combinations of signals correspond to those
// digits. Counting only digits in the output values (the part after | on each
// line), in the above example, there are 26 instances of digits that use a
// unique number of segments (highlighted above).

// In the output values, how many times do digits 1, 4, 7, or 8 appear?

// Through a little deduction, you should now be able to determine the remaining
// digits. Consider again the first example above:

// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
// cdfeb fcadb cdfeb cdbaf

// After some careful analysis, the mapping between signal wires and segments
// only make sense in the following configuration:

//  dddd
// e    a
// e    a
//  ffff
// g    b
// g    b
//  cccc

// So, the unique signal patterns would correspond to the following digits:

//     acedgfb: 8
//     cdfbe: 5
//     gcdfa: 2
//     fbcad: 3
//     dab: 7
//     cefabd: 9
//     cdfgeb: 6
//     eafb: 4
//     cagedb: 0
//     ab: 1

// Then, the four digits of the output value can be decoded:

//     cdfeb: 5
//     fcadb: 3
//     cdfeb: 5
//     cdbaf: 3

// Therefore, the output value for this entry is 5353.

// Following this same process for each entry in the second, larger example
// above, the output value of each entry can be determined:

//     fdgacbe cefdb cefbgd gcbe: 8394
//     fcgedb cgb dgebacf gc: 9781
//     cg cg fdcagb cbg: 1197
//     efabcd cedba gadfec cb: 9361
//     gecf egdcabf bgf bfgea: 4873
//     gebdcfa ecba ca fadegcb: 8418
//     cefg dcbef fcge gbcadfe: 4548
//     ed bcgafe cdgba cbgef: 1625
//     gbdfcae bgc cg cgb: 8717
//     fgae cfgab fg bagce: 4315

// Adding all of the output values in this larger example produces 61229.

// For each entry, determine all of the wire/segment connections and decode the
// four-digit output values. What do you get if you add up all of the output
// values?

use std::collections::HashMap;

pub fn part_1(s: &str) -> usize {
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for line in s.lines() {
        let mut found_delim = false;
        for segment in line.split_whitespace() {
            if found_delim {
                *freq.entry(segment.len()).or_default() += 1;
            }
            found_delim |= segment == "|";
        }
    }

    freq.get(&2).copied().unwrap_or_default()
        + freq.get(&4).copied().unwrap_or_default()
        + freq.get(&3).copied().unwrap_or_default()
        + freq.get(&7).copied().unwrap_or_default()
}

fn shared_chars(s1: &str, s2: &str) -> usize {
    s1.chars().filter(|c| s2.contains(*c)).count()
}

pub fn part_2(s: &str) -> usize {
    use std::iter::FromIterator;

    let mut total_sum = 0;

    for line in s.lines() {
        let mut decoder: HashMap<String, usize> = HashMap::new();
        // Pre-normalize the segments
        let signals = line
            .split_whitespace()
            .map(|s| {
                let mut chars: Vec<_> = s.chars().collect();
                chars.sort();
                String::from_iter(chars)
            })
            .collect::<Vec<_>>();
        let mut found_delim = false;
        for segment in &signals {
            if segment == "|" {
                break;
            }
            // Initialize the mappings
            if segment.len() == 2 {
                decoder.insert(segment.to_owned(), 1);
            } else if segment.len() == 3 {
                decoder.insert(segment.to_owned(), 7);
            } else if segment.len() == 4 {
                decoder.insert(segment.to_owned(), 4);
            } else if segment.len() == 7 {
                decoder.insert(segment.to_owned(), 8);
            } else {
                // 2, 3 and 5 all have 5 segments
                // 0, 6, and 9 all have 6 segments
            }
        }

        let mut s = 0;
        for segment in &signals {
            if segment == "|" {
                found_delim = true;
            } else if found_delim {
                // decoder
                s = decoder.get(segment).unwrap() + s * 10;
            } else {
                let one = decoder
                    .iter()
                    .filter(|(_, v)| **v == 1)
                    .map(|(k, _)| k)
                    .next()
                    .unwrap();
                let four = decoder
                    .iter()
                    .filter(|(_, v)| **v == 4)
                    .map(|(k, _)| k)
                    .next()
                    .unwrap();
                // Find mappings
                if decoder.contains_key(segment) {
                    // continue
                } else if segment.len() == 5 {
                    // it's either 2, 3, or 5.
                    // 2 and 5 share exactly one segment with 1, while 3 shares 2 segments
                    match shared_chars(segment, one) {
                        1 => {
                            // 2 shares 2 segments with 4, 5 shares 3
                            match shared_chars(segment, four) {
                                2 => {
                                    decoder.insert(segment.to_owned(), 2);
                                }
                                3 => {
                                    decoder.insert(segment.to_owned(), 5);
                                }
                                x => unreachable!("{:?} unexpected {:?} {:?}", x, segment, four),
                            }
                        }
                        2 => {
                            decoder.insert(segment.to_owned(), 3);
                        }
                        x => unreachable!("{:?} unexpected {:?} {:?}", x, segment, one),
                    }
                } else if segment.len() == 6 {
                    // it's either 0, 6, or 9.
                    // 0 and 9 share exactly two segments with 1, while 6 shares 1 segment
                    match shared_chars(segment, one) {
                        1 => {
                            decoder.insert(segment.to_owned(), 6);
                        }
                        2 => {
                            // 0 shares 3 segments with 4, 9 shares 4
                            match shared_chars(segment, four) {
                                3 => {
                                    decoder.insert(segment.to_owned(), 0);
                                }
                                4 => {
                                    decoder.insert(segment.to_owned(), 9);
                                }
                                x => unreachable!("{:?} unexpected {:?} {:?}", x, segment, four),
                            }
                        }
                        x => unreachable!("{:?} unexpected {:?} {:?}", x, segment, one),
                    }
                } else {
                    unreachable!("shouldn't get here {:?} {:?}", segment, decoder);
                }
            }
        }
        total_sum += s;
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_day_8_example_part_1() {
        assert_eq!(part_1(INPUT), 26);
    }

    #[test]
    fn test_day_8_part_1() {
        assert_eq!(part_1(include_str!("input/day_8.txt")), 493);
    }

    #[test]
    fn test_day_8_example_part_2() {
        assert_eq!(part_2(INPUT), 61229);
    }

    #[test]
    fn test_day_8_part_2() {
        assert_eq!(part_2(include_str!("input/day_8.txt")), 1010460);
    }
}
