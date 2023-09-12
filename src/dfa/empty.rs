use crate::*;

impl<X> Dfa<X> for Empty {
  type State = ();
  fn initial(&self) -> Self::State {}
  fn next(&self, _: Self::State, _: X) -> Option<Self::State> {
    None
  }
  fn accept(&self, _: &Self::State) -> bool {
    true
  }
}
