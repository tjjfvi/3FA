use crate::*;

pub fn matches<X, D: Dfa<X>>(dfa: D, str: impl IntoIterator<Item = X>) -> bool {
  let mut state = dfa.initial();
  for char in str {
    state = match dfa.next(state, char) {
      Some(x) => x,
      None => return false,
    }
  }
  dfa.accept(&state)
}
