use crate::*;

impl<X, A: Dfa<X>> ThreeFA<X> for FromDfa<A> {
  type Pre = ();
  type Active = A::State;
  type Post = ();
  fn initial(&self) -> Self::Pre {
    ()
  }
  fn step_pre(&self, _: Self::Pre, _: X) -> Option<Self::Pre> {
    Some(())
  }
  fn step_active(&self, state: Self::Active, char: X) -> Option<Self::Active> {
    self.0.next(state, char)
  }
  fn step_post(&self, _: Self::Post, _: X) -> Option<Self::Post> {
    Some(())
  }
  fn accept(&self, _: &Self::Post) -> bool {
    true
  }
  fn enter(&self, _: Self::Pre) -> Option<Self::Active> {
    Some(self.0.initial())
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
    if self.0.accept(&state) {
      Some(())
    } else {
      None
    }
  }
}
