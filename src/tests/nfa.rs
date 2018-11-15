use crate::reg::{
  nfa::{builder::NfaBuilder, Nfa},
  re::Re,
};

fn gen_re_1() -> Re<char> {
  Re::alt(
    'a'.into(),
    Re::cat(Re::cat('a'.into(), 'b'.into()).star(), 'c'.into()),
  )
}

#[test]
fn builder() {
  // TODO: figure out how to test properties of the graph
  let _ = gen_re_1().build_nfa();
}

#[test]
fn nfa() {
  // TODO: figure out how to test properties of the NFA
  let _ = gen_re_1().build_nfa().build();
}

#[test]
fn compact() {
  // TODO: figure out how to test properties of the NFA
  let _ = gen_re_1().build_nfa().build().compact();
}
