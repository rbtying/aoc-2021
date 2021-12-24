// --- Day 24: Arithmetic Logic Unit ---
// Magic smoke starts leaking from the submarine's arithmetic logic unit (ALU). Without the ability
// to perform basic arithmetic and logic functions, the submarine can't produce cool patterns with
// its Christmas lights!

// It also can't navigate. Or run the oxygen system.

// Don't worry, though - you probably have enough oxygen left to give you enough time to build a
// new ALU.

// The ALU is a four-dimensional processing unit: it has integer variables w, x, y, and z. These
// variables all start with the value 0. The ALU also supports six instructions:

// inp a - Read an input value and write it to variable a.
// add a b - Add the value of a to the value of b, then store the result in variable a.
// mul a b - Multiply the value of a by the value of b, then store the result in variable a.
// div a b - Divide the value of a by the value of b, truncate the result to an integer, then store
// the result in variable a. (Here, "truncate" means to round the value toward zero.)
// mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This
// is also called the modulo operation.)
// eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise,
// store the value 0 in variable a.
// In all of these instructions, a and b are placeholders; a will always be the variable where the
// result of the operation is stored (one of w, x, y, or z), while b can be either a variable or a
// number. Numbers can be positive or negative, but will always be integers.

// The ALU has no jump instructions; in an ALU program, every instruction is run exactly once in
// order from top to bottom. The program halts after the last instruction has finished executing.

// (Program authors should be especially cautious; attempting to execute div with b=0 or attempting
// to execute mod with a<0 or b<=0 will cause the program to crash and might even damage the ALU.
// These operations are never intended in any serious ALU program.)

// For example, here is an ALU program which takes an input number, negates it, and stores it in x:

// inp x
// mul x -1
// Here is an ALU program which takes two input numbers, then sets z to 1 if the second input
// number is three times larger than the first input number, or sets z to 0 otherwise:

// inp z
// inp x
// mul z 3
// eql z x
// Here is an ALU program which takes a non-negative integer as input, converts it into binary, and
// stores the lowest (1's) bit in z, the second-lowest (2's) bit in y, the third-lowest (4's) bit
// in x, and the fourth-lowest (8's) bit in w:

// inp w
// add z w
// mod z 2
// div w 2
// add y w
// mod y 2
// div w 2
// add x w
// mod x 2
// div w 2
// mod w 2
// Once you have built a replacement ALU, you can install it in the submarine, which will
// immediately resume what it was doing when the ALU failed: validating the submarine's model
// number. To do this, the ALU will run the MOdel Number Automatic Detector program (MONAD, your
// puzzle input).

// Submarine model numbers are always fourteen-digit numbers consisting only of digits 1 through 9.
// The digit 0 cannot appear in a model number.

// When MONAD checks a hypothetical fourteen-digit model number, it uses fourteen separate inp
// instructions, each expecting a single digit of the model number in order of most to least
// significant. (So, to check the model number 13579246899999, you would give 1 to the first inp
// instruction, 3 to the second inp instruction, 5 to the third inp instruction, and so on.) This
// means that when operating MONAD, each input instruction should only ever be given an integer
// value of at least 1 and at most 9.

// Then, after MONAD has finished running all of its instructions, it will indicate that the model
// number was valid by leaving a 0 in variable z. However, if the model number was invalid, it will
// leave some other non-zero value in z.

// MONAD imposes additional, mysterious restrictions on model numbers, and legend says the last
// copy of the MONAD documentation was eaten by a tanuki. You'll need to figure out what MONAD does
// some other way.

// To enable as many submarine features as possible, find the largest valid fourteen-digit model
// number that contains no 0 digits. What is the largest model number accepted by MONAD?

// --- Part Two ---
// As the submarine starts booting up things like the Retro Encabulator, you realize that maybe you
// don't need all these submarine features after all.

// What is the smallest model number accepted by MONAD?

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
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
    Z(usize),
    Add(Box<Pair>),
    Mul(Box<Pair>),
    Div(Box<Pair>),
    Mod(Box<Pair>),
    Eql(Box<Pair>),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Pair(Expr, Expr);

impl Expr {
    fn inner(&self) -> Option<&Pair> {
        match self {
            Expr::Input(_) | Expr::Immediate(_) | Expr::Z(_) => None,
            Expr::Add(p) | Expr::Mul(p) | Expr::Div(p) | Expr::Mod(p) | Expr::Eql(p) => Some(p),
        }
    }

