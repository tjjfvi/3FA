use crate::*;

mod anything;
mod concat;
mod dot;
mod empty;
mod from_dfa;
mod iff;
mod not;
mod or;
mod plus;
mod str;

pub trait Dfa<X> {
  type State: Finite;
  fn initial(&self) -> Self::State;
  fn next(&self, state: Self::State, char: X) -> Option<Self::State>;
  fn accept(&self, state: &Self::State) -> bool;
}

impl<'a, X, T: Dfa<X> + ?Sized> Dfa<X> for &'a T {
  type State = T::State;
  fn initial(&self) -> Self::State {
    (*self).initial()
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    (*self).next(state, char)
  }
  fn accept(&self, state: &Self::State) -> bool {
    (*self).accept(state)
  }
}
