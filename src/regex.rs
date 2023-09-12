#[macro_export]
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

#[derive(Debug, Clone, Copy)]
pub struct Not<A>(pub A);

#[derive(Debug, Clone, Copy)]
pub struct Or<A, B>(pub A, pub B);

#[derive(Debug, Clone, Copy)]
pub struct Iff<A, B>(pub A, pub B);

#[derive(Debug, Clone, Copy)]
pub struct Concat<A, B>(pub A, pub B);

#[derive(Debug, Clone, Copy)]
pub struct Empty;

#[derive(Debug, Clone, Copy)]
pub struct Anything;

#[derive(Debug, Clone, Copy)]
pub struct Dot;

#[derive(Debug, Clone, Copy)]
pub struct Plus<A>(pub A);

#[derive(Debug, Clone, Copy)]
pub struct Start;

#[derive(Debug, Clone, Copy)]
pub struct End;

#[derive(Debug, Clone, Copy)]
pub struct LookAhead<A>(pub A);

#[derive(Debug, Clone, Copy)]
pub struct ToDfa<A>(pub A);

#[derive(Debug, Clone, Copy)]
pub struct LookBehind<A>(pub A);

#[derive(Debug, Clone, Copy)]
pub struct FromDfa<A>(pub A);
