use crate::reg::re::Re;

fn gen_re_1() -> Re<char> {
  Re::alt(
    'a'.into(),
    Re::cat(Re::cat('a'.into(), 'b'.into()).star(), 'c'.into()),
  )
}

#[test]
fn builder() {
  // Just make sure it doesn't panic.
  // I can test properties of the graph with great difficulty, but I'd rather not
  // if it's not necessary
  let _ = gen_re_1().build_nfa();
}

#[test]
fn nfa() {
  // Just make sure it doesn't panic. (see comments on the builder tests)
  let _ = gen_re_1().build_nfa().build();
}
