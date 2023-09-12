use crate::*;

impl<X: Clone, A: ThreeFA<X>> ThreeFA<X> for LookBehind<A>
where
  A::Pre: Clone + Ord,
  A::Active: Clone + Ord,
  A::Post: Ord,
{
  type Pre = (BTreeSet<A::Pre>, BTreeSet<A::Active>);
  type Active = (BTreeSet<A::Pre>, BTreeSet<A::Active>);
  type Post = BTreeSet<A::Post>;
  fn initial(&self) -> Self::Pre {
    (
      [self.0.initial()].into_iter().collect(),
      self.0.enter(self.0.initial()).into_iter().collect(),
    )
  }
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    let s = state
      .0
      .into_iter()
      .filter_map(|x| self.0.step_pre(x, char.clone()))
      .collect::<BTreeSet<_>>();
    let e = state
      .1
      .into_iter()
      .filter_map(|x| self.0.step_active(x, char.clone()))
      .chain(s.iter().filter_map(|x| self.0.enter(x.clone())))
      .collect::<BTreeSet<_>>();
    if !s.is_empty() || !e.is_empty() {
      Some((s, e))
    } else {
      None
    }
  }
  fn step_active(&self, _: Self::Active, _: X) -> Option<Self::Active> {
    None
  }
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let x = state
      .into_iter()
      .filter_map(|x| self.0.step_post(x, char.clone()))
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state.iter().any(|x| self.0.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::Active> {
    Some(state)
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
    let x = state
      .1
      .into_iter()
      .filter_map(|x| self.0.exit(x))
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
  }
}
