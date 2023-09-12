use crate::*;

impl<X, A: Dfa<X>> Dfa<X> for Not<A> {
  type State = Option<A::State>;
  fn initial(&self) -> Self::State {
    Some(self.0.initial())
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    Some(state.and_then(|state| self.0.next(state, char)))
  }
  fn accept(&self, state: &Self::State) -> bool {
    !state.as_ref().map_or(false, |state| self.0.accept(state))
  }
}
