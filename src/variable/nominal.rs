use std::cell::{RefCell};
use std::cmp;
use std::fmt;
use variable::{Variable};

thread_local!(static COUNTER: RefCell<u64> = RefCell::new(0));

fn with_fresh<F, R>(k: F) -> R where
    F: FnOnce(u64) -> R
{
    COUNTER.with(|x| {
        *x.borrow_mut() += 1;
        k(*x.borrow())
    })
}

#[derive(Clone, Debug)]
pub struct Nominal {
    name: Option<String>,
    index: u64
}

impl PartialEq for Nominal {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Nominal {}

impl PartialOrd for Nominal {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for Nominal {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl fmt::Display for Nominal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}@{}", self.name, self.index)
    }
}

impl Nominal {
    pub fn nameless() -> Nominal {
        with_fresh(|n| {
            Nominal { name: None, index: n }
        })
    }
}

impl Variable for Nominal {
    fn new(s: String) -> Nominal {
        let mut res = Self::nameless();
        res.name = Some(s);
        res
    }

    fn dup(&self) -> Nominal {
        with_fresh(|n| {
            Nominal { name: self.name.clone(), index: n }
        })
    }

    fn name(&self) -> String {
        match self.name {
            None => { format!("@{}", self.index) }
            Some(ref s) => { s.clone() }
        }
    }

    fn prime(&self) -> Nominal {
        self.dup()
    }
}