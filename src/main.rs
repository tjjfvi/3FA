use std::{
  collections::{BTreeMap, BTreeSet},
  fmt::Debug,
};

trait Dfa<X> {
  type State;
  fn initial(&self) -> Self::State;
  fn next(&self, state: Self::State, char: X) -> Option<Self::State>;
  fn accept(&self, state: &Self::State) -> bool;
}

impl<'a, X, T: Dfa<X> + ?Sized> Dfa<X> for &'a T {
  type State = T::State;
  fn initial(&self) -> Self::State {
    (*self).initial()
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    (*self).next(state, char)
  }
  fn accept(&self, state: &Self::State) -> bool {
    (*self).accept(state)
  }
}

impl<'a, C: Eq> Dfa<C> for [C] {
  type State = usize;
  fn initial(&self) -> Self::State {
    0
  }
  fn next(&self, state: Self::State, char: C) -> Option<Self::State> {
    if self.get(state) == Some(&char) {
      Some(state + 1)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state == &self.len()
  }
}

impl<'a, C: Eq, const N: usize> Dfa<C> for [C; N] {
  type State = usize;
  fn initial(&self) -> Self::State {
    0
  }
  fn next(&self, state: Self::State, char: C) -> Option<Self::State> {
    if self.get(state) == Some(&char) {
      Some(state + 1)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state == &self.len()
  }
}

#[derive(Debug, Clone, Copy)]
struct Not<A>(A);

impl<X, A: Dfa<X>> Dfa<X> for Not<A> {
  type State = Option<A::State>;
  fn initial(&self) -> Self::State {
    Some(self.0.initial())
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    Some(state.and_then(|state| self.0.next(state, char)))
  }
  fn accept(&self, state: &Self::State) -> bool {
    !state.as_ref().map_or(false, |state| self.0.accept(state))
  }
}

#[derive(Debug, Clone, Copy)]
struct Or<A, B>(A, B);

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for Or<A, B> {
  type State = (Option<A::State>, Option<B::State>);
  fn initial(&self) -> Self::State {
    (Some(self.0.initial()), Some(self.1.initial()))
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    let a = state.0.and_then(|x| self.0.next(x, char.clone()));
    let b = state.1.and_then(|x| self.1.next(x, char));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.0.as_ref().map_or(false, |x| self.0.accept(x))
      || state.1.as_ref().map_or(false, |x| self.1.accept(x))
  }
}

#[derive(Debug, Clone, Copy)]
struct Iff<A, B>(A, B);

impl<X: Clone, A: Dfa<X>, B: Dfa<X>> Dfa<X> for Iff<A, B> {
  type State = (Option<A::State>, Option<B::State>);
  fn initial(&self) -> Self::State {
    (Some(self.0.initial()), Some(self.1.initial()))
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    Some((
      state.0.and_then(|x| self.0.next(x, char.clone())),
      state.1.and_then(|x| self.1.next(x, char)),
    ))
  }
  fn accept(&self, state: &Self::State) -> bool {
    state.0.as_ref().map_or(false, |x| self.0.accept(x))
      == state.1.as_ref().map_or(false, |x| self.1.accept(x))
  }
}

#[derive(Debug, Clone, Copy)]
struct Concat<A, B>(A, B);

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

#[derive(Debug, Clone, Copy)]
struct Empty;

impl<X> Dfa<X> for Empty {
  type State = ();
  fn initial(&self) -> Self::State {}
  fn next(&self, _: Self::State, _: X) -> Option<Self::State> {
    None
  }
  fn accept(&self, _: &Self::State) -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy)]
struct Anything;

impl<X> Dfa<X> for Anything {
  type State = ();
  fn initial(&self) -> Self::State {
    ()
  }
  fn next(&self, _: Self::State, _: X) -> Option<Self::State> {
    Some(())
  }
  fn accept(&self, _: &Self::State) -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy)]
struct Dot;

