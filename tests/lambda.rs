extern crate abt;

use abt::binding::locally_nameless::*;
use abt::operator::*;
use abt::variable::*;
use abt::variable::nominal::*;
use std::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Op {
    Lam,
    Ap,
    Ax
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Op::Lam => { write!(f, "λ") }
            &Op::Ap => { write!(f, "ap") }
            &Op::Ax => { write!(f, "<>") }
        }
    }
}

impl Operator for Op {
    fn arity(&self) -> Vec<u64> {
        match self {
            &Op::Lam => { vec![1] }
            &Op::Ap => { vec![0, 0] }
            &Op::Ax => { vec![] }
        }
    }
}

#[test]
fn example() {
    let x: Nominal = Variable::new("x".to_string());
    println!("{}", app(Op::Lam, vec![abs(x.clone(), var(x))]));
}