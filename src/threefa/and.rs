use crate::*;

impl<X: Clone, A: ThreeFA<X>, B: ThreeFA<X>> ThreeFA<X> for And<A, B> {
  type Pre = (A::Pre, B::Pre);
  type Active = (A::Active, B::Active);
  type Post = (A::Post, B::Post);
  fn initial(&self) -> Self::Pre {
    (self.0.initial(), self.1.initial())
  }
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    Some((
      self.0.step_pre(state.0, char.clone())?,
      self.1.step_pre(state.1, char)?,
    ))
  }
  fn step_active(&self, state: Self::Active, char: X) -> Option<Self::Active> {
    Some((
      self.0.step_active(state.0, char.clone())?,
      self.1.step_active(state.1, char)?,
    ))
  }
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    Some((
      self.0.step_post(state.0, char.clone())?,
      self.1.step_post(state.1, char)?,
    ))
  }
  fn accept(&self, state: &Self::Post) -> bool {
    self.0.accept(&state.0) && self.1.accept(&state.1)
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::Active> {
    Some((self.0.enter(state.0)?, self.1.enter(state.1)?))
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
    Some((self.0.exit(state.0)?, self.1.exit(state.1)?))
  }
}
