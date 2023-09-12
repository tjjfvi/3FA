use crate::*;

pub trait Finite {}

impl Finite for usize {}
impl Finite for bool {}

impl<T: Finite> Finite for Option<T> {}
impl<T: Finite> Finite for BTreeSet<T> {}

impl Finite for () {}
impl<T: Finite> Finite for (T,) {}
impl<T: Finite, U: Finite> Finite for (T, U) {}
impl<T: Finite, U: Finite, V: Finite> Finite for (T, U, V) {}
