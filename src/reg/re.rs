use super::nfa::builder::NfaBuilder;
use std::{
  fmt::{self, Display, Formatter},
  hash::Hash,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Re<T> {
  Nil,                         // An empty string, ϵ
  Lit(T),                      // A literal,       a
  Rep(Box<Re<T>>),             // Kleene closure,  a*
  Cat(Box<Re<T>>, Box<Re<T>>), // Concatenation,   a b ... z
  Alt(Box<Re<T>>, Box<Re<T>>), // Alternation,     a | b | ... | z
}

impl<T> Re<T> {
  #[inline]
  pub fn cat(a: Self, b: Self) -> Self { Re::Cat(Box::new(a), Box::new(b)) }

  #[inline]
  pub fn alt(a: Self, b: Self) -> Self { Re::Alt(Box::new(a), Box::new(b)) }

  #[inline]
  pub fn star(self) -> Self { Re::Rep(Box::new(self)) }

  #[inline]
  pub fn opt(self) -> Self { Re::alt(Re::Nil, self) }

  pub fn cat_all<I: IntoIterator<Item = Self>>(it: I) -> Self
  where
    <I as IntoIterator>::IntoIter: DoubleEndedIterator,
  {
    let mut it = it.into_iter().rev(); // TODO: can I do this top-down instead?

    let mut ret = it.next().expect("Re::cat_all called with empty iterator");

    for re in it {
      ret = Re::cat(re, ret);
    }

    ret
  }

  pub fn alt_all<I: IntoIterator<Item = Self>>(it: I) -> Self
  where
    <I as IntoIterator>::IntoIter: DoubleEndedIterator,
  {
    let mut it = it.into_iter().rev(); // TODO: can I do this top-down instead?

    let mut ret = it.next().expect("Re::alt_all called with empty iterator");

    for re in it {
      ret = Re::alt(re, ret);
    }

    ret
  }
}

impl<T: Clone> Re<T> {
  #[inline]
  pub fn plus(self) -> Self { Re::cat(self.clone(), self.star()) }

  // TODO: can the last instance of self be moved into the result instead?
  pub fn dup(&self, n: u32) -> Self {
    Re::cat_all((0..n).into_iter().map(|_| self.clone()))
  }

  pub fn rep<X: Into<Option<u32>>>(&self, min: u32, max: X) -> Self {
    let max = max.into();

    match max {
      Some(max) => {
        if max < min {
          panic!("Re::rep called with max < min");
        } else if max == min {
          self.dup(min)
        } else {
          assert!(max > min);

          let mut ret = Re::alt(Re::Nil, self.clone());

          for _ in 0..max - min - 1 {
            ret = Re::alt(Re::Nil, Re::cat(self.clone(), ret));
          }

          for _ in 0..min {
            ret = Re::cat(self.clone(), ret);
          }

          ret
        }
      },
      None => {
        let mut ret = self.clone().star();

        for _ in 0..min {
          ret = Re::cat(self.clone(), ret);
        }

        ret
      },
    }
  }
}

impl<T: Hash + Eq> Re<T> {
  #[inline]
  pub fn build_nfa(self) -> NfaBuilder<T> { self.into() }
}

impl<T: Display> Re<T> {
  fn disp_prec(&self, prec: u32, fmt: &mut Formatter) -> fmt::Result {
    use self::Re::*;

    let my_prec = match self {
      Nil => 3,
      Lit(_) => 3,
      Rep(_) => 2,
      Cat(_, _) => 1,
      Alt(_, _) => 0,
    };

    if my_prec < prec {
      fmt.write_str("(")?;
    }

    match self {
      Nil => fmt.write_str("ϵ")?,
      Lit(l) => {
        fmt.write_str("｢")?;
        l.fmt(fmt)?;
        fmt.write_str("｣")?;
      },
      Rep(r) => {
        r.disp_prec(my_prec, fmt)?;
        fmt.write_str("*")?;
      },
      Cat(a, b) => {
        a.disp_prec(my_prec, fmt)?;
        fmt.write_str("~")?;
        b.disp_prec(my_prec, fmt)?;
      },
      Alt(a, b) => {
        a.disp_prec(my_prec, fmt)?;
        fmt.write_str("|")?;
        b.disp_prec(my_prec, fmt)?;
      },
    }

    if my_prec < prec {
      fmt.write_str(")")?;
    }

    Ok(())
  }
}

impl<T> From<T> for Re<T> {
  fn from(lit: T) -> Self { Re::Lit(lit) }
}

impl<T: Display> Display for Re<T> {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.disp_prec(0, fmt) }
}
