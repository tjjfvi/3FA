use crate::*;

impl<'a, C: Eq> Dfa<C> for [C] {
  type State = usize;
  fn initial(&self) -> Self::State {
    0
  }
  fn next(&self, state: Self::State, char: C) -> Option<Self::State> {
    if self.get(state) == Some(&char) {
      Some(state + 1)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state == &self.len()
  }
}

impl<'a, C: Eq, const N: usize> Dfa<C> for [C; N] {
  type State = usize;
  fn initial(&self) -> Self::State {
    0
  }
  fn next(&self, state: Self::State, char: C) -> Option<Self::State> {
    if self.get(state) == Some(&char) {
      Some(state + 1)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state == &self.len()
  }
}
