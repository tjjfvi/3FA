use crate::*;

pub fn equal<X: Clone, A: Dfa<X>, B: Dfa<X>>(
  a: A,
  b: B,
  alphabet: impl Clone + IntoIterator<Item = X>,
) -> Result<(), Vec<X>>
where
  A::State: Clone + Ord,
  B::State: Clone + Ord,
{
  is_empty(Not(Iff(a, b)), alphabet)
}

pub fn is_empty<X: Clone, D: Dfa<X>>(
  dfa: D,
  alphabet: impl Clone + IntoIterator<Item = X>,
) -> Result<(), Vec<X>>
where
  D::State: Clone + Ord,
{
  let mut visited = BTreeSet::new();
  let mut next = BTreeMap::new();
  next.insert(dfa.initial(), vec![]);
  while !next.is_empty() {
    for (state, msg) in std::mem::take(&mut next) {
      if dfa.accept(&state) {
        return Err(msg);
      }
      for char in alphabet.clone() {
        if let Some(state) = dfa.next(state.clone(), char.clone()) {
          if visited.insert(state.clone()) {
            next.insert(state, msg.iter().cloned().chain([char]).collect());
          }
        }
      }
    }
  }
  Ok(())
}
