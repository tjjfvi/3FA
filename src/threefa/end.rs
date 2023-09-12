use crate::*;

impl<X> ThreeFA<X> for End {
  type Pre = ();
  type Active = ();
  type Post = ();
  fn initial(&self) -> Self::Pre {
    ()
  }
  fn step_pre(&self, _: Self::Pre, _: X) -> Option<Self::Pre> {
    Some(())
  }
  fn step_active(&self, _: Self::Active, _: X) -> Option<Self::Active> {
    None
  }
  fn step_post(&self, _: Self::Post, _: X) -> Option<Self::Post> {
    None
  }
  fn accept(&self, _: &Self::Post) -> bool {
    true
  }
  fn enter(&self, _: Self::Pre) -> Option<Self::Active> {
    Some(())
  }
  fn exit(&self, _: Self::Active) -> Option<Self::Post> {
    Some(())
  }
}
