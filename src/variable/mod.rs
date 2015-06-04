use std::fmt::{Display};

pub mod nominal;

pub trait Variable: Clone + Eq + Ord + Display {
    fn new(String) -> Self;
    fn dup(&self) -> Self;
    fn name(&self) -> String;
    fn prime(&self) -> Self;
}

impl Variable for String {
    fn dup(&self) -> String {
        self.clone()
    }

    fn new(x: String) -> String {
        x
    }

    fn name(&self) -> String {
        self.clone()
    }

    fn prime(&self) -> String {
        let mut res = self.clone();
        res.push_str("'");
        res
    }
}
