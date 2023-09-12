use crate::*;

impl<X: Clone, A: ThreeFA<X>> ThreeFA<X> for Plus<A>
where
  A::Pre: Clone,
  A::Active: Ord + Clone,
  A::Post: Ord + Clone,
{
  type Pre = A::Pre;
  type Active = (Option<A::Pre>, BTreeSet<(BTreeSet<A::Post>, A::Active)>);
  type Post = BTreeSet<BTreeSet<A::Post>>;
  fn initial(&self) -> Self::Pre {
    self.0.initial()
  }
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre> {
    self.0.step_pre(state, char)
  }
  fn step_active(&self, state: Self::Active, char: X) -> Option<Self::Active> {
    let a = state.0.and_then(|x| self.0.step_pre(x, char.clone()));
    let x = (
      a.clone(),
      state
        .1
        .into_iter()
        .filter_map(|x| {
          Some((
            x.0
              .into_iter()
              .map(|x| self.0.step_post(x, char.clone()))
              .collect::<Option<BTreeSet<_>>>()?,
            self.0.step_active(x.1, char.clone())?,
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
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post> {
    let x = state
      .into_iter()
      .filter_map(|x| {
        x.into_iter()
          .map(|x| self.0.step_post(x, char.clone()))
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
  fn enter(&self, state: Self::Pre) -> Option<Self::Active> {
    Some((
      Some(state.clone()),
      [(BTreeSet::new(), self.0.enter(state)?)]
        .into_iter()
        .collect(),
    ))
  }
  fn exit(&self, state: Self::Active) -> Option<Self::Post> {
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
