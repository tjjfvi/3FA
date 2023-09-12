use crate::*;

impl<X: Clone, A: Dfa<X>> Dfa<X> for Plus<A>
where
  A::State: Ord,
{
  type State = BTreeSet<A::State>;
  fn initial(&self) -> Self::State {
    [self.0.initial()].into_iter().collect()
  }
  fn next(&self, mut state: Self::State, char: X) -> Option<Self::State> {
    state = state
      .into_iter()
      .filter_map(|x| self.0.next(x, char.clone()))
      .collect();
    if self.accept(&state) {
      state.insert(self.0.initial());
    }
    if !state.is_empty() {
      Some(state)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.iter().any(|x| self.0.accept(x))
  }
}
