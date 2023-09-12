use crate::*;

impl<X> Dfa<X> for Dot {
  type State = bool;
  fn initial(&self) -> Self::State {
    false
  }
  fn next(&self, state: Self::State, _: X) -> Option<Self::State> {
    match state {
      false => Some(true),
      _ => None,
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    *state
  }
}
