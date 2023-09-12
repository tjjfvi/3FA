use std::{
  collections::{BTreeMap, BTreeSet},
  fmt::Debug,
};

trait Dfa<X> {
  type State;
  fn initial(&self) -> Self::State;
  fn next(&self, state: Self::State, char: X) -> Self::State;
  fn accept(&self, state: &Self::State) -> bool;
}

impl<'a, X, T: Dfa<X>> Dfa<X> for &'a T {
  type State = T::State;
  fn initial(&self) -> Self::State {
    (*self).initial()
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    (*self).next(state, char)
  }
  fn accept(&self, state: &Self::State) -> bool {
    (*self).accept(state)
  }
}

impl<'a, C: Eq> Dfa<C> for [C] {
  type State = Option<usize>;
  fn initial(&self) -> Self::State {
    Some(0)
  }
  fn next(&self, state: Self::State, char: C) -> Self::State {
    state.and_then(|i| {
      if self.get(i) == Some(&char) {
        Some(i + 1)
      } else {
        None
      }
    })
  }
  fn accept(&self, state: &Self::State) -> bool {
    state == &Some(self.len())
  }
}

impl<'a, C: Eq, const N: usize> Dfa<C> for [C; N] {
  type State = Option<usize>;
  fn initial(&self) -> Self::State {
    Some(0)
  }
  fn next(&self, state: Self::State, char: C) -> Self::State {
    state.and_then(|i| {
      if self.get(i) == Some(&char) {
        Some(i + 1)
      } else {
        None
      }
    })
  }
  fn accept(&self, state: &Self::State) -> bool {
    state == &Some(self.len())
  }
}

#[derive(Debug, Clone, Copy)]
struct Not<A>(A);

impl<X, A: Dfa<X>> Dfa<X> for Not<A> {
  type State = A::State;
  fn initial(&self) -> Self::State {
    self.0.initial()
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    self.0.next(state, char)
  }
  fn accept(&self, state: &Self::State) -> bool {
    !self.0.accept(state)
  }
}

#[derive(Debug, Clone, Copy)]
struct Or<A, B>(A, B);

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for Or<A, B> {
  type State = (A::State, B::State);
  fn initial(&self) -> Self::State {
    (self.0.initial(), self.1.initial())
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    (
      self.0.next(state.0, char.clone()),
      self.1.next(state.1, char),
    )
  }
  fn accept(&self, state: &Self::State) -> bool {
    self.0.accept(&state.0) || self.1.accept(&state.1)
  }
}

#[derive(Debug, Clone, Copy)]
struct Iff<A, B>(A, B);

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for Iff<A, B> {
  type State = (A::State, B::State);
  fn initial(&self) -> Self::State {
    (self.0.initial(), self.1.initial())
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    (
      self.0.next(state.0, char.clone()),
      self.1.next(state.1, char),
    )
  }
  fn accept(&self, state: &Self::State) -> bool {
    self.0.accept(&state.0) == self.1.accept(&state.1)
  }
}

#[derive(Debug, Clone, Copy)]
struct Concat<A, B>(A, B);

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for Concat<A, B>
where
  B::State: Ord,
{
  type State = (A::State, BTreeSet<B::State>);
  fn initial(&self) -> Self::State {
    let a = self.0.initial();
    let b = if self.0.accept(&a) {
      Some(self.1.initial())
    } else {
      None
    };
    (a, b.into_iter().collect())
  }
  fn next(&self, mut state: Self::State, char: X) -> Self::State {
    state.0 = self.0.next(state.0, char.clone());
    state.1 = state
      .1
      .into_iter()
      .map(|x| self.1.next(x, char.clone()))
      .chain(if self.0.accept(&state.0) {
        Some(self.1.initial())
      } else {
        None
      })
      .collect();
    state
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.1.iter().any(|x| self.1.accept(x))
  }
}

#[derive(Debug, Clone, Copy)]
struct Empty;

