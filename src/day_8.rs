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
