#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Registers {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl Registers {
    fn get(&self, register: Register) -> isize {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn set(&mut self, register: Register, value: isize) {
        match register {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn new(s: &str) -> Register {
        match s {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arg {
    Register(Register),
    Immediate(isize),
}

impl Arg {
    fn new(s: &str) -> Arg {
        match s {
            "w" | "x" | "y" | "z" => Arg::Register(Register::new(s)),
            s => Arg::Immediate(s.parse().unwrap()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Inp { a: Register },
    Add { a: Register, b: Arg },
    Mul { a: Register, b: Arg },
    Div { a: Register, b: Arg },
    Mod { a: Register, b: Arg },
    Eql { a: Register, b: Arg },
}

pub fn eval(
    mut input: impl Iterator<Item = isize>,
    program: &[Instruction],
    initial: Registers,
) -> Registers {
    let mut r = initial;

    for instruction in program {
        match *instruction {
            Instruction::Inp { a } => {
                let v = input.next().unwrap();
                r.set(a, v);
            }
            Instruction::Add { a, b } => {
                let v = r.get(a)
                    + match b {
                        Arg::Immediate(v) => v,
                        Arg::Register(b) => r.get(b),
                    };
                r.set(a, v);
            }
            Instruction::Mul { a, b } => {
                let v = r.get(a)
                    * match b {
                        Arg::Immediate(v) => v,
                        Arg::Register(b) => r.get(b),
                    };
                r.set(a, v);
            }
            Instruction::Div { a, b } => {
                let v = r.get(a)
                    / match b {
                        Arg::Immediate(v) => v,
                        Arg::Register(b) => r.get(b),
                    };
                r.set(a, v);
            }
            Instruction::Mod { a, b } => {
                let v = r.get(a)
                    % match b {
                        Arg::Immediate(v) => v,
                        Arg::Register(b) => r.get(b),
                    };
                r.set(a, v);
            }
            Instruction::Eql { a, b } => {
                let v = r.get(a)
                    == match b {
                        Arg::Immediate(v) => v,
                        Arg::Register(b) => r.get(b),
                    };
                r.set(a, v as isize);
            }
        }
    }

    r
}

pub fn parse_prog(s: &str) -> Vec<Instruction> {
    let mut prog = vec![];
    for line in s.lines() {
        let (cmd, args) = line.split_once(' ').unwrap();
        prog.push(match cmd {
            "inp" => Instruction::Inp {
                a: Register::new(args),
            },
            "add" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instruction::Add {
                    a: Register::new(a),
                    b: Arg::new(b),
                }
            }
            "mul" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instruction::Mul {
                    a: Register::new(a),
                    b: Arg::new(b),
                }
            }
            "div" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instruction::Div {
                    a: Register::new(a),
                    b: Arg::new(b),
                }
            }
            "mod" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instruction::Mod {
                    a: Register::new(a),
                    b: Arg::new(b),
                }
            }
            "eql" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instruction::Eql {
                    a: Register::new(a),
                    b: Arg::new(b),
                }
            }
            _ => unreachable!(),
        });
    }
    prog
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Expr {
    Input(usize),
    Immediate(isize),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Eql(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn max(&self) -> Option<isize> {
        match self {
            Expr::Input(_) => Some(9),
            Expr::Immediate(x) => Some(*x),
            Expr::Add(a, b) => match (a.max(), b.max()) {
                (Some(a), Some(b)) => Some(a + b),
                _ => None,
            },
            Expr::Mul(_, _) => None,
            Expr::Div(a, _) => a.max(),
            Expr::Mod(_, b) => b.max(),
            Expr::Eql(_, _) => Some(1),
        }
    }

    fn min(&self) -> Option<isize> {
        match self {
            Expr::Input(_) => Some(0),
            Expr::Immediate(x) => Some(*x),
            Expr::Add(a, b) => match (a.min(), b.min()) {
                (Some(a), Some(b)) => Some(a + b),
                _ => None,
            },
            Expr::Div(_, _) | Expr::Mul(_, _) | Expr::Mod(_, _) => None,
            Expr::Eql(_, _) => Some(1),
        }
    }
}

use std::fmt;
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Input(v) => write!(f, "IN[{}]", v)?,
            Expr::Immediate(v) => write!(f, "{}", v)?,
            Expr::Add(a, b) => write!(f, "({:?} + {:?})", a, b)?,
            Expr::Mul(a, b) => write!(f, "({:?} * {:?})", a, b)?,
            Expr::Div(a, b) => write!(f, "({:?} / {:?})", a, b)?,
            Expr::Mod(a, b) => write!(f, "({:?} % {:?})", a, b)?,
            Expr::Eql(a, b) => write!(f, "({:?} == {:?})", a, b)?,
        }
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Analysis {
    w: Expr,
    x: Expr,
    y: Expr,
    z: Expr,
}

impl Analysis {
    fn get(&self, register: Register) -> &Expr {
        match register {
            Register::W => &self.w,
            Register::X => &self.x,
            Register::Y => &self.y,
            Register::Z => &self.z,
        }
    }

    fn set(&mut self, register: Register, value: Expr) {
        let simplified_value = match value.clone() {
            Expr::Input(v) => Expr::Input(v),
            Expr::Immediate(v) => Expr::Immediate(v),
            Expr::Mul(a, b) => match (*a, *b) {
                (Expr::Immediate(0), _) | (_, Expr::Immediate(0)) => Expr::Immediate(0),
                (Expr::Immediate(1), o) | (o, Expr::Immediate(1)) => o,
                (Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a * b),
                (a, b) => Expr::Mul(Box::new(a), Box::new(b)),
            },
            Expr::Div(a, b) => match (*a, *b) {
                (e, Expr::Immediate(b)) if e.max().unwrap_or(isize::MAX) <= b => Expr::Immediate(0),
                (Expr::Immediate(0), _) | (_, Expr::Immediate(0)) => Expr::Immediate(0),
                (Expr::Immediate(1), o) | (o, Expr::Immediate(1)) => o,
                (Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a / b),
                (a, b) => Expr::Div(Box::new(a), Box::new(b)),
            },
            Expr::Add(a, b) => match (*a, *b) {
                (Expr::Immediate(0), o) | (o, Expr::Immediate(0)) => o,
                (Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a + b),
                (a, b) => Expr::Add(Box::new(a), Box::new(b)),
            },
            Expr::Mod(a, b) => match (*a, *b) {
                (Expr::Immediate(0), _) => Expr::Immediate(0),
                (Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a % b),
                (a, Expr::Immediate(b)) if a.max().unwrap_or(isize::MAX) < b => a,
                (a, b) => Expr::Mod(Box::new(a), Box::new(b)),
            },
            Expr::Eql(a, b) => match (*a, *b) {
                (Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate((a == b) as isize),
                (Expr::Input(_), e) | (e, Expr::Input(_)) if e.min().unwrap_or(isize::MIN) > 9 => {
                    Expr::Immediate(0)
                }
                (a, b) => Expr::Eql(Box::new(a), Box::new(b)),
            },
        };

        match register {
            Register::W => self.w = simplified_value,
            Register::X => self.x = simplified_value,
            Register::Y => self.y = simplified_value,
            Register::Z => {
                self.z = simplified_value;
            }
        }
    }
}

pub fn parse_prog_2(program: &[Instruction]) -> Analysis {
    let mut analysis = Analysis {
        w: Expr::Immediate(0),
        x: Expr::Immediate(0),
        y: Expr::Immediate(0),
        z: Expr::Immediate(0),
    };

    let mut input = 0;
    for instruction in program {
        match *instruction {
            Instruction::Inp { a } => {
                analysis.set(a, Expr::Input(input));
                eprintln!("{:?}", analysis.get(Register::Z));
                input += 1;
            }
            Instruction::Add { a, b } => {
                let a_ = Box::new(analysis.get(a).clone());
                let b_ = Box::new(match b {
                    Arg::Immediate(v) => Expr::Immediate(v),
                    Arg::Register(r) => analysis.get(r).clone(),
                });
                analysis.set(a, Expr::Add(a_, b_));
            }
            Instruction::Mul { a, b } => {
                let a_ = Box::new(analysis.get(a).clone());
                let b_ = Box::new(match b {
                    Arg::Immediate(v) => Expr::Immediate(v),
                    Arg::Register(r) => analysis.get(r).clone(),
                });
                analysis.set(a, Expr::Mul(a_, b_));
            }
            Instruction::Div { a, b } => {
                let a_ = Box::new(analysis.get(a).clone());
                let b_ = Box::new(match b {
                    Arg::Immediate(v) => Expr::Immediate(v),
                    Arg::Register(r) => analysis.get(r).clone(),
                });
                analysis.set(a, Expr::Div(a_, b_));
            }
            Instruction::Mod { a, b } => {
                let a_ = Box::new(analysis.get(a).clone());
                let b_ = Box::new(match b {
                    Arg::Immediate(v) => Expr::Immediate(v),
                    Arg::Register(r) => analysis.get(r).clone(),
                });
                analysis.set(a, Expr::Mod(a_, b_));
            }
            Instruction::Eql { a, b } => {
                let a_ = Box::new(analysis.get(a).clone());
                let b_ = Box::new(match b {
                    Arg::Immediate(v) => Expr::Immediate(v),
                    Arg::Register(r) => analysis.get(r).clone(),
                });
                analysis.set(a, Expr::Eql(a_, b_));
            }
        }
    }

    analysis
}

pub fn part_1(s: &str) -> isize {
    let prog = parse_prog(s);
    parse_prog_2(&prog);

    let mut numbers = [8isize; 14];
    let mut max_number = 0;

    loop {
        let mut as_isize = 0;
        for v in numbers {
            as_isize = as_isize * 10 + v;
        }
        let registers = eval(
            IntoIterator::into_iter(numbers),
            &prog,
            Registers {
                w: 0,
                x: 0,
                y: 0,
                z: 0,
            },
        );
        if registers.z == 0 {
            max_number = max_number.max(as_isize);
        }

        // increment numbers
        let mut idx_to_increment = 13;
        let done = loop {
            numbers[idx_to_increment] += 1;
            if numbers[idx_to_increment] == 10 {
                numbers[idx_to_increment] = 1;
                if idx_to_increment > 0 {
                    idx_to_increment -= 1;
                } else {
                    break true;
                }
            } else {
                break false;
            }
        };
        if done {
            break;
        }
    }

    // z = (numbers[0] % 2) == =
    // 89999999999999

    max_number
}

pub fn part_2(s: &str) -> isize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_day_24_example_part_1() {
        assert_eq!(part_1(EXAMPLE_PROG), 12521);
    }

    #[test]
    fn test_day_24_part_1() {
        assert_eq!(part_1(include_str!("input/day_24.txt")), 10411);
    }

    #[test]
    fn test_day_24_example_part_2() {
        assert_eq!(part_2(EXAMPLE_PROG), 44169);
    }

    #[test]
    fn test_day_24_part_2() {
        let answer = part_2(include_str!("input/day_24.txt"));
        assert_eq!(answer, 46721);
    }

    const EXAMPLE_PROG: &str = r#"inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"#;
}
