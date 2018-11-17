use super::nfa::{Nfa, NfaTable};
use std::{
  collections::{
    hash_map::Entry as HashEntry, BTreeSet, HashMap, HashSet, VecDeque,
  },
  fmt::{self, Debug, Write},
  hash::Hash,
};

pub type DfaTable<T, S> = HashMap<S, HashMap<T, S>>;

pub struct Dfa<T, S> {
  states: DfaTable<T, S>,
  start: S,
  accept: HashSet<S>,
}

impl<T: Hash + Eq + Debug, S: Hash + Eq + Debug> Dfa<T, S> {
  pub fn dot(&self) -> Result<String, fmt::Error> {
    let mut s = String::new();

    s.write_str("digraph{edge[arrowhead=normal,arrowtail=dot];")?;

    for state in self.states.keys() {
      write!(s, "{:?}", format!("{:?}", state))?;

      if self.accept.contains(state) {
        s.write_str("[peripheries=2];")?;
      } else {
        s.write_str(";")?;
      }
    }

    for (state, outs) in &self.states {
      let state_str = format!("{:?}", state);

      for (by, to) in outs {
        let by_str = format!("{:?}", by);

        write!(
          s,
          "{:?}->{:?}[label={:?}];",
          state_str,
          format!("{:?}", to),
          by_str
        )?;
      }
    }

    s.write_str("}")?;

    Ok(s)
  }
}

impl<T: Hash + Clone + Eq, S: Hash + Clone + Ord> From<Nfa<T, S>>
  for Dfa<T, BTreeSet<S>>
{
  fn from(nfa: Nfa<T, S>) -> Self {
    let (nstates, head, tail) = nfa.into_parts();

    fn collect_states<'a, T: Hash + Eq, S: Hash + Clone + Ord>(
      states: &NfaTable<T, S>,
      state: &S,
      set: &mut BTreeSet<S>,
    ) {
      let mut q = VecDeque::new();

      q.push_back(state);

      loop {
        let curr = match q.pop_front() {
          Some(c) => c,
          None => break,
        };

        if set.insert(curr.clone()) {
          if let Some(outs) = states[curr].get(&None) {
            for out in outs {
              q.push_back(out);
            }
          }
        }
      }
    }

    let mut states = DfaTable::new();
    let mut accept = HashSet::new();

    let mut q = VecDeque::new();

    let start = {
      let mut set = BTreeSet::new();
      collect_states(&nstates, &head, &mut set);
      set
    };

    q.push_back(start.clone());

    loop {
      let curr = match q.pop_front() {
        Some(c) => c,
        None => break,
      };

      let outs = match states.entry(curr.clone()) {
        HashEntry::Vacant(v) => v.insert(HashMap::new()),
        HashEntry::Occupied(_) => continue,
      };

      for state in &curr {
        for (by, tos) in nstates[state]
          .iter()
          .filter_map(|(b, t)| b.as_ref().map(|b| (b, t)))
        {
          let new_tos = outs.entry(by.clone()).or_insert(BTreeSet::new());

          for to in tos {
            collect_states(&nstates, to, new_tos);
          }
        }
      }

      let outs = &states[&curr]; // Gotta downgrade the reference to borrow states

      for to in outs.values() {
        if !states.contains_key(&to) {
          q.push_back(to.clone());
        }
      }

      if curr.contains(&tail) {
        accept.insert(curr);
      }
    }

    Self {
      states,
      start,
      accept,
    }
  }
}
