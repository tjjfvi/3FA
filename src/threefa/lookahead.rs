use crate::*;

impl<X: Clone, A: ThreeFA<X>> ThreeFA<X> for LookAhead<A>
where
  A::Pre: Clone,
  A::Active: Clone + Ord,
  A::Post: Ord,
{
  type Pre = A::Pre;
  type Active = A::Pre;
  type Post = (BTreeSet<A::Active>, BTreeSet<A::Post>);
  fn initial(&self) -> Self::Pre {
    self.0.initial()
  }
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    self.0.step_pre(state, char)
  }
  fn step_active(&self, _: Self::Active, _: X) -> Option<Self::Active> {
    None
  }
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let s = state
      .0
      .into_iter()
      .filter_map(|x| self.0.step_active(x, char.clone()))
      .collect::<BTreeSet<_>>();
    let e = state
      .1
      .into_iter()
      .filter_map(|x| self.0.step_post(x, char.clone()))
      .chain(s.iter().filter_map(|x| self.0.exit(x.clone())))
      .collect::<BTreeSet<_>>();
    if !s.is_empty() || !e.is_empty() {
      Some((s, e))
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state.1.iter().any(|x| self.0.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::Active> {
    Some(state)
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
    let state = self.0.enter(state)?;
    Some((
      [state.clone()].into_iter().collect(),
      self.0.exit(state).into_iter().collect(),
    ))
  }
}
