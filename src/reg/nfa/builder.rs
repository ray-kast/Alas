use super::{super::prelude::*, Nfa};
use std::{
  borrow::Cow,
  cell::RefCell,
  collections::{HashMap, HashSet},
  fmt::{self, Debug, Formatter, Write},
  hash::{Hash, Hasher},
  mem,
  rc::{Rc, Weak},
};

pub struct NfaBuilder<T> {
  nodes: Vec<Rc<NfaNode<T>>>,
  head: NfaNodeRef<T>,
  tail: NfaNodeRef<T>,
}

pub struct NfaNode<T> {
  outs: RefCell<HashMap<Option<T>, HashSet<NfaNodeRef<T>>>>,
}

pub struct NfaNodeRef<T>(Weak<NfaNode<T>>);

impl<T> NfaBuilder<T> {
  // returns (nodes, head, tail)
  pub fn into_parts(
    self,
  ) -> (Vec<Rc<NfaNode<T>>>, NfaNodeRef<T>, NfaNodeRef<T>) {
    (self.nodes, self.head, self.tail)
  }
}

impl<T: Hash + Eq> NfaBuilder<T> {
  pub fn new() -> Self {
    let head = NfaNode::new_rc();
    let tail = NfaNode::new_rc();

    let head_ref = (&head).into();
    let tail_ref = (&tail).into();

    Self {
      nodes: vec![head, tail],
      head: head_ref,
      tail: tail_ref,
    }
  }

  #[inline]
  pub fn build(self) -> Nfa<T, usize> { self.into() }

  pub fn nodes(&self) -> &Vec<Rc<NfaNode<T>>> { &self.nodes }

  pub fn head(&self) -> &NfaNodeRef<T> { &self.head }

  pub fn tail(&self) -> &NfaNodeRef<T> { &self.tail }

  pub fn add_node(&mut self) -> Rc<NfaNode<T>> {
    let node = NfaNode::new_rc();

    self.nodes.push(node.clone());

    node
  }
}

impl<T: Hash + Eq> From<Re<T>> for NfaBuilder<T> {
  fn from(re: Re<T>) -> Self {
    use self::Re::*;

    let mut ret = NfaBuilder::new();

    match re {
      Nil => {
        ret.head.upgrade().unwrap().connect(None, ret.tail.clone());
      },
      Lit(l) => {
        ret.head.upgrade().unwrap().connect(l, ret.tail.clone());
      },
      Rep(r) => {
        let head = ret.head.upgrade().unwrap();

        head.connect(None, ret.tail.clone());

        let r = NfaBuilder::from(*r);

        ret.nodes.extend(r.nodes);

        head.connect(None, r.head.clone());

        let r_tail = r.tail.upgrade().unwrap();

        r_tail.connect(None, r.head);
        r_tail.connect(None, ret.tail.clone());
      },
      Cat(a, b) => {
        let a = NfaBuilder::from(*a);
        let b = NfaBuilder::from(*b);

        ret.nodes.extend(a.nodes);
        ret.nodes.extend(b.nodes);

        ret.head.upgrade().unwrap().connect(None, a.head);
        a.tail.upgrade().unwrap().connect(None, b.head);
        b.tail.upgrade().unwrap().connect(None, ret.tail.clone());
      },
      Alt(a, b) => {
        let a = NfaBuilder::from(*a);
        let b = NfaBuilder::from(*b);

        ret.nodes.extend(a.nodes);
        ret.nodes.extend(b.nodes);

        let head = ret.head.upgrade().unwrap();

        head.connect(None, a.head);
        head.connect(None, b.head);
        a.tail.upgrade().unwrap().connect(None, ret.tail.clone());
        b.tail.upgrade().unwrap().connect(None, ret.tail.clone());
      },
    }

    ret
  }
}

impl<T: Hash + Eq + Debug> NfaBuilder<T> {
  pub fn dot(&self) -> Result<String, fmt::Error> {
    let mut s = String::new();

    s.write_str("digraph{edge[arrowhead=normal,arrowtail=dot];")?;

    let mut ids: HashMap<NfaNodeRef<T>, usize> = HashMap::new();

    for (i, node) in self.nodes.iter().enumerate() {
      ids.insert(node.into(), i);

      // TODO: style head nodes

      write!(s, "{:?}", i.to_string())?;

      if NfaNodeRef::from(node) == self.tail {
        s.write_str("[peripheries=2];")?;
      } else {
        s.write_str(";")?;
      }
    }

    for (i, node) in self.nodes.iter().enumerate() {
      let i_str = i.to_string();

      for (by, outs) in &*node.outs.borrow() {
        let by_str: Cow<str> = by
          .as_ref()
          .map_or("λ".into(), |b| format!("{:?}", b).into());

        for to in outs {
          write!(
            s,
            "{:?}->{:?}[label={:?}];",
            i_str,
            ids[to].to_string(),
            by_str,
          )?;
        }
      }
    }

    s.write_str("}")?;

    Ok(s)
  }
}

impl<T: Hash + Eq> NfaNode<T> {
  fn new_rc() -> Rc<Self> {
    Rc::new(Self {
      outs: RefCell::new(HashMap::new()),
    })
  }

  pub fn connect<B: Into<Option<T>>, R: Into<NfaNodeRef<T>>>(
    &self,
    by: B,
    to: R,
  ) -> bool {
    self
      .outs
      .borrow_mut()
      .entry(by.into())
      .or_insert(HashSet::new())
      .insert(to.into())
  }

  pub fn take_outs(&self) -> HashMap<Option<T>, HashSet<NfaNodeRef<T>>> {
    let mut ret = HashMap::new();

    mem::swap(&mut ret, &mut *self.outs.borrow_mut());

    ret
  }
}

impl<T> NfaNodeRef<T> {
  pub fn upgrade(&self) -> Option<Rc<NfaNode<T>>> { self.0.upgrade() }
}

impl<T> From<&Rc<NfaNode<T>>> for NfaNodeRef<T> {
  fn from(node: &Rc<NfaNode<T>>) -> Self { NfaNodeRef(Rc::downgrade(node)) }
}

// NB: I don't know why, but the Clone derive didn't work correctly
impl<T> Clone for NfaNodeRef<T> {
  fn clone(&self) -> Self { NfaNodeRef(self.0.clone()) }
}

impl<T> Hash for NfaNodeRef<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    let ptr = Rc::into_raw(
      self
        .0
        .upgrade()
        .expect("attempted to hash dead reference")
        .clone(),
    );

    ptr.hash(state);

    let _ = unsafe { Rc::from_raw(ptr) };
  }
}

impl<T> PartialEq for NfaNodeRef<T> {
  fn eq(&self, rhs: &Self) -> bool {
    static DEAD_MSG: &str = "attempted to compare dead reference";

    Rc::ptr_eq(
      &self.0.upgrade().expect(DEAD_MSG),
      &rhs.0.upgrade().expect(DEAD_MSG),
    )
  }
}

impl<T> Eq for NfaNodeRef<T> {}

impl<T> Debug for NfaNodeRef<T> {
  fn fmt(&self, state: &mut Formatter) -> fmt::Result {
    state.write_str("NodeRef(")?;

    match self.0.upgrade() {
      Some(p) => {
        let ptr = Rc::into_raw(p);

        write!(state, "{:p}", ptr)?;

        let _ = unsafe { Rc::from_raw(ptr) };
      },
      None => {
        state.write_str("dead")?;
      },
    }

    state.write_str(")")?;

    Ok(())
  }
}
