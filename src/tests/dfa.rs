use crate::reg::{nfa::Nfa, re::Re};
use std::{fs::File, io::prelude::*};

fn gen_re_1() -> Re<char> {
  Re::alt(
    'a'.into(),
    Re::cat(Re::cat('a'.into(), 'b'.into()).star(), 'c'.into()),
  )
}

fn gen_nfa_1() -> Nfa<char, usize> { gen_re_1().build_nfa().build() }

#[test]
fn dfa() {
  // Just make sure it doesn't panic. (see comments on the nfa::builder test)
  let _ = gen_nfa_1().build_dfa();
}

#[test]
fn dot() {
  let mut builder_file = File::create("dfa::dot-builder.test.log").unwrap();
  let mut nfa_file = File::create("dfa::dot-nfa.test.log").unwrap();
  let mut dfa_file = File::create("dfa::dot-dfa.test.log").unwrap();

  let re = gen_re_1();

  let builder = re.build_nfa();
  write!(builder_file, "{}", builder.dot().unwrap()).unwrap();

  let nfa = builder.build();
  write!(nfa_file, "{}", nfa.dot().unwrap()).unwrap();

  let dfa = nfa.build_dfa();
  write!(dfa_file, "{}", dfa.dot().unwrap()).unwrap();
}
