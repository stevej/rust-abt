use std::fmt::{Display};

pub trait Operator: Clone + Eq + Display {
    fn arity(self) -> Vec<u64>;
}
