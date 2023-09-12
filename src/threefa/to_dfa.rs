use crate::*;

impl<X: Clone, A: ThreeFA<X>> Dfa<X> for ToDfa<A>
where
  A::Pre: Clone,
  A::Active: Ord + Clone,
  A::Post: Ord + Clone,
{
  type State = (Option<A::Pre>, BTreeSet<A::Active>, BTreeSet<A::Post>);
  fn initial(&self) -> Self::State {
    let i = self.0.initial();
    let s = self.0.enter(i.clone());
    let e = s.clone().and_then(|x| self.0.exit(x));
    (Some(i), s.into_iter().collect(), e.into_iter().collect())
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    let i = state.0.and_then(|x| self.0.step_pre(x, char.clone()));
    let s = state
      .1
      .into_iter()
      .filter_map(|x| self.0.step_active(x, char.clone()))
      .chain(i.clone().and_then(|x| self.0.enter(x)))
      .collect::<BTreeSet<_>>();
    let e = state
      .2
      .into_iter()
      .filter_map(|x| self.0.step_post(x, char.clone()))
      .chain(s.iter().filter_map(|x| self.0.exit(x.clone())))
      .collect::<BTreeSet<_>>();
    if i.is_some() || !s.is_empty() || !e.is_empty() {
      Some((i, s, e))
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.2.iter().any(|x| self.0.accept(x))
  }
}
