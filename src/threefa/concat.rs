use crate::*;

impl<X: Clone, A: ThreeFA<X>, B: ThreeFA<X>> ThreeFA<X> for Concat<A, B>
where
  B::Pre: Clone,
  A::Active: Ord + Clone,
  A::Post: Ord,
  B::Post: Ord,
  B::Active: Ord,
{
  type Pre = (A::Pre, B::Pre);
  type Active = (Option<(A::Active, B::Pre)>, BTreeSet<(A::Post, B::Active)>);
  type Post = BTreeSet<(A::Post, B::Post)>;
  fn initial(&self) -> Self::Pre {
    (self.0.initial(), self.1.initial())
  }
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    Some((
      self.0.step_pre(state.0, char.clone())?,
      self.1.step_pre(state.1, char.clone())?,
    ))
  }
  fn step_active(&self, (x, state): Self::Active, char: X) -> Option<Self::Active> {
    let x = x.and_then(|x| {
      Some((
        self.0.step_active(x.0, char.clone())?,
        self.1.step_pre(x.1, char.clone())?,
      ))
    });
    let state = (
      x.clone(),
      state
        .into_iter()
        .filter_map(|x| {
          Some((
            self.0.step_post(x.0, char.clone())?,
            self.1.step_active(x.1, char.clone())?,
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
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let x = state
      .into_iter()
      .filter_map(|x| {
        Some((
          self.0.step_post(x.0, char.clone())?,
          self.1.step_post(x.1, char.clone())?,
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
  fn enter(&self, state: Self::Pre) -> Option<Self::Active> {
    let state = (self.0.enter(state.0)?, state.1);
    Some((
      Some(state.clone()),
      (|| Some((self.0.exit(state.0)?, self.1.enter(state.1)?)))()
        .into_iter()
        .collect(),
    ))
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
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
