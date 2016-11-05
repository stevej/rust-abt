pub trait Into<V, O, A> {
    fn into_a(self) -> A;
}

pub trait From<V, O>: Eq where View<V, O, Self>: Into<V, O, Self>, Self: Sized {
    fn from_a(self) -> View<V, O, Self>;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum View<V, O, A> {
    Var(V),
    Abs(V, A),
    App(O, Vec<A>),
}