impl<X> Dfa<X> for Empty {
  type State = bool;
  fn initial(&self) -> Self::State {
    true
  }
  fn next(&self, _: Self::State, _: X) -> Self::State {
    false
  }
  fn accept(&self, state: &Self::State) -> bool {
    *state
  }
}

#[derive(Debug, Clone, Copy)]
struct Anything;

impl<X> Dfa<X> for Anything {
  type State = ();
  fn initial(&self) -> Self::State {
    ()
  }
  fn next(&self, _: Self::State, _: X) -> Self::State {
    ()
  }
  fn accept(&self, _: &Self::State) -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy)]
struct Dot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DotState {
  Start,
  Accept,
  Fail,
}

impl<X> Dfa<X> for Dot {
  type State = DotState;
  fn initial(&self) -> Self::State {
    DotState::Start
  }
  fn next(&self, state: Self::State, _: X) -> Self::State {
    match state {
      DotState::Start => DotState::Accept,
      _ => DotState::Fail,
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    match state {
      DotState::Accept => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Plus<A>(A);

impl<X: Clone, A: Dfa<X>> Dfa<X> for Plus<A>
where
  A::State: Ord,
{
  type State = BTreeSet<A::State>;
  fn initial(&self) -> Self::State {
    [self.0.initial()].into_iter().collect()
  }
  fn next(&self, mut state: Self::State, char: X) -> Self::State {
    state = state
      .into_iter()
      .map(|x| self.0.next(x, char.clone()))
      .collect();
    if self.accept(&state) {
      state.insert(self.0.initial());
    }
    state
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.iter().any(|x| self.0.accept(x))
  }
}

fn is_empty<X: Clone + Debug, D: Dfa<X>>(dfa: D, alphabet: impl Clone + Iterator<Item = X>) -> bool
where
  D::State: Clone + Ord,
{
  let mut visited = BTreeSet::new();
  let mut next = BTreeMap::new();
  next.insert(dfa.initial(), vec![]);
  while !next.is_empty() {
    for (state, msg) in std::mem::take(&mut next) {
      if dfa.accept(&state) {
        dbg!(msg);
        return false;
      }
      for char in alphabet.clone() {
        let state = dfa.next(state.clone(), char.clone());
        if visited.insert(state.clone()) {
          next.insert(state, msg.iter().cloned().chain([char]).collect());
        }
      }
    }
  }
  true
}

fn dfa_equal<X: Clone + Debug, A: Dfa<X>, B: Dfa<X>>(
  a: A,
  b: B,
  alphabet: impl Clone + Iterator<Item = X>,
) -> bool
where
  A::State: Clone + Ord,
  B::State: Clone + Ord,
{
  is_empty(Not(Iff(a, b)), alphabet)
}

macro_rules! regex {
  ( (?= $($x:tt)*) ) => {
    LookAhead(regex!($($x)*))
  };
  ( (?! $($x:tt)*) ) => {
    LookAhead(Not(regex!($($x)*)))
  };
  ( (?<= $($x:tt)*) ) => {
    LookBehind(regex!($($x)*))
  };
  ( (?<! $($x:tt)*) ) => {
    LookBehind(Not(regex!($($x)*)))
  };
  ( ($x:expr) ) => {
    regex!($x)
  };
  ( {$x:expr} ) => {
    $x
  };
  ( $x:literal ) => {
    FromDfa($x)
  };
  ( ^ ) => {
    Start
  };
  ( $ ) => {
    End
  };
  ( . ) => {
    FromDfa(Dot)
  };
  ( $x:tt | $($y:tt)* ) => {
    Or(regex!($x), regex!($($y)*))
  };
  ( $x:tt ? $($y:tt)* ) => {
    regex!({Or(FromDfa(Empty), regex!($x))} $($y)*)
  };
  ( $x:tt * $($y:tt)* ) => {
    regex!({Or(FromDfa(Empty), Plus(regex!($x)))} $($y)*)
  };
  ( $x:tt + $($y:tt)* ) => {
    regex!({Plus(regex!($x))} $($y)*)
  };
  ( $x:tt $($y:tt)+ ) => {
    Concat(regex!($x), regex!($($y)+))
  };
}

fn matches<X, D: Dfa<X>>(dfa: D, str: impl IntoIterator<Item = X>) -> bool {
  let mut state = dfa.initial();
  for char in str {
    state = dfa.next(state, char)
  }
  dfa.accept(&state)
}

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
  dbg!(regex!(b"aaa" b"a"* b"b" .*));
  assert!(dfa_equal(
    y,
    regex!(b"aaa" b"a"* b"b" .*),
    b"abx".iter().copied()
  ));
}

trait Regex<X> {
  type Pre;
  type State;
  type Post;
  fn initial(&self) -> Self::Pre;
  fn pre(&self, state: Self::Pre, char: X) -> Self::Pre;
  fn next(&self, state: Self::State, char: X) -> Self::State;
  fn post(&self, state: Self::Post, char: X) -> Self::Post;
  fn accept(&self, state: &Self::Post) -> bool;
  fn enter(&self, state: Self::Pre) -> Self::State;
  fn exit(&self, state: Self::State) -> Self::Post;
}

#[derive(Debug, Clone, Copy)]
struct Start;

impl<X> Regex<X> for Start {
  type Pre = bool;
  type State = bool;
  type Post = bool;
  fn initial(&self) -> Self::State {
    true
  }
  fn pre(&self, _: Self::Pre, _: X) -> Self::Pre {
    false
  }
  fn next(&self, _: Self::State, _: X) -> Self::State {
    false
  }
  fn post(&self, state: Self::Post, _: X) -> Self::Post {
    state
  }
  fn accept(&self, state: &Self::Post) -> bool {
    *state
  }
  fn enter(&self, state: Self::Pre) -> Self::State {
    state
  }
  fn exit(&self, state: Self::State) -> Self::Post {
    state
  }
}

#[derive(Debug, Clone, Copy)]
struct End;

impl<X> Regex<X> for End {
  type Pre = ();
  type State = bool;
  type Post = bool;
  fn initial(&self) -> Self::Pre {
    ()
  }
  fn pre(&self, _: Self::Pre, _: X) -> Self::Pre {
    ()
  }
  fn next(&self, _: Self::State, _: X) -> Self::State {
    false
  }
  fn post(&self, _: Self::Post, _: X) -> Self::Post {
    false
  }
  fn accept(&self, state: &Self::Post) -> bool {
    *state
  }
  fn enter(&self, _: Self::Pre) -> Self::State {
    true
  }
  fn exit(&self, state: Self::State) -> Self::Post {
    state
  }
}

impl<X: Clone, A: Regex<X>, B: Regex<X>> Regex<X> for Or<A, B> {
  type Pre = (A::Pre, B::Pre);
  type State = (A::State, B::State);
  type Post = (A::Post, B::Post);
  fn initial(&self) -> Self::Pre {
    (self.0.initial(), self.1.initial())
  }
  fn pre(&self, state: Self::Pre, char: X) -> Self::Pre {
    (self.0.pre(state.0, char.clone()), self.1.pre(state.1, char))
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    (
      self.0.next(state.0, char.clone()),
      self.1.next(state.1, char),
    )
  }
  fn post(&self, state: Self::Post, char: X) -> Self::Post {
    (
      self.0.post(state.0, char.clone()),
      self.1.post(state.1, char),
    )
  }
  fn accept(&self, state: &Self::Post) -> bool {
    self.0.accept(&state.0) || self.1.accept(&state.1)
  }
  fn enter(&self, state: Self::Pre) -> Self::State {
    (self.0.enter(state.0), self.1.enter(state.1))
  }
  fn exit(&self, state: Self::State) -> Self::Post {
    (self.0.exit(state.0), self.1.exit(state.1))
  }
}

impl<X: Clone, A: Regex<X>, B: Regex<X>> Regex<X> for Concat<A, B>
where
  B::Pre: Clone,
  A::State: Ord + Clone,
  A::Post: Ord,
  B::Post: Ord,
  B::State: Ord,
{
  type Pre = (A::Pre, B::Pre);
  type State = ((A::State, B::Pre), BTreeSet<(A::Post, B::State)>);
  type Post = BTreeSet<(A::Post, B::Post)>;
  fn initial(&self) -> Self::Pre {
    (self.0.initial(), self.1.initial())
  }
  fn pre(&self, state: Self::Pre, char: X) -> Self::Pre {
    (
      self.0.pre(state.0, char.clone()),
      self.1.pre(state.1, char.clone()),
    )
  }
  fn next(&self, (x, state): Self::State, char: X) -> Self::State {
    let x = (
      self.0.next(x.0, char.clone()),
      self.1.pre(x.1, char.clone()),
    );
    (
      x.clone(),
      state
        .into_iter()
        .map(|x| {
          (
            self.0.post(x.0, char.clone()),
            self.1.next(x.1, char.clone()),
          )
        })
        .chain([(self.0.exit(x.0), self.1.enter(x.1))])
        .collect(),
    )
  }
  fn post(&self, state: Self::Post, char: X) -> Self::Post {
    state
      .into_iter()
      .map(|x| {
        (
          self.0.post(x.0, char.clone()),
          self.1.post(x.1, char.clone()),
        )
      })
      .collect()
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state
      .iter()
      .any(|x| self.0.accept(&x.0) && self.1.accept(&x.1))
  }
  fn enter(&self, state: Self::Pre) -> Self::State {
    let state = (self.0.enter(state.0), state.1);
    (
      state.clone(),
      [(self.0.exit(state.0), self.1.enter(state.1))]
        .into_iter()
        .collect(),
    )
  }
  fn exit(&self, state: Self::State) -> Self::Post {
    state
      .1
      .into_iter()
      .map(|x| (x.0, self.1.exit(x.1)))
      .collect()
  }
}

impl<X: Clone, A: Regex<X>> Regex<X> for Plus<A>
where
  A::Pre: Clone,
  A::State: Ord + Clone,
  A::Post: Ord + Clone,
{
  type Pre = A::Pre;
  type State = (A::Pre, BTreeSet<(BTreeSet<A::Post>, A::State)>);
  type Post = BTreeSet<BTreeSet<A::Post>>;
  fn initial(&self) -> Self::Pre {
    self.0.initial()
  }
  fn pre(&self, state: Self::Pre, char: X) -> Self::Pre {
    self.0.pre(state, char)
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    let a = self.0.pre(state.0, char.clone());
    (
      a.clone(),
      state
        .1
        .into_iter()
        .map(|x| {
          (
            x.0
              .into_iter()
              .map(|x| self.0.post(x, char.clone()))
              .collect::<BTreeSet<_>>(),
            self.0.next(x.1, char.clone()),
          )
        })
        .flat_map(|mut x| {
          [x.clone(), {
            x.0.insert(self.0.exit(x.1));
            (x.0, self.0.enter(a.clone()))
          }]
        })
        .collect(),
    )
  }
  fn post(&self, state: Self::Post, char: X) -> Self::Post {
    state
      .into_iter()
      .map(|x| {
        x.into_iter()
          .map(|x| self.0.post(x, char.clone()))
          .collect()
      })
      .collect()
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state
      .iter()
      .any(|x| x.into_iter().all(|x| self.0.accept(x)))
  }
  fn enter(&self, state: Self::Pre) -> Self::State {
    (
      state.clone(),
      [(BTreeSet::new(), self.0.enter(state))]
        .into_iter()
        .collect(),
    )
  }
  fn exit(&self, state: Self::State) -> Self::Post {
    state
      .1
      .into_iter()
      .map(|mut x| {
        x.0.insert(self.0.exit(x.1));
        x.0
      })
      .collect()
  }
}

#[derive(Debug, Clone, Copy)]
struct LookAhead<A>(A);

impl<X: Clone, A: Regex<X>> Regex<X> for LookAhead<A>
where
  A::Pre: Clone,
  A::State: Clone + Ord,
  A::Post: Ord,
{
  type Pre = A::Pre;
  type State = Option<A::Pre>;
  type Post = Option<(BTreeSet<A::State>, BTreeSet<A::Post>)>;
  fn initial(&self) -> Self::Pre {
    self.0.initial()
  }
  fn pre(&self, state: Self::Pre, char: X) -> Self::Pre {
    self.0.pre(state, char)
  }
  fn next(&self, _: Self::State, _: X) -> Self::State {
    None
  }
  fn post(&self, state: Self::Post, char: X) -> Self::Post {
    let state = state?;
    let s = state
      .0
      .into_iter()
      .map(|x| self.0.next(x, char.clone()))
      .collect::<BTreeSet<_>>();
    let e = state
      .1
      .into_iter()
      .map(|x| self.0.post(x, char.clone()))
      .chain(s.iter().map(|x| self.0.exit(x.clone())))
      .collect();
    Some((s, e))
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state
      .as_ref()
      .map_or(false, |state| state.1.iter().any(|x| self.0.accept(x)))
  }
  fn enter(&self, state: Self::Pre) -> Self::State {
    Some(state)
  }
  fn exit(&self, state: Self::State) -> Self::Post {
    let state = self.0.enter(state?);
    Some((
      [state.clone()].into_iter().collect(),
      [self.0.exit(state)].into_iter().collect(),
    ))
  }
}

#[derive(Debug, Clone, Copy)]
struct ToDfa<A>(A);

impl<X: Clone, A: Regex<X>> Dfa<X> for ToDfa<A>
where
  A::Pre: Clone,
  A::State: Ord + Clone,
  A::Post: Ord + Clone,
{
  type State = (A::Pre, BTreeSet<A::State>, BTreeSet<A::Post>);
  fn initial(&self) -> Self::State {
    let i = self.0.initial();
    let s = self.0.enter(i.clone());
    let e = self.0.exit(s.clone());
    (i, [s].into_iter().collect(), [e].into_iter().collect())
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    let i = self.0.pre(state.0, char.clone());
    let s = state
      .1
      .into_iter()
      .map(|x| self.0.next(x, char.clone()))
      .chain([self.0.enter(i.clone())])
      .collect::<BTreeSet<_>>();
    let e = state
      .2
      .into_iter()
      .map(|x| self.0.post(x, char.clone()))
      .chain(s.iter().map(|x| self.0.exit(x.clone())))
      .collect();
    (i, s, e)
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.2.iter().any(|x| self.0.accept(x))
  }
}

#[derive(Debug, Clone, Copy)]
struct FromDfa<A>(A);

impl<X, A: Dfa<X>> Dfa<X> for FromDfa<A> {
  type State = A::State;
  fn initial(&self) -> Self::State {
    self.0.initial()
  }
  fn accept(&self, state: &Self::State) -> bool {
    self.0.accept(state)
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    self.0.next(state, char)
  }
}

impl<X, A: Dfa<X>> Regex<X> for FromDfa<A> {
  type Pre = ();
  type State = A::State;
  type Post = bool;
  fn initial(&self) -> Self::Pre {
    ()
  }
  fn pre(&self, _: Self::Pre, _: X) -> Self::Pre {
    ()
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    self.0.next(state, char)
  }
  fn post(&self, state: Self::Post, _: X) -> Self::Post {
    state
  }
  fn accept(&self, state: &Self::Post) -> bool {
    *state
  }
  fn enter(&self, _: Self::Pre) -> Self::State {
    self.0.initial()
  }
  fn exit(&self, state: Self::State) -> Self::Post {
    self.0.accept(&state)
  }
}
