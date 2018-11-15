use crate::reg::re::Re;

#[test]
fn cat_all() {
  assert_eq!(
    Re::cat('a'.into(), Re::cat('b'.into(), 'c'.into())),
    Re::cat_all(vec!['a'.into(), 'b'.into(), 'c'.into()])
  );
}

#[test]
#[should_panic(expected = "empty iterator")]
fn cat_all_bad() { Re::<char>::cat_all(Vec::new()); }

#[test]
fn alt_all() {
  assert_eq!(
    Re::alt('a'.into(), Re::alt('b'.into(), 'c'.into())),
    Re::alt_all(vec!['a'.into(), 'b'.into(), 'c'.into()]),
  )
}

#[test]
#[should_panic(expected = "empty iterator")]
fn alt_all_bad() { Re::<char>::alt_all(Vec::new()); }

#[test]
fn dup() {
  let re: Re<_> = 'a'.into();

  assert_eq!(
    Re::cat_all(vec![re.clone(), re.clone(), re.clone()]),
    re.dup(3),
  );
}

#[test]
fn rep() {
  let re: Re<_> = 'a'.into();

  assert_eq!(re.clone().star(), re.clone().rep(0, None));
  assert_eq!(re.clone().plus(), re.clone().rep(1, None));
  assert_eq!(re.clone().opt(), re.clone().rep(0, 1));
  assert_eq!(re.dup(3), re.clone().rep(3, 3));

  assert_eq!(
    Re::cat_all(vec![re.clone(), re.clone(), re.clone(), re.clone().star()]),
    re.clone().rep(3, None)
  );

  assert_eq!(
    Re::cat_all(vec![
      re.clone(),
      re.clone(),
      re.clone(),
      Re::alt(Re::Nil, Re::cat(re.clone(), Re::alt(Re::Nil, re.clone())))
    ]),
    re.clone().rep(3, 5)
  )
}

#[test]
#[should_panic(expected = "max < min")]
fn rep_bad() { Re::Lit::<char>('a').rep(5, 4); }

#[test]
fn test_disp() {
  assert_eq!("ϵ", format!("{}", Re::Nil::<char>));

  assert_eq!(
    "｢a｣",
    format!("{}", {
      // I'm using .into() to ensure that From is implemented correctly
      let r: Re<_> = 'a'.into();
      r
    })
  );

  // assert_eq!(); // TODO: test the other variants

  // Test precedence
  assert_eq!(
    "｢a｣|(｢a｣~｢b｣)*~｢c｣",
    format!(
      "{}",
      Re::alt(
        'a'.into(),
        Re::cat(Re::cat('a'.into(), 'b'.into()).star(), 'c'.into()),
      )
    )
  );
}
