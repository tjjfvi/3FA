use crate::*;

#[derive(Debug, Clone)]
pub struct BakedDfa<X>(Vec<(bool, BTreeMap<X, usize>)>);

pub fn bake_dfa<'a, X: Clone + Ord + 'a, D: Dfa<X>>(
  dfa: D,
  alphabet: impl Clone + IntoIterator<Item = &'a X>,
) -> BakedDfa<X>
where
  D::State: Clone + Ord,
{
  let mut states = Vec::new();
  let mut reverse = BTreeMap::new();

  visit(&dfa, alphabet, &mut states, &mut reverse, &dfa.initial());

  return BakedDfa(states);

  fn visit<'a, X: Clone + Ord + 'a, D: Dfa<X>>(
    dfa: &D,
    alphabet: impl Clone + IntoIterator<Item = &'a X>,
    states: &mut Vec<(bool, BTreeMap<X, usize>)>,
    reverse: &mut BTreeMap<D::State, usize>,
    state: &D::State,
  ) -> usize
  where
    D::State: Clone + Ord,
  {
    let id = states.len();
    reverse.insert(state.clone(), id);
    states.push((dfa.accept(&state), BTreeMap::new()));
    for char in alphabet.clone() {
      if let Some(next) = dfa.next(state.clone(), char.clone()) {
        let next = reverse
          .get(&next)
          .copied()
          .unwrap_or_else(|| visit(dfa, alphabet.clone(), states, reverse, &next));
        states.get_mut(id).unwrap().1.insert(char.clone(), next);
      }
    }
    id
  }
}

impl<X: Ord> Dfa<X> for BakedDfa<X> {
  type State = usize;
  fn initial(&self) -> Self::State {
    0
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    self.0[state].1.get(&char).copied()
  }
  fn accept(&self, state: &Self::State) -> bool {
    self.0[*state].0
  }
}

impl<'a, X: Ord> Dfa<&'a X> for BakedDfa<X> {
  type State = usize;
  fn initial(&self) -> Self::State {
    0
  }
  fn next(&self, state: Self::State, char: &'a X) -> Option<Self::State> {
    self.0[state].1.get(char).copied()
  }
  fn accept(&self, state: &Self::State) -> bool {
    self.0[*state].0
  }
}
