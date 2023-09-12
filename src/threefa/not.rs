use crate::*;

impl<X: Clone, A: ThreeFA<X>> ThreeFA<X> for Not<A> {
  type Pre = Option<A::Pre>;
  type Active = Option<A::Active>;
  type Post = Option<A::Post>;
  fn initial(&self) -> Self::Pre {
    Some(self.0.initial())
  }
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    Some(state.and_then(|x| self.0.step_pre(x, char)))
  }
  fn step_active(&self, state: Self::Active, char: X) -> Option<Self::Active> {
    Some(state.and_then(|x| self.0.step_active(x, char)))
  }
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    Some(state.and_then(|x| self.0.step_post(x, char)))
  }
  fn accept(&self, state: &Self::Post) -> bool {
    !state.as_ref().map_or(false, |x| self.0.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::Active> {
    Some(state.and_then(|x| self.0.enter(x)))
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
    Some(state.and_then(|x| self.0.exit(x)))
  }
}
