use std::fmt::{Display};

pub trait Variable: Clone + Eq + Ord + Display {
    fn named(String) -> Self;
    fn name(self) -> String;
    fn prime(self) -> Self;
}
