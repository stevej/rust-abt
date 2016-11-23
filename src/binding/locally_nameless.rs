use operator::{Operator};
use std::fmt;
use variable::{Variable};
use view::{Unroll, Roll, View};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Abt<V, O> where V: Variable, O: Operator {
    Free(V),
    Bound(u64),
    Abs(V, Box<Abt<V, O>>),
    App(O, Vec<Abt<V, O>>),
}

impl<V, O> Abt<V, O> where
    V: Variable,
    O: Operator,
{
    pub fn match_arity(&self, n: u64) -> bool {
        match self {
            &Abt::Abs(_, _) if n == 0 => { false }
            &Abt::Abs(_, ref e) => { e.match_arity(n - 1) }
            _ if n == 0 => { true }
            _ => { false }
        }
    }

    pub fn shift_var(self, v: V, n: u64) -> Self {
        match self {
            Abt::Free(fv) => {
                if v == fv {
                    Abt::Bound(n)
                } else {
                    Abt::Free(fv)
                }
            }
            Abt::Bound(bv) => {
                Abt::Bound(bv)
            }
            Abt::Abs(x, e) => {
                Abt::Abs(x, Box::new(e.shift_var(v, n + 1)))
            }
            Abt::App(o, es) => {
                let res = es.iter().map(|e| e.clone().shift_var(v.clone(), n));
                Abt::App(o, res.collect::<_>())
            }
        }
    }

    pub fn add_var(self, v: V, n: u64) -> Self {
        match self {
            Abt::Free(fv) => {
                Abt::Free(fv)
            }
            Abt::Bound(m)  => {
                if m == n {
                    Abt::Free(v)
                } else {
                    Abt::Bound(m)
                }
            }
            Abt::Abs(x, e) => {
                Abt::Abs(x, Box::new(e.add_var(v, n + 1)))
            }
            Abt::App(o, es) => {
                let res = es.iter().map(|e| e.clone().add_var(v.clone(), n));
                Abt::App(o, res.collect::<_>())
            }
        }
    }
}

impl<V, O> Roll<V, O, Abt<V, O>> for View<V, O, Abt<V, O>> where
    V: Variable,
    O: Operator,
{
    fn roll(self) -> Abt<V, O> {
        match self {
            View::Var(v) => {
                Abt::Free(v)
            }
            View::Abs(v, e) => {
                Abt::Abs(v.clone(), Box::new(e.shift_var(v, 0)))
            }
            View::App(o, es) => {
                let ar = o.arity();
                if ar.len() != es.len() {
                    panic!()
                } else {
                    let mut b = true;
                    for (x, y) in es.iter().zip(ar.iter()) {
                        b = b && x.match_arity(*y);
                    }
                    if b {
                        Abt::App(o, es)
                    } else {
                        panic!()
                    }
                }
            }
        }
    }
}

impl<V, O> Unroll<V, O> for Abt<V, O> where
    V: Variable,
    O: Operator,
    View<V, O, Abt<V, O>>: Roll<V, O, Abt<V, O>>,
{
    fn unroll(self) -> View<V, O, Abt<V, O>> {
        match self {
            Abt::Free(v) => {
                View::Var(v)
            }
            Abt::Bound(_) => {
                panic!()
            }
            Abt::Abs(x, e) => {
                let v = x.dup();
                View::Abs(v.clone(), e.add_var(v, 0))
            }
            Abt::App(o, es) => {
                View::App(o, es)
            }
        }
    }
}

pub fn abs<V, O>(x: V, e: Abt<V, O>) -> Abt<V, O> where
    V: Variable,
    O: Operator,
{
    View::Abs(x, e).roll()
}

pub fn app<V, O>(o: O, es: Vec<Abt<V, O>>) -> Abt<V, O> where
    V: Variable,
    O: Operator,
{
    View::App(o, es).roll()
}

pub fn var<V, O>(x: V) -> Abt<V, O> where
    V: Variable,
    O: Operator,
{
    View::Var(x).roll()
}

impl<V, O> fmt::Display for Abt<V, O> where
    V: Variable,
    O: Operator,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.clone().unroll() {
            View::Var(v) => {
                try!(write!(f, "{}", v));
            }
            View::Abs(v, e) => {
                try!(write!(f, "{}.{}", v, e));
            }
            View::App(o, es) => {
                try!(write!(f, "{}", o));
                if es.len() > 0 {
                    let mut i = 0;
                    try!(write!(f, "("));
                    while i < es.len() * 2 - 1 {
                        if i & 1 == 0 {
                            try!(write!(f, "{}", es[i / 2]));
                        } else {
                            try!(write!(f, "; "));
                        }
                        i += 1;
                    }
                    try!(write!(f, ")"));
                }
            }
        }
        write!(f, "")
    }
}