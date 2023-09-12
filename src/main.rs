use std::collections::{BTreeMap, BTreeSet};

mod dfa;
mod equal;
mod finite;
mod matches;
mod regex;
mod threefa;
mod to_regex;

use dfa::*;
use equal::*;
use finite::*;
use matches::*;
use regex::*;
use threefa::*;
use to_regex::*;

fn main() {
  #[rustfmt::skip]
  let x = dfa![ a* b ];
  assert!(matches(x, b"b"));
  assert!(matches(x, b"ab"));
  assert!(matches(x, b"aaab"));
  assert!(!matches(x, b""));
  assert!(!matches(x, b"a"));
  assert!(!matches(x, b"aba"));
  assert!(!matches(x, b"bbb"));

  let y = regex![ ^(?= a* b) aaa ];
  assert!(matches(y, b"aaab"));
  assert!(matches(y, b"aaaab"));
  assert!(matches(y, b"aaaabab"));
  assert!(!matches(y, b"b"));
  assert!(!matches(y, b"aab"));
  assert!(!matches(y, b"aabaaab"));

  let alphabet = b"abx";

  assert_eq!(equal(y, dfa![ aaa a* b .* ], alphabet), Ok(()));

  assert_eq!(
    equal(regex![ (?=a* b) aaa ], dfa![ .* aaa a* b .* ], alphabet),
    Ok(())
  );

  assert_eq!(
    equal(regex![ (?= .* b $) a+ ], dfa![ .* a .* b ], alphabet),
    Ok(())
  );

  assert_eq!(
    equal(
      regex![ ((?<= ab) ... (?! b))+ $ ],
      dfa![ .* ab ((a|x) ab)* ... ],
      alphabet
    ),
    Ok(())
  );

  println!("{}", to_regex(regex![ ^ (?=a* b) aaa ], alphabet));
}
