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

impl<T, S> Nfa<T, S> {
  // returns (states, head, tail)
  pub fn into_parts(
    self,
  ) -> (HashMap<S, HashMap<Option<T>, HashSet<S>>>, S, S) {
    (self.states, self.head, self.tail)
  }
}

impl<T: Hash + Eq> From<NfaBuilder<T>> for Nfa<T, usize> {
  fn from(builder: NfaBuilder<T>) -> Self {
    let (nodes, head, tail) = builder.into_parts();

    let mut node_ids: HashMap<NfaNodeRef<T>, usize> = HashMap::new();
    let mut states = HashMap::new();

    // TODO: don't create IDs for unused nodes
    for node in &nodes {
      let id = node_ids.len();
      node_ids.insert(node.into(), id);
      states.insert(id, HashMap::new());
    }

    for node in &nodes {
      let id = node_ids[&node.into()];
      let outs = node.take_outs();

      for (by, tos) in outs {
        let set = states
          .get_mut(&id)
          .unwrap()
          .entry(by)
          .or_insert(HashSet::new());

        for to in tos {
          set.insert(node_ids[&to]);
        }
      }
    }

    Self {
      states,
      head: node_ids[&head],
      tail: node_ids[&tail],
    }
  }
}

impl<T: Hash + Eq> From<Re<T>> for Nfa<T, usize> {
  #[inline]
  fn from(re: Re<T>) -> Self { NfaBuilder::from(re).into() }
}
