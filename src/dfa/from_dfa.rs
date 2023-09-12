use crate::*;

impl<X, A: Dfa<X>> Dfa<X> for FromDfa<A> {
  type State = A::State;
  fn initial(&self) -> Self::State {
    self.0.initial()
  }
  fn accept(&self, state: &Self::State) -> bool {
    self.0.accept(state)
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    self.0.next(state, char)
  }
}
