use super::prelude::*;
use std::{collections::HashMap, hash::Hash};

struct Nfa<T, S> {
  states: HashMap<S, NfaState<T, S>>,
}

struct NfaState<T, S> {
  delta: HashMap<T, S>,
}

impl<T, S: Hash + Eq> Nfa<T, S> {
  pub fn from_re(re: &Re<T>) -> Self {
    Self {
      states: HashMap::new(),
    }
  }
}
