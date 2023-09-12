use crate::*;

impl<X: Clone, A: ThreeFA<X>, B: ThreeFA<X>> ThreeFA<X> for Or<A, B> {
  type Pre = (Option<A::Pre>, Option<B::Pre>);
  type Active = (Option<A::Active>, Option<B::Active>);
  type Post = (Option<A::Post>, Option<B::Post>);
  fn initial(&self) -> Self::Pre {
    (Some(self.0.initial()), Some(self.1.initial()))
  }
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    let a = state.0.and_then(|x| self.0.step_pre(x, char.clone()));
    let b = state.1.and_then(|x| self.1.step_pre(x, char));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn step_active(&self, state: Self::Active, char: X) -> Option<Self::Active> {
    let a = state.0.and_then(|x| self.0.step_active(x, char.clone()));
    let b = state.1.and_then(|x| self.1.step_active(x, char));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let a = state.0.and_then(|x| self.0.step_post(x, char.clone()));
    let b = state.1.and_then(|x| self.1.step_post(x, char));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state.0.as_ref().map_or(false, |x| self.0.accept(x))
      || state.1.as_ref().map_or(false, |x| self.1.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::Active> {
    let a = state.0.and_then(|x| self.0.enter(x));
    let b = state.1.and_then(|x| self.1.enter(x));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
    let a = state.0.and_then(|x| self.0.exit(x));
    let b = state.1.and_then(|x| self.1.exit(x));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
}
