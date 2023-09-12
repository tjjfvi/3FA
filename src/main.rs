use std::{
  collections::{BTreeSet, HashSet},
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

fn is_empty<X, D: Dfa<X>>(dfa: D, alphabet: impl Clone + Iterator<Item = X>) -> bool
where
  D::State: Clone + Ord,
{
  let mut visited = BTreeSet::new();
  let mut next = BTreeSet::new();
  next.insert(dfa.initial());
  while !next.is_empty() {
    for state in std::mem::take(&mut next) {
      if dfa.accept(&state) {
        return false;
      }
      for char in alphabet.clone() {
        let state = dfa.next(state.clone(), char);
        if visited.insert(state.clone()) {
          next.insert(state);
        }
      }
    }
  }
  true
}

fn dfa_equal<X: Clone, A: Dfa<X>, B: Dfa<X>>(
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
  ( (?= $x:expr) ) => {
    LookAhead(regex!($x))
  };
  ( (?! $x:expr) ) => {
    LookAhead(Not(regex!($x)))
  };
  ( (?<= $x:expr) ) => {
    LookBehind(regex!($x))
  };
  ( (?<! $x:expr) ) => {
    LookBehind(Not(regex!($x)))
  };
  ( ($x:expr) ) => {
    regex!($x)
  };
  ( {$x:expr} ) => {
    $x
  };
  ( $x:literal ) => {
    $x
  };
  ( . ) => {
    Dot
  };
  ( $x:tt | $($y:tt)* ) => {
    Or(regex!($x), regex!($($y)*))
  };
  ( $x:tt ? $($y:tt)* ) => {
    regex!({Or(Empty, regex!($x))} $($y)*)
  };
  ( $x:tt * $($y:tt)* ) => {
    regex!({Or(Empty, Plus(regex!($x)))} $($y)*)
  };
  ( $x:tt + $($y:tt)* ) => {
    regex!({Plus(regex!($x))} $($y)*)
  };
  ( $x:tt $y:tt $($z:tt)* ) => {
    regex!({Concat(regex!($x), regex!($y))} $($z)*)
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
}

trait Regex<X> {
  type State;
  fn initial(&self) -> Self::State;
  fn next(&self, state: Self::State, char: X) -> Self::State;
  fn accept(&self, state: &Self::State) -> bool;
  fn enter(&self, state: Self::State) -> Self::State;
  fn exit(&self, state: Self::State) -> Self::State;
}

struct Start;

enum StartState {
  Start,
  Entered,
  Exited,
  Fail,
}

impl<X> Regex<X> for Start {
  type State = StartState;
  fn initial(&self) -> Self::State {
    StartState::Start
  }
  fn next(&self, state: Self::State, _: X) -> Self::State {
    match state {
      StartState::Exited => StartState::Exited,
      _ => StartState::Fail,
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    match state {
      StartState::Entered => true,
      _ => false,
    }
  }
  fn enter(&self, state: Self::State) -> Self::State {
    match state {
      StartState::Start => StartState::Entered,
      _ => StartState::Fail,
    }
  }
  fn exit(&self, state: Self::State) -> Self::State {
    match state {
      StartState::Entered => StartState::Exited,
      _ => StartState::Fail,
    }
  }
}

struct End;

enum EndState {
  Initial,
  Entered,
  Exited,
  Fail,
}

impl<X> Regex<X> for End {
  type State = EndState;
  fn initial(&self) -> Self::State {
    EndState::Initial
  }
  fn next(&self, state: Self::State, _: X) -> Self::State {
    match state {
      EndState::Initial => EndState::Initial,
      _ => EndState::Fail,
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    match state {
      EndState::Exited => true,
      _ => false,
    }
  }
  fn enter(&self, state: Self::State) -> Self::State {
    match state {
      EndState::Initial => EndState::Entered,
      _ => EndState::Fail,
    }
  }
  fn exit(&self, state: Self::State) -> Self::State {
    match state {
      EndState::Entered => EndState::Exited,
      _ => EndState::Fail,
    }
  }
}

impl<X: Clone, A: Regex<X>, B: Regex<X>> Regex<X> for Or<A, B> {
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
  fn enter(&self, state: Self::State) -> Self::State {
    (self.0.enter(state.0), self.1.enter(state.1))
  }
  fn exit(&self, state: Self::State) -> Self::State {
    (self.0.exit(state.0), self.1.exit(state.1))
  }
}

enum ConcatState<A, B> {
  Initial((A, B)),
  Entered((A, B), BTreeSet<(A, B)>),
  Exited(BTreeSet<(A, B)>),
  Fail,
}

impl<X: Clone, A: Regex<X>, B: Regex<X>> Regex<X> for Concat<A, B>
where
  A::State: Ord + Clone,
  B::State: Ord + Clone,
{
  type State = ConcatState<A::State, B::State>;
  fn initial(&self) -> Self::State {
    ConcatState::Initial((self.0.initial(), self.1.initial()))
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    let next_pair = |x: (A::State, B::State)| {
      (
        self.0.next(x.0, char.clone()),
        self.1.next(x.1, char.clone()),
      )
    };
    match state {
      ConcatState::Initial(x) => ConcatState::Initial(next_pair(x)),
      ConcatState::Entered(x, state) => {
        let x = next_pair(x);
        ConcatState::Entered(
          x.clone(),
          state
            .into_iter()
            .map(next_pair)
            .chain([(self.0.exit(x.0), self.1.enter(x.1))])
            .collect(),
        )
      }
      ConcatState::Exited(state) => ConcatState::Exited(state.into_iter().map(next_pair).collect()),
      ConcatState::Fail => ConcatState::Fail,
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    match state {
      ConcatState::Exited(state) => state
        .iter()
        .any(|x| self.0.accept(&x.0) && self.1.accept(&x.1)),
      _ => false,
    }
  }
  fn enter(&self, state: Self::State) -> Self::State {
    match state {
      ConcatState::Initial(mut x) => {
        x.0 = self.0.enter(x.0);
        ConcatState::Entered(
          x.clone(),
          [(self.0.exit(x.0), self.1.enter(x.1))]
            .into_iter()
            .collect(),
        )
      }
      _ => ConcatState::Fail,
    }
  }
  fn exit(&self, state: Self::State) -> Self::State {
    match state {
      ConcatState::Entered(_, state) => {
        ConcatState::Exited(state.into_iter().map(|x| (x.0, self.1.exit(x.1))).collect())
      }
      _ => ConcatState::Fail,
    }
  }
}

enum PlusState<A> {
  Initial(A),
  Entered(A, BTreeSet<(BTreeSet<A>, A)>),
  Exited(BTreeSet<BTreeSet<A>>),
  Fail,
}

impl<X: Clone, A: Regex<X>> Regex<X> for Plus<A>
where
  A::State: Ord + Clone,
{
  type State = PlusState<A::State>;
  fn initial(&self) -> Self::State {
    PlusState::Initial(self.0.initial())
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    match state {
      PlusState::Initial(x) => PlusState::Initial(self.0.next(x, char)),
      PlusState::Entered(a, state) => {
        let a = self.0.next(a, char.clone());
        PlusState::Entered(
          a.clone(),
          state
            .into_iter()
            .map(|x: (BTreeSet<A::State>, A::State)| {
              (
                x.0
                  .into_iter()
                  .map(|x| self.0.next(x, char.clone()))
                  .collect::<BTreeSet<_>>(),
                self.0.next(x.1, char.clone()),
              )
            })
            .flat_map(|mut x| {
              [x.clone(), {
                x.0.insert(x.1);
                (x.0, a.clone())
              }]
            })
            .collect(),
        )
      }
      PlusState::Exited(state) => PlusState::Exited(
        state
          .into_iter()
          .map(|x| {
            x.into_iter()
              .map(|x| self.0.next(x, char.clone()))
              .collect()
          })
          .collect(),
      ),
      PlusState::Fail => PlusState::Fail,
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    match state {
      PlusState::Exited(state) => state
        .iter()
        .any(|x| x.into_iter().all(|x| self.0.accept(x))),
      _ => false,
    }
  }
  fn enter(&self, state: Self::State) -> Self::State {
    match state {
      PlusState::Initial(x) => PlusState::Entered(
        x.clone(),
        [(BTreeSet::new(), self.0.enter(x))].into_iter().collect(),
      ),
      _ => PlusState::Fail,
    }
  }
  fn exit(&self, state: Self::State) -> Self::State {
    match state {
      PlusState::Entered(_, state) => PlusState::Exited(
        state
          .into_iter()
          .map(|mut x| {
            x.0.insert(self.0.exit(x.1));
            x.0
          })
          .collect(),
      ),
      _ => PlusState::Fail,
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct LookAhead<A>(A);

#[derive(Debug, Clone, Copy)]
struct ToDfa<A>(A);

impl<X: Clone, A: Regex<X>> Dfa<X> for ToDfa<A>
where
  A::State: Ord + Clone,
{
  type State = (A::State, BTreeSet<A::State>, BTreeSet<A::State>);
  fn initial(&self) -> Self::State {
    let i = self.0.initial();
    let s = self.0.enter(i.clone());
    let e = self.0.exit(i.clone());
    (i, [s].into_iter().collect(), [e].into_iter().collect())
  }
  fn next(&self, state: Self::State, char: X) -> Self::State {
    let i = self.0.next(state.0, char.clone());
    let s = state
      .1
      .into_iter()
      .map(|x| self.0.next(x, char.clone()))
      .chain([self.0.enter(i.clone())])
      .collect::<BTreeSet<_>>();
    let e = state
      .2
      .into_iter()
      .map(|x| self.0.next(x, char.clone()))
      .chain(s.iter().map(|x| self.0.exit(x.clone())))
      .collect();
    (i, s, e)
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.2.iter().any(|x| self.0.accept(x))
  }
}
