#[macro_export]
macro_rules! regex {
    ($($x:tt)*) => {
        ToDfa(_regex!($($x)*))
    };
}

#[macro_export]
macro_rules! dfa {
    ($($x:tt)*) => {
        _regex!($($x)*)
    };
}

#[macro_export]
macro_rules! _regex {
  ( (?= $($x:tt)*) ) => {
    LookAhead(_regex!($($x)*))
  };
  ( (?! $($x:tt)*) ) => {
    LookAhead(Not(_regex!($($x)*)))
  };
  ( (?<= $($x:tt)*) ) => {
    LookBehind(_regex!($($x)*))
  };
  ( (?<! $($x:tt)*) ) => {
    LookBehind(Not(_regex!($($x)*)))
  };
  ( ($($x:tt)*) ) => {
    _regex!($($x)*)
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
    Or(_regex!($x), _regex!($($y)*))
  };
  ( $x:tt ? $($y:tt)* ) => {
    _regex!({Or(FromDfa(Empty), _regex!($x))} $($y)*)
  };
  ( $x:tt * $($y:tt)* ) => {
    _regex!({Or(FromDfa(Empty), Plus(_regex!($x)))} $($y)*)
  };
  ( $x:tt + $($y:tt)* ) => {
    _regex!({Plus(_regex!($x))} $($y)*)
  };
  ( $x:tt $($y:tt)+ ) => {
    Concat(_regex!($x), _regex!($($y)+))
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
