use crate::*;

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for Iff<A, B> {
  type State = (Option<A::State>, Option<B::State>);
  fn initial(&self) -> Self::State {
    (Some(self.0.initial()), Some(self.1.initial()))
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    Some((
      state.0.and_then(|x| self.0.next(x, char.clone())),
      state.1.and_then(|x| self.1.next(x, char)),
    ))
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.0.as_ref().map_or(false, |x| self.0.accept(x))
      == state.1.as_ref().map_or(false, |x| self.1.accept(x))
  }
}