impl<X> Dfa<X> for Dot {
  type State = bool;
  fn initial(&self) -> Self::State {
    false
  }
  fn next(&self, state: Self::State, _: X) -> Option<Self::State> {
    match state {
      false => Some(true),
      _ => None,
    }
  }
  fn accept(&self, state: &Self::State) -> bool {
    *state
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

fn is_empty<X: Clone, D: Dfa<X>>(
  dfa: D,
  alphabet: impl Clone + Iterator<Item = X>,
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

fn dfa_equal<X: Clone, A: Dfa<X>, B: Dfa<X>>(
  a: A,
  b: B,
  alphabet: impl Clone + Iterator<Item = X>,
) -> Result<(), Vec<X>>
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
  ( ($($x:tt)*) ) => {
    regex!($($x)*)
  };
  ( {$x:expr} ) => {
    $x
  };
  ( $x:ident ) => {
    FromDfa(stringify!($x).as_bytes())
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
  ( .. ) => {
    Concat(FromDfa(Dot), FromDfa(Dot))
  };
  ( ... ) => {
    Concat(FromDfa(Dot), Concat(FromDfa(Dot), FromDfa(Dot)))
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
    state = match dfa.next(state, char) {
      Some(x) => x,
      None => return false,
    }
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

trait Regex<X> {
  type Pre;
  type State;
  type Post;
  fn initial(&self) -> Self::Pre;
  fn pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre>;
  fn next(&self, state: Self::State, char: X) -> Option<Self::State>;
  fn post(&self, state: Self::Post, char: X) -> Option<Self::Post>;
  fn accept(&self, state: &Self::Post) -> bool;
  fn enter(&self, state: Self::Pre) -> Option<Self::State>;
  fn exit(&self, state: Self::State) -> Option<Self::Post>;
}

#[derive(Debug, Clone, Copy)]
struct Start;

impl<X> Regex<X> for Start {
  type Pre = ();
  type State = ();
  type Post = ();
  fn initial(&self) -> Self::Pre {
    ()
  }
  fn pre(&self, _: Self::Pre, _: X) -> Option<Self::Pre> {
    None
  }
  fn next(&self, _: Self::State, _: X) -> Option<Self::State> {
    None
  }
  fn post(&self, _: Self::Post, _: X) -> Option<Self::Post> {
    Some(())
  }
  fn accept(&self, _: &Self::Post) -> bool {
    true
  }
  fn enter(&self, _: Self::Pre) -> Option<Self::State> {
    Some(())
  }
  fn exit(&self, _: Self::State) -> Option<Self::Post> {
    Some(())
  }
}

#[derive(Debug, Clone, Copy)]
struct End;

impl<X> Regex<X> for End {
  type Pre = ();
  type State = ();
  type Post = ();
  fn initial(&self) -> Self::Pre {
    ()
  }
  fn pre(&self, _: Self::Pre, _: X) -> Option<Self::Pre> {
    Some(())
  }
  fn next(&self, _: Self::State, _: X) -> Option<Self::State> {
    None
  }
  fn post(&self, _: Self::Post, _: X) -> Option<Self::Post> {
    None
  }
  fn accept(&self, _: &Self::Post) -> bool {
    true
  }
  fn enter(&self, _: Self::Pre) -> Option<Self::State> {
    Some(())
  }
  fn exit(&self, _: Self::State) -> Option<Self::Post> {
    Some(())
  }
}

impl<X: Clone, A: Regex<X>> Regex<X> for Not<A> {
  type Pre = Option<A::Pre>;
  type State = Option<A::State>;
  type Post = Option<A::Post>;
  fn initial(&self) -> Self::Pre {
    Some(self.0.initial())
  }
  fn pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    Some(state.and_then(|x| self.0.pre(x, char)))
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    Some(state.and_then(|x| self.0.next(x, char)))
  }
  fn post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    Some(state.and_then(|x| self.0.post(x, char)))
  }
  fn accept(&self, state: &Self::Post) -> bool {
    !state.as_ref().map_or(false, |x| self.0.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::State> {
    Some(state.and_then(|x| self.0.enter(x)))
  }
  fn exit(&self, state: Self::State) -> Option<Self::Post> {
    Some(state.and_then(|x| self.0.exit(x)))
  }
}

impl<X: Clone, A: Regex<X>, B: Regex<X>> Regex<X> for Or<A, B> {
  type Pre = (Option<A::Pre>, Option<B::Pre>);
  type State = (Option<A::State>, Option<B::State>);
  type Post = (Option<A::Post>, Option<B::Post>);
  fn initial(&self) -> Self::Pre {
    (Some(self.0.initial()), Some(self.1.initial()))
  }
  fn pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    let a = state.0.and_then(|x| self.0.pre(x, char.clone()));
    let b = state.1.and_then(|x| self.1.pre(x, char));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    let a = state.0.and_then(|x| self.0.next(x, char.clone()));
    let b = state.1.and_then(|x| self.1.next(x, char));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let a = state.0.and_then(|x| self.0.post(x, char.clone()));
    let b = state.1.and_then(|x| self.1.post(x, char));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state.0.as_ref().map_or(false, |x| self.0.accept(x))
      || state.1.as_ref().map_or(false, |x| self.1.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::State> {
    let a = state.0.and_then(|x| self.0.enter(x));
    let b = state.1.and_then(|x| self.1.enter(x));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
  }
  fn exit(&self, state: Self::State) -> Option<Self::Post> {
    let a = state.0.and_then(|x| self.0.exit(x));
    let b = state.1.and_then(|x| self.1.exit(x));
    if a.is_some() || b.is_some() {
      Some((a, b))
    } else {
      None
    }
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
  type State = (Option<(A::State, B::Pre)>, BTreeSet<(A::Post, B::State)>);
  type Post = BTreeSet<(A::Post, B::Post)>;
  fn initial(&self) -> Self::Pre {
    (self.0.initial(), self.1.initial())
  }
  fn pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    Some((
      self.0.pre(state.0, char.clone())?,
      self.1.pre(state.1, char.clone())?,
    ))
  }
  fn next(&self, (x, state): Self::State, char: X) -> Option<Self::State> {
    let x = x.and_then(|x| {
      Some((
        self.0.next(x.0, char.clone())?,
        self.1.pre(x.1, char.clone())?,
      ))
    });
    let state = (
      x.clone(),
      state
        .into_iter()
        .filter_map(|x| {
          Some((
            self.0.post(x.0, char.clone())?,
            self.1.next(x.1, char.clone())?,
          ))
        })
        .chain(x.and_then(|x| Some((self.0.exit(x.0)?, self.1.enter(x.1)?))))
        .collect::<BTreeSet<_>>(),
    );
    if state.0.is_some() || !state.1.is_empty() {
      Some(state)
    } else {
      None
    }
  }
  fn post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let x = state
      .into_iter()
      .filter_map(|x| {
        Some((
          self.0.post(x.0, char.clone())?,
          self.1.post(x.1, char.clone())?,
        ))
      })
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state
      .iter()
      .any(|x| self.0.accept(&x.0) && self.1.accept(&x.1))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::State> {
    let state = (self.0.enter(state.0)?, state.1);
    Some((
      Some(state.clone()),
      (|| Some((self.0.exit(state.0)?, self.1.enter(state.1)?)))()
        .into_iter()
        .collect(),
    ))
  }
  fn exit(&self, state: Self::State) -> Option<Self::Post> {
    let x = state
      .1
      .into_iter()
      .filter_map(|x| Some((x.0, self.1.exit(x.1)?)))
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
  }
}

impl<X: Clone, A: Regex<X>> Regex<X> for Plus<A>
where
  A::Pre: Clone,
  A::State: Ord + Clone,
  A::Post: Ord + Clone,
{
  type Pre = A::Pre;
  type State = (Option<A::Pre>, BTreeSet<(BTreeSet<A::Post>, A::State)>);
  type Post = BTreeSet<BTreeSet<A::Post>>;
  fn initial(&self) -> Self::Pre {
    self.0.initial()
  }
  fn pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    self.0.pre(state, char)
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    let a = state.0.and_then(|x| self.0.pre(x, char.clone()));
    let x = (
      a.clone(),
      state
        .1
        .into_iter()
        .filter_map(|x| {
          Some((
            x.0
              .into_iter()
              .map(|x| self.0.post(x, char.clone()))
              .collect::<Option<BTreeSet<_>>>()?,
            self.0.next(x.1, char.clone())?,
          ))
        })
        .flat_map(|mut x| {
          [x.clone()].into_iter().chain(a.clone().and_then(|a| {
            x.0.insert(self.0.exit(x.1)?);
            Some((x.0, self.0.enter(a)?))
          }))
        })
        .collect::<BTreeSet<_>>(),
    );
    if x.0.is_some() || !x.1.is_empty() {
      Some(x)
    } else {
      None
    }
  }
  fn post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let x = state
      .into_iter()
      .filter_map(|x| {
        x.into_iter()
          .map(|x| self.0.post(x, char.clone()))
          .collect::<Option<BTreeSet<_>>>()
      })
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state
      .iter()
      .any(|x| x.into_iter().all(|x| self.0.accept(x)))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::State> {
    Some((
      Some(state.clone()),
      [(BTreeSet::new(), self.0.enter(state)?)]
        .into_iter()
        .collect(),
    ))
  }
  fn exit(&self, state: Self::State) -> Option<Self::Post> {
    let x = state
      .1
      .into_iter()
      .filter_map(|mut x| {
        x.0.insert(self.0.exit(x.1)?);
        Some(x.0)
      })
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
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
  type State = A::Pre;
  type Post = (BTreeSet<A::State>, BTreeSet<A::Post>);
  fn initial(&self) -> Self::Pre {
    self.0.initial()
  }
  fn pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    self.0.pre(state, char)
  }
  fn next(&self, _: Self::State, _: X) -> Option<Self::State> {
    None
  }
  fn post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let s = state
      .0
      .into_iter()
      .filter_map(|x| self.0.next(x, char.clone()))
      .collect::<BTreeSet<_>>();
    let e = state
      .1
      .into_iter()
      .filter_map(|x| self.0.post(x, char.clone()))
      .chain(s.iter().filter_map(|x| self.0.exit(x.clone())))
      .collect::<BTreeSet<_>>();
    if !s.is_empty() || !e.is_empty() {
      Some((s, e))
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state.1.iter().any(|x| self.0.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::State> {
    Some(state)
  }
  fn exit(&self, state: Self::State) -> Option<Self::Post> {
    let state = self.0.enter(state)?;
    Some((
      [state.clone()].into_iter().collect(),
      self.0.exit(state).into_iter().collect(),
    ))
  }
}

#[derive(Debug, Clone, Copy)]
struct LookBehind<A>(A);

impl<X: Clone, A: Regex<X>> Regex<X> for LookBehind<A>
where
  A::Pre: Clone + Ord,
  A::State: Clone + Ord,
  A::Post: Ord,
{
  type Pre = (BTreeSet<A::Pre>, BTreeSet<A::State>);
  type State = (BTreeSet<A::Pre>, BTreeSet<A::State>);
  type Post = BTreeSet<A::Post>;
  fn initial(&self) -> Self::Pre {
    (
      [self.0.initial()].into_iter().collect(),
      self.0.enter(self.0.initial()).into_iter().collect(),
    )
  }
  fn pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    let s = state
      .0
      .into_iter()
      .filter_map(|x| self.0.pre(x, char.clone()))
      .collect::<BTreeSet<_>>();
    let e = state
      .1
      .into_iter()
      .filter_map(|x| self.0.next(x, char.clone()))
      .chain(s.iter().filter_map(|x| self.0.enter(x.clone())))
      .collect::<BTreeSet<_>>();
    if !s.is_empty() || !e.is_empty() {
      Some((s, e))
    } else {
      None
    }
  }
  fn next(&self, _: Self::State, _: X) -> Option<Self::State> {
    None
  }
  fn post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let x = state
      .into_iter()
      .filter_map(|x| self.0.post(x, char.clone()))
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
  }
  fn accept(&self, state: &Self::Post) -> bool {
    state.iter().any(|x| self.0.accept(x))
  }
  fn enter(&self, state: Self::Pre) -> Option<Self::State> {
    Some(state)
  }
  fn exit(&self, state: Self::State) -> Option<Self::Post> {
    let x = state
      .1
      .into_iter()
      .filter_map(|x| self.0.exit(x))
      .collect::<BTreeSet<_>>();
    if !x.is_empty() {
      Some(x)
    } else {
      None
    }
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
  type State = (Option<A::Pre>, BTreeSet<A::State>, BTreeSet<A::Post>);
  fn initial(&self) -> Self::State {
    let i = self.0.initial();
    let s = self.0.enter(i.clone());
    let e = s.clone().and_then(|x| self.0.exit(x));
    (Some(i), s.into_iter().collect(), e.into_iter().collect())
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    let i = state.0.and_then(|x| self.0.pre(x, char.clone()));
    let s = state
      .1
      .into_iter()
      .filter_map(|x| self.0.next(x, char.clone()))
      .chain(i.clone().and_then(|x| self.0.enter(x)))
      .collect::<BTreeSet<_>>();
    let e = state
      .2
      .into_iter()
      .filter_map(|x| self.0.post(x, char.clone()))
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
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    self.0.next(state, char)
  }
}

impl<X, A: Dfa<X>> Regex<X> for FromDfa<A> {
  type Pre = ();
  type State = A::State;
  type Post = ();
  fn initial(&self) -> Self::Pre {
    ()
  }
  fn pre(&self, _: Self::Pre, _: X) -> Option<Self::Pre> {
    Some(())
  }
  fn next(&self, state: Self::State, char: X) -> Option<Self::State> {
    self.0.next(state, char)
  }
  fn post(&self, _: Self::Post, _: X) -> Option<Self::Post> {
    Some(())
  }
  fn accept(&self, _: &Self::Post) -> bool {
    true
  }
  fn enter(&self, _: Self::Pre) -> Option<Self::State> {
    Some(self.0.initial())
  }
  fn exit(&self, state: Self::State) -> Option<Self::Post> {
    if self.0.accept(&state) {
      Some(())
    } else {
      None
    }
  }
}
