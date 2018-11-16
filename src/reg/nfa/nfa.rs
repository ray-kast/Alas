use super::{
  super::prelude::*,
  builder::{NfaBuilder, NfaNodeRef},
};
use std::{
  collections::{HashMap, HashSet},
  hash::Hash,
};

pub struct Nfa<T, S> {
  states: HashMap<S, HashMap<Option<T>, HashSet<S>>>,
  head: S,
  tail: S,
}

impl<T: Hash + Eq> From<NfaBuilder<T>> for Nfa<T, usize> {
  fn from(builder: NfaBuilder<T>) -> Self {
    let mut ret = Self {
      states: HashMap::new(),
      head: unimplemented!(),
      tail: unimplemented!(),
    };

    let mut node_ids: HashMap<NfaNodeRef<T>, usize> = HashMap::new();

    // TODO: don't create IDs for unused nodes
    for node in builder.nodes() {
      let id = node_ids.len();
      node_ids.insert(node.into(), id);
      ret.states.insert(id, HashMap::new());
    }

    ret
  }
}

impl<T: Hash + Eq> From<Re<T>> for Nfa<T, usize> {
  #[inline]
  fn from(re: Re<T>) -> Self { NfaBuilder::from(re).into() }
}
