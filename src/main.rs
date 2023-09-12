use std::collections::{BTreeMap, BTreeSet};

mod dfa;
mod equal;
mod matches;
mod regex;
mod threefa;
use dfa::*;
use equal::*;
use matches::*;
use regex::*;
use threefa::*;

fn main() {
  #[rustfmt::skip]
  let x = regex!(b"a"* b"b");
  assert!(matches(x, b"b".into_iter().copied()));
  assert!(matches(x, b"ab".into_iter().copied()));
  assert!(matches(x, b"aaab".into_iter().copied()));
  assert!(!matches(x, b"".into_iter().copied()));
  assert!(!matches(x, b"a".into_iter().copied()));
  assert!(!matches(x, b"aba".into_iter().copied()));
  assert!(!matches(x, b"bbb".into_iter().copied()));
  let y = ToDfa(regex!(^(?= b"a"* b"b") b"aaa"));
  assert!(matches(y, b"aaab".into_iter().copied()));
  assert!(matches(y, b"aaaab".into_iter().copied()));
  assert!(matches(y, b"aaaabab".into_iter().copied()));
  assert!(!matches(y, b"b".into_iter().copied()));
  assert!(!matches(y, b"aab".into_iter().copied()));
  assert!(!matches(y, b"aabaaab".into_iter().copied()));
  let y2 = regex!(b"aaa" b"a"* b"b" .*);
  assert!(!matches(y2, b"b".into_iter().copied()));
  assert_eq!(dfa_equal(y, y2, b"abx".iter().copied()), Ok(()));

  assert_eq!(
    dfa_equal(
      ToDfa(regex!((?=a* b) aaa)),
      regex!(.* aaa a* b .*),
      b"abx".iter().copied()
    ),
    Ok(())
  );

  assert_eq!(
    dfa_equal(
      ToDfa(regex!((?=.* b$) a+)),
      regex!(.* a .* b),
      b"abx".iter().copied()
    ),
    Ok(())
  );

  assert_eq!(
    dfa_equal(
      ToDfa(regex!(((?<=ab)...(?!b))+$)),
      regex!(.* ab ((a|x) ab)* ...),
      b"abx".iter().copied()
    ),
    Ok(())
  );
}
