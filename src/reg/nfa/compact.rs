use super::Nfa;
use std::collections::{HashMap, HashSet};

pub struct CompactNfa<T, S> {
  states: HashMap<S, HashMap<T, HashSet<S>>>,
  head: S,
  tail: S,
}

impl<T, S> From<Nfa<T, S>> for CompactNfa<T, S> {
  fn from(nfa: Nfa<T, S>) -> Self { unimplemented!() }
}
