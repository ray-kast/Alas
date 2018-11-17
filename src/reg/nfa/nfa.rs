use super::{
  super::{dfa::Dfa, prelude::*},
  builder::{NfaBuilder, NfaNodeRef},
};
use std::{
  borrow::Cow,
  collections::{BTreeSet, HashMap, HashSet},
  fmt::{self, Debug, Write},
  hash::Hash,
};

pub type NfaTable<T, S> = HashMap<S, HashMap<Option<T>, HashSet<S>>>;

pub struct Nfa<T, S> {
  states: NfaTable<T, S>,
  head: S,
  tail: S,
}

impl<T, S> Nfa<T, S> {
  // returns (states, head, tail)
  pub fn into_parts(self) -> (NfaTable<T, S>, S, S) {
    (self.states, self.head, self.tail)
  }
}

impl<T: Hash + Clone + Eq, S: Hash + Clone + Ord> Nfa<T, S> {
  #[inline]
  pub fn build_dfa(self) -> Dfa<T, BTreeSet<S>> { self.into() }
}

impl<T: Hash + Eq + Debug, S: Hash + Eq + Debug> Nfa<T, S> {
  pub fn dot(&self) -> Result<String, fmt::Error> {
    let mut s = String::new();

    s.write_str("digraph{edge[arrowhead=normal,arrowtail=dot];")?;

    for state in self.states.keys() {
      write!(s, "{:?}", format!("{:?}", state))?;

      if state == &self.tail {
        s.write_str("[peripheries=2];")?;
      } else {
        s.write_str(";")?;
      }
    }

    for (state, outs) in &self.states {
      let state_str = format!("{:?}", state);

      for (by, tos) in outs {
        let by_str: Cow<str> = by
          .as_ref()
          .map_or("λ".into(), |b| format!("{:?}", b).into());

        for to in tos {
          write!(
            s,
            "{:?}->{:?}[label={:?}];",
            state_str,
            format!("{:?}", to),
            by_str
          )?;
        }
      }
    }

    s.write_str("}")?;

    Ok(s)
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
