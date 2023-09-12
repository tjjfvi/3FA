use crate::*;

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for Concat<A, B>
where
  B::State: Ord,
{
  type State = (Option<A::State>, BTreeSet<B::State>);
  fn initial(&self) -> Self::State {
    let a = self.0.initial();
    let b = if self.0.accept(&a) {
      Some(self.1.initial())
    } else {
      None
    };
    (Some(a), b.into_iter().collect())
  }
  fn next(&self, mut state: Self::State, char: X) -> Option<Self::State> {
    state.0 = state.0.and_then(|x| self.0.next(x, char.clone()));
    state.1 = state
      .1
      .into_iter()
      .filter_map(|x| self.1.next(x, char.clone()))
      .chain(state.0.as_ref().and_then(|x| {
        if self.0.accept(&x) {
          Some(self.1.initial())
        } else {
          None
        }
      }))
      .collect();
    if state.0.is_some() || !state.1.is_empty() {
      Some(state)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.1.iter().any(|x| self.1.accept(x))
  }
}
