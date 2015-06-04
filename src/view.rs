pub trait Into<V, O, A> {
    fn into(self) -> A;
}

pub trait From<V, O>: Eq where View<V, O, Self>: Into<V, O, Self> {
    fn from(self) -> View<V, O, Self>;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum View<V, O, A> {
    Var(V),
    Abs(V, A),
    App(O, Vec<A>),
}
