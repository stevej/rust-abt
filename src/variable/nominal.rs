use std::cell::{RefCell};
use std::thread;

thread_local!(static COUNTER: RefCell<u64> = RefCell::new(0));

fn with_fresh<F, R>(k: F) -> R where
    F: FnOnce(u64) -> R
{
    COUNTER.with(|x| {
        *x.borrow_mut() += 1;
        k(*x.borrow())
    })
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Nominal {
    name: String,
    index: u64
}

impl Clone for Nominal {
    fn clone(&self) -> Nominal {
        with_fresh(|n| {
            Nominal { name: self.name.clone(), index: n }
        })
    }
}

impl Nominal {
    pub fn new(x: String) -> Nominal {
        with_fresh(|n| {
            Nominal { name: x, index: n }
        })
    }
}