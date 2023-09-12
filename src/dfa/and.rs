use crate::*;

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for And<A, B> {
  type State = (A::State, B::State);
  fn initial(&self) -> Self::State {
    (self.0.initial(), self.1.initial())
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    Some((
      self.0.next(state.0, char.clone())?,
      self.1.next(state.1, char)?,
    ))
  }
  fn accept(&self, state: &Self::State) -> bool {
    self.0.accept(&state.0) && self.1.accept(&state.1)
  }
}
