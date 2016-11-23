pub trait Roll<V, O, A> {
    fn roll(self) -> A;
}

pub trait Unroll<V, O>: Eq where View<V, O, Self>: Roll<V, O, Self>, Self: Sized {
    fn unroll(self) -> View<V, O, Self>;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum View<V, O, A> {
    Var(V),
    Abs(V, A),
    App(O, Vec<A>),
}
