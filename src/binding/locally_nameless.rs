use operator::{Operator};
use variable::{Variable};
use view::{From, Into, View};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Abt<V, O> where V: Variable, O: Operator {
    Free(V),
    Bound(u64),
    Abs(V, Box<Abt<V, O>>),
    App(O, Vec<Abt<V, O>>),
}

impl<V, O> Abt<V, O> where
    V: Variable,
    O: Operator,
{
    fn match_arity(&self, n: u64) -> bool {
        match self {
            &Abt::Abs(_, _) if n == 0 => { false }
            &Abt::Abs(_, ref e) => { e.match_arity(n - 1) }
            _ if n == 0 => { true }
            _ => { false }
        }
    }

    fn shift_var(self, v: V, n: u64) -> Self {
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
                Abt::App(o, unimplemented!())
            }
        }
    }

    fn add_var(self, v: V, n: u64) -> Self {
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
                Abt::App(o, unimplemented!())
            }
        }
    }
}

impl<V, O> Into<V, O, Abt<V, O>> for View<V, O, Abt<V, O>> where
    V: Variable,
    O: Operator,
{
    fn into(self) -> Abt<V, O> {
        match self {
            View::View(v) => {
                Abt::Free(v)
            }
            View::Abs(v, e) => {
                Abt::Abs(v.clone(), Box::new(e.shift_var(v, 0)))
            }
            View::App(o, es) => {
                let ar = o.clone().arity();
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

impl<V, O> From<V, O> for Abt<V, O> where
    V: Variable,
    O: Operator,
    View<V, O, Abt<V, O>>: Into<V, O, Abt<V, O>>,
{
    fn from(self) -> View<V, O, Abt<V, O>> {
        match self {
            Abt::Free(v) => {
                View::View(v)
            }
            Abt::Bound(n) => {
                panic!()
            }
            Abt::Abs(x, e) => {
                unimplemented!()
            }
            Abt::App(o, es) => {
                View::App(o, es)
            }
        }
    }
}
