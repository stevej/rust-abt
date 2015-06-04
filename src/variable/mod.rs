use std::fmt::{Display};

pub mod nominal;

pub trait Variable: Clone + Eq + Ord + Display {
    fn dup(&self) -> Self;
    fn named(String) -> Self;
    fn name(self) -> String;
    fn prime(self) -> Self;
}

impl Variable for String {
    fn dup(&self) -> String {
        self.clone()
    }

    fn named(x: String) -> String {
        x
    }

    fn name(self) -> String {
        self
    }

    fn prime(self) -> String {
        let mut res = self.clone();
        res.push_str("'");
        res
    }
}
