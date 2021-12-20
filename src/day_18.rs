#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum V {
    Open,
    Delim,
    Literal(usize),
    Close,
}

fn parse_val(s: impl IntoIterator<Item = char>) -> Vec<V> {
    let mut v = vec![];
    let mut digits = None;
    for c in s {
        match c {
            '[' => {
                v.push(V::Open);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if digits.is_none() {
                    digits = Some(String::new());
                }
                digits.as_mut().unwrap().push(c);
            }
            ',' => {
                if let Some(p) = digits.take() {
                    v.push(V::Literal(p.parse().unwrap()));
                }
                v.push(V::Delim);
            }
            ']' => {
                if let Some(p) = digits.take() {
                    v.push(V::Literal(p.parse().unwrap()));
                }
                v.push(V::Close);
            }
            _ => unreachable!(),
        }
    }
    v
}

pub fn print_val(v: &[V]) {
    for vv in v {
        match vv {
            V::Open => eprint!("["),
            V::Close => eprint!("]"),
            V::Delim => eprint!(","),
            V::Literal(x) => eprint!("{}", x),
        }
    }
    eprintln!();
}

fn reduce(mut v: Vec<V>) -> (Vec<V>, bool) {
    let mut depth = 0;

    #[derive(Debug, Copy, Clone)]
    enum Action {
        Split { at: usize, val: usize },
        Explode { at: usize },
    }

    let mut action = None;

    for (idx, vv) in v.iter().enumerate() {
        match vv {
            V::Open => {
                depth += 1;
                if depth == 5 {
                    action = Some(Action::Explode { at: idx });
                    break;
                }
            }
            V::Close => depth -= 1,
            _ => (),
        }
    }
    if action.is_none() {
        for (idx, vv) in v.iter().enumerate() {
            if let V::Literal(x) = vv {
                if *x >= 10 {
                    action = Some(Action::Split { at: idx, val: *x });
                    break;
                }
            }
        }
    }

    match action {
        Some(Action::Split { at, val }) => {
            let lhs = val / 2;
            let rhs = (val + 1) / 2;
            v[at] = V::Open;
            v.insert(at + 1, V::Literal(lhs));
            v.insert(at + 2, V::Delim);
            v.insert(at + 3, V::Literal(rhs));
            v.insert(at + 4, V::Close);
            (v, true)
        }
        Some(Action::Explode { at }) => {
            let (lhs, rhs) = match (v[at], v[at + 1], v[at + 2], v[at + 3], v[at + 4]) {
                (V::Open, V::Literal(lhs), V::Delim, V::Literal(rhs), V::Close) => (lhs, rhs),
                _ => unreachable!(),
            };
            v.remove(at + 4);
            v.remove(at + 3);
            v.remove(at + 2);
            v.remove(at + 1);

            v[at] = V::Literal(0);

            for idx in (0..at).rev() {
                if let V::Literal(x) = v[idx] {
                    v[idx] = V::Literal(x + lhs);
                    break;
                }
            }

            for vv in v.iter_mut().skip(at + 1) {
                if let V::Literal(x) = vv {
                    *x += rhs;
                    break;
                }
            }
            (v, true)
        }
        None => (v, false),
    }
}

fn reduced(mut v: Vec<V>) -> Vec<V> {
    loop {
        let (v_, reduced) = reduce(v);
        // print_val(&v_);
        v = v_;
        if !reduced {
            break;
        }
    }
    v
}

fn magnitude(v: &[V]) -> (usize, usize) {
    match v[0] {
        V::Literal(x) => (x, 1),
        V::Open => {
            let (lhs, len) = magnitude(&v[1..]);
            let (rhs, len_) = magnitude(&v[(2 + len..)]);
            // lhs len + rhs len + [,]
            (lhs * 3 + rhs * 2, len_ + len + 3)
        }
        _ => unreachable!(),
    }
}

pub fn part_1(s: &str) -> usize {
    let mut iter = s.lines();
    let first = iter.next().unwrap();
    let mut v = reduced(parse_val(first.chars()));

    for next in iter {
        let v2 = reduced(parse_val(next.chars()));
        v.insert(0, V::Open);
        v.push(V::Delim);
        v.extend(v2);
        v.push(V::Close);
        v = reduced(v);
    }

    magnitude(&v).0
}

pub fn part_2(s: &str) -> usize {
    let mut max_magnitude = 0;

    for (idx, line) in s.lines().enumerate() {
        let v = reduced(parse_val(line.chars()));
        for (idx2, line2) in s.lines().enumerate() {
            if idx != idx2 {
                let v2 = reduced(parse_val(line2.chars()));
                let mut v3 = vec![V::Open];
                v3.extend(v.clone());
                v3.push(V::Delim);
                v3.extend(v2);
                v3.push(V::Close);
                v3 = reduced(v3);
                max_magnitude = max_magnitude.max(magnitude(&v3).0)
            }
        }
    }
    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const EXAMPLE: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    #[test]
    fn test_day_18_example_part_1() {
        assert_eq!(part_1(EXAMPLE), 4140);
    }

    #[test]
    fn test_day_18_part_1() {
        assert_eq!(part_1(include_str!("input/day_18.txt")), 2907);
    }

    #[test]
    fn test_day_18_example_part_2() {
        assert_eq!(part_2(EXAMPLE), 3993);
    }

    #[test]
    fn test_day_18_part_2() {
        let answer = part_2(include_str!("input/day_18.txt"));
        assert_eq!(answer, 4690);
    }
}
