use crate::*;

mod and;
mod concat;
mod end;
mod from_dfa;
mod lookahead;
mod lookbehind;
mod not;
mod or;
mod plus;
mod start;
mod to_dfa;

pub trait ThreeFA<X> {
  type Pre: Finite;
  type Active: Finite;
  type Post: Finite;
  fn initial(&self) -> Self::Pre;
  fn step_pre(&self, state: Self::Pre, char: X) -> Option<Self::Pre>;
  fn step_active(&self, state: Self::Active, char: X) -> Option<Self::Active>;
  fn step_post(&self, state: Self::Post, char: X) -> Option<Self::Post>;
  fn accept(&self, state: &Self::Post) -> bool;
  fn enter(&self, state: Self::Pre) -> Option<Self::Active>;
  fn exit(&self, state: Self::Active) -> Option<Self::Post>;
}