    fn children(&self) -> Option<(&Expr, &Expr)> {
        self.inner().map(|p| (&p.0, &p.1))
    }

    fn inputs(&self) -> Vec<usize> {
        match self {
            Expr::Input(x) => vec![*x],
            Expr::Immediate(_) => vec![],
            Expr::Z(_) => vec![],
            Expr::Add(p) | Expr::Mul(p) | Expr::Div(p) | Expr::Mod(p) | Expr::Eql(p) => {
                let mut v = p.0.inputs();
                v.extend(p.1.inputs());
                v
            }
        }
    }

    fn eval(&self, inputs: &HashMap<usize, isize>) -> isize {
        match self {
            Expr::Input(x) => inputs[x],
            Expr::Immediate(x) => *x,
            Expr::Z(_) => unreachable!("Can't evaluate an expr which still has Z!"),
            Expr::Add(p) | Expr::Mul(p) | Expr::Div(p) | Expr::Mod(p) | Expr::Eql(p) => {
                let lhs = p.0.eval(inputs);
                let rhs = p.1.eval(inputs);

                match self {
                    Expr::Add(_) => lhs + rhs,
                    Expr::Mul(_) => lhs * rhs,
                    Expr::Mod(_) => lhs % rhs,
                    Expr::Div(_) => lhs / rhs,
                    Expr::Eql(_) => (lhs == rhs) as isize,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn max(&self) -> isize {
        match self {
            Expr::Input(_) => 9,
            Expr::Immediate(x) => *x,
            Expr::Add(p) => isize::saturating_add(p.0.max(), p.1.max()),
            // assume both sides are positive...
            Expr::Mul(p) => isize::saturating_mul(p.0.max(), p.1.max()),
            Expr::Div(p) => p.0.max(),
            Expr::Mod(p) => p.1.max(),
            Expr::Eql(_) => 1,
            Expr::Z(_) => isize::MAX,
        }
    }

    fn min(&self) -> isize {
        match self {
            Expr::Input(_) => 0,
            Expr::Immediate(x) => *x,
            Expr::Add(p) => isize::saturating_add(p.0.min(), p.1.min()),
            Expr::Mod(p) => {
                if p.0.min() >= 0 && p.1.min() >= 0 {
                    p.0.min() % p.1.min()
                } else {
                    isize::MIN
                }
            }
            Expr::Div(_) | Expr::Mul(_) => isize::MIN,
            Expr::Eql(_) => 1,
            Expr::Z(_) => 0,
        }
    }

    fn distribute_mods(&self) -> Expr {
        match self.clone() {
            Expr::Input(v) => Expr::Input(v),
            Expr::Z(x) => Expr::Z(x),
            Expr::Immediate(v) => Expr::Immediate(v),
            Expr::Add(p) | Expr::Mul(p) | Expr::Div(p) | Expr::Eql(p) => {
                let new_p = Box::new(Pair(p.0.distribute_mods(), p.1.distribute_mods()));

                match self {
                    Expr::Add(_) => Expr::Add(new_p),
                    Expr::Mul(_) => Expr::Mul(new_p),
                    Expr::Div(_) => Expr::Div(new_p),
                    Expr::Eql(_) => Expr::Eql(new_p),
                    _ => unreachable!(),
                }
            }
            Expr::Mod(p) => match p.0 {
                Expr::Add(ap) => Expr::Mod(Box::new(Pair(
                    Expr::Add(Box::new(Pair(
                        Expr::Mod(Box::new(Pair(ap.0.distribute_mods(), p.1.clone()))).simplify(),
                        Expr::Mod(Box::new(Pair(ap.1.distribute_mods(), p.1.clone()))).simplify(),
                    ))),
                    p.1,
                ))),
                _ => Expr::Mod(Box::new(Pair(p.0.distribute_mods(), p.1.distribute_mods()))),
            },
        }
        .simplify()
    }

    fn distribute_divs(&self) -> Expr {
        match self.clone() {
            Expr::Input(v) => Expr::Input(v),
            Expr::Z(x) => Expr::Z(x),
            Expr::Immediate(v) => Expr::Immediate(v),
            Expr::Add(p) | Expr::Mul(p) | Expr::Mod(p) | Expr::Eql(p) => {
                let new_p = Box::new(Pair(p.0.distribute_divs(), p.1.distribute_divs()));

                match self {
                    Expr::Add(_) => Expr::Add(new_p),
                    Expr::Mul(_) => Expr::Mul(new_p),
                    Expr::Eql(_) => Expr::Eql(new_p),
                    Expr::Mod(_) => Expr::Eql(new_p),
                    _ => unreachable!(),
                }
            }
            Expr::Div(p) => match p.0 {
                Expr::Add(ap) => Expr::Add(Box::new(Pair(
                    Expr::Div(Box::new(Pair(ap.0.distribute_divs(), p.1.clone()))).simplify(),
                    Expr::Div(Box::new(Pair(ap.1.distribute_divs(), p.1.clone()))).simplify(),
                ))),
                _ => Expr::Div(Box::new(Pair(p.0.distribute_divs(), p.1.distribute_divs()))),
            },
        }
        .simplify()
    }

    fn replace(&self, from: &Expr, to: Expr) -> Expr {
        if self == from {
            to
        } else if let Some(inner) = self.inner() {
            let new_inner = Box::new(Pair(
                inner.0.replace(from, to.clone()),
                inner.1.replace(from, to),
            ));

            match self {
                Expr::Add(_) => Expr::Add(new_inner),
                Expr::Mul(_) => Expr::Mul(new_inner),
                Expr::Div(_) => Expr::Div(new_inner),
                Expr::Mod(_) => Expr::Mod(new_inner),
                Expr::Eql(_) => Expr::Eql(new_inner),
                _ => unreachable!(),
            }
            .simplify()
        } else {
            self.clone()
        }
    }

    fn simplify(&self) -> Expr {
        match self.clone() {
            Expr::Input(v) => Expr::Input(v),
            Expr::Z(x) => Expr::Z(x),
            Expr::Immediate(v) => Expr::Immediate(v),
            Expr::Mul(p) => match *p {
                Pair(Expr::Immediate(0), _) | Pair(_, Expr::Immediate(0)) => Expr::Immediate(0),
                Pair(Expr::Immediate(1), o) | Pair(o, Expr::Immediate(1)) => o,
                Pair(Expr::Div(ap), b) if ap.1 == b => ap.0,
                Pair(Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a * b),
                _ => Expr::Mul(p),
            },
            Expr::Div(p) => match *p {
                Pair(Expr::Immediate(0), _) => Expr::Immediate(0),
                Pair(Expr::Mul(ap), b) if ap.1 == b => ap.0,
                Pair(Expr::Mul(ap), b) if ap.0 == b => ap.1,
                Pair(o, Expr::Immediate(1)) => o,
                Pair(Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a / b),
                Pair(a, Expr::Immediate(b)) if a.max() < b => Expr::Immediate(0),
                _ => Expr::Div(p),
            },
            Expr::Add(p) => match *p {
                Pair(Expr::Immediate(0), o) | Pair(o, Expr::Immediate(0)) => o,
                Pair(Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a + b),
                _ => Expr::Add(p),
            },
            Expr::Mod(p) => match *p {
                Pair(Expr::Immediate(0), _) => Expr::Immediate(0),
                Pair(a, Expr::Immediate(b)) if a.max() < b => a,
                Pair(Expr::Mul(ap), b) if ap.1 == b => Expr::Immediate(0),
                Pair(Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate(a % b),
                Pair(Expr::Mod(ap), Expr::Immediate(b)) if ap.1 == Expr::Immediate(b) => {
                    Expr::Mod(ap)
                }
                _ => Expr::Mod(p),
            },
            Expr::Eql(p) => match *p {
                Pair(Expr::Immediate(a), Expr::Immediate(b)) => Expr::Immediate((a == b) as isize),
                Pair(Expr::Input(_), e) | Pair(e, Expr::Input(_)) if e.min() > 9 => {
                    Expr::Immediate(0)
                }
                _ => Expr::Eql(p),
            },
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Input(v) => write!(f, "IN[{}]", v)?,
            Expr::Immediate(v) => write!(f, "{}", v)?,
            Expr::Add(p) => write!(f, "({:?} + {:?})", p.0, p.1)?,
            Expr::Mul(p) => write!(f, "({:?} * {:?})", p.0, p.1)?,
            Expr::Div(p) => write!(f, "({:?} / {:?})", p.0, p.1)?,
            Expr::Mod(p) => write!(f, "({:?} % {:?})", p.0, p.1)?,
            Expr::Eql(p) => match &**p {
                Pair(Expr::Eql(ap), Expr::Immediate(0)) => write!(f, "({:?} != {:?})", ap.0, ap.1)?,
                _ => write!(f, "({:?} == {:?})", p.0, p.1)?,
            },
            Expr::Z(v) => write!(f, "Z[{}]", v)?,
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
    z_history: Vec<Expr>,
}

impl Analysis {
    fn get(&self, register: Register) -> Expr {
        match register {
            Register::W => self.w.clone(),
            Register::X => self.x.clone(),
            Register::Y => self.y.clone(),
            Register::Z => self.z.clone(),
        }
    }

    fn get_z_value(&self, idx: usize) -> &Expr {
        &self.z_history[idx]
    }

    fn snapshot_z(&mut self) {
        let z = self.z.clone();
        self.z_history.push(z);
        self.z = Expr::Z(self.z_history.len() - 1);
    }

    fn set_z_value(&mut self, idx: usize, value: Expr) {
        self.z_history[idx] = value.simplify();
    }

    fn set(&mut self, register: Register, value: Expr) {
        match register {
            Register::W => self.w = value.simplify(),
            Register::X => self.x = value.simplify(),
            Register::Y => self.y = value.simplify(),
            Register::Z => self.z = value.simplify(),
        }
    }
}

pub fn parse_prog_2(program: &[Instruction]) -> Analysis {
    let mut analysis = Analysis {
        w: Expr::Immediate(0),
        x: Expr::Immediate(0),
        y: Expr::Immediate(0),
        z: Expr::Immediate(0),
        z_history: vec![],
    };

    let mut input = 0;
    for instruction in program {
        match *instruction {
            Instruction::Inp { a } => {
                analysis.snapshot_z();
                analysis.set(a, Expr::Input(input));
                input += 1;
            }
            Instruction::Add { a, b }
            | Instruction::Mul { a, b }
            | Instruction::Div { a, b }
            | Instruction::Mod { a, b }
            | Instruction::Eql { a, b } => {
                let p = Box::new(Pair(
                    analysis.get(a).clone(),
                    match b {
                        Arg::Immediate(v) => Expr::Immediate(v),
                        Arg::Register(r) => analysis.get(r).clone(),
                    },
                ));
                analysis.set(
                    a,
                    match *instruction {
                        Instruction::Add { .. } => Expr::Add(p),
                        Instruction::Mul { .. } => Expr::Mul(p),
                        Instruction::Div { .. } => Expr::Div(p),
                        Instruction::Mod { .. } => Expr::Mod(p),
                        Instruction::Eql { .. } => Expr::Eql(p),
                        _ => unreachable!(),
                    },
                );
            }
        }
    }

    analysis.snapshot_z();
    analysis
}

pub fn solve(
    mut analysis: Analysis,
    idx: usize,
    parent: Vec<(Expr, isize)>,
) -> Option<Vec<(Expr, isize)>> {
    if idx == 15 {
        // This is the end!
        return if *analysis.get_z_value(14) == Expr::Immediate(0) {
            Some(parent)
        } else {
            None
        };
    }

    eprint!("Z[{}]\t= ", idx);
    eprintln!("{:?}", analysis.get_z_value(idx));

    loop {
        let mut did_something = false;

        for j in (1..15).rev() {
            let z = analysis.get_z_value(idx);
            let from = Expr::Mod(Box::new(Pair(Expr::Z(j), Expr::Immediate(26))));
            let to = Expr::Mod(Box::new(Pair(
                analysis.get_z_value(j).clone(),
                Expr::Immediate(26),
            )))
            .distribute_mods();
            let n = z.replace(&from, to);
            if n != *z {
                did_something = true;
                println!("\t= {:?}", n);
                analysis.set_z_value(idx, n.clone());
            }
        }

        for j in (0..15).rev() {
            let z = analysis.get_z_value(idx);
            let from = Expr::Z(j);
            let to = analysis.get_z_value(j).clone();
            let n = z.replace(&from, to);
            if n != *z {
                did_something = true;
                eprintln!("\t= {:?}", n);
                analysis.set_z_value(idx, n.clone());
            }
        }

        loop {
            let z = analysis.get_z_value(idx);
            let n = z.distribute_divs();
            if n != *z {
                did_something = true;
                eprintln!("\t= {:?}", n);
                analysis.set_z_value(idx, n.clone());
            } else {
                break;
            }
        }

        if !did_something {
            break;
        }
    }

    let mut eq_exprs = HashSet::new();
    let mut stk = vec![analysis.get_z_value(idx).clone()];
    while let Some(v) = stk.pop() {
        if let Expr::Eql(_) = v {
            eq_exprs.insert(v.clone());
        } else if let Some((l, r)) = v.children() {
            stk.push(r.clone());
            stk.push(l.clone());
        }
    }

    if eq_exprs.is_empty() {
        // Just recurse to the right
        return solve(analysis, idx + 1, parent);
    }
    let eq_exprs = eq_exprs.into_iter().collect::<Vec<_>>();
    let mut routes = vec![];
    for s in 0..(1 << eq_exprs.len()) {
        let mut a = analysis.clone();
        let mut p = parent.clone();
        let mut z = analysis.get_z_value(idx).clone();

        for (t, eq_expr) in eq_exprs.iter().enumerate() {
            let v = s & (1 << t) != 0;
            if v {
                z = z.replace(eq_expr, Expr::Immediate(1));
                p.push((eq_expr.clone(), 1));
            } else {
                z = z.replace(eq_expr, Expr::Immediate(0));
                p.push((eq_expr.clone(), 0));
            }
        }
        a.set_z_value(idx, z);
        routes.push((a, p));
    }

    for (a, p) in routes {
        if let Some(x) = solve(a, idx + 1, p) {
            return Some(x);
        }
    }
    None
}

fn common(s: &str) -> (Vec<Instruction>, HashMap<usize, Vec<isize>>) {
    let prog = parse_prog(s);
    let analysis = parse_prog_2(&prog);
    let soln = solve(analysis, 0, vec![]).unwrap();
    let mut viable: HashMap<usize, Vec<isize>> = HashMap::new();
    for (eq, v) in soln {
        eprintln!("solving for {:?} == {:?}", eq, v);
        let inputs = eq.inputs();
        let mut m = HashMap::new();
        for i in &inputs {
            m.insert(*i, 1);
        }

        loop {
            if eq.eval(&m) == v {
                for (i, iv) in &m {
                    viable.entry(*i).or_default().push(*iv);
                }
            }

            let mut carry = false;
            for i in &inputs {
                carry = false;
                let iv = m.get_mut(i).unwrap();
                *iv += 1;
                if *iv == 10 {
                    *iv = 1;
                    carry = true;
                } else {
                    break;
                }
            }
            if carry {
                break;
            }
        }
    }
    eprintln!();
    eprintln!("viable mappings");
    for i in 0..14 {
        eprintln!("IN[{}]: {:?}", i, viable[&i]);
    }

    (prog, viable)
}

pub fn part_1(s: &str) -> isize {
    let (prog, viable) = common(s);

    let mut min_v = isize::MAX;

    let mut idxes = vec![0; 14];

    loop {
        let iter = (0..14).map(|i| viable[&i][idxes[i]]);
        let registers = eval(iter.clone(), &prog, Registers::default());

        if registers.z == 0 {
            let v = iter.reduce(|acc, v| acc * 10 + v).unwrap();
            min_v = min_v.min(v);
            break;
        }

        let mut carry = false;
        for i in 0..14 {
            carry = false;
            idxes[i] += 1;
            if idxes[i] >= viable[&i].len() {
                idxes[i] = 0;
                carry = true;
            } else {
                break;
            }
        }
        if carry {
            break;
        }
    }

    min_v
}

pub fn part_2(s: &str) -> isize {
    let (prog, viable) = common(s);

    let mut max_v = isize::MIN;

    let mut idxes = vec![0; 14];
    for i in 0..idxes.len() {
        idxes[i] = viable[&i].len() - 1;
    }

    loop {
        let iter = (0..14).map(|i| viable[&i][idxes[i]]);
        let registers = eval(iter.clone(), &prog, Registers::default());

        if registers.z == 0 {
            let v = iter.reduce(|acc, v| acc * 10 + v).unwrap();
            max_v = max_v.max(v);
            break;
        }

        let mut carry = false;
        for i in 0..14 {
            if idxes[i] == 0 {
                idxes[i] = viable[&i].len() - 1;
                carry = true;
            } else {
                carry = false;
                idxes[i] -= 1;
                break;
            }
        }
        if carry {
            break;
        }
    }

    max_v
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_day_24_part_1() {
        assert_eq!(part_1(include_str!("input/day_24.txt")), 11717131211195);
    }

    #[test]
    fn test_day_24_part_2() {
        let answer = part_2(include_str!("input/day_24.txt"));
        assert_eq!(answer, 51939397989999);
    }
}
