use crate::Disj;
use failure::Fallible;
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
    path::Path,
    rc::Rc,
};

/// The reason a `Disj` is in the `KB`.
#[derive(Clone, Debug)]
pub enum Cause {
    /// The `Disj` was assumed, in order to prove by contradiction.
    Assumed,

    /// The `Disj` was part of the initial knowlege base.
    Known,

    /// The `Disj` was derived from other facts, which are the accompanying data. If the derivation
    /// was performed by annihilation, the annihilated variable is present.
    Derived(Rc<Disj>, Rc<Disj>, Option<char>),
}

/// The knowledge base.
#[derive(Debug)]
pub struct KB(Rc<RefCell<KBInner>>);

impl KB {
    /// Creates a new `KB` with the given goal and known facts.
    pub fn new(goal: Disj, facts: Vec<Disj>) -> KB {
        let mut causes = Vec::new();
        let mut known = HashMap::new();

        for disj in facts {
            let disj = Rc::new(disj);
            known.insert(disj.clone(), causes.len());
            causes.push((disj, Cause::Known));
        }

        for term in goal.0 {
            let mut set = BTreeSet::new();
            set.insert(term.opposite());
            let disj = Rc::new(Disj(set));
            known.insert(disj.clone(), causes.len());
            causes.push((disj, Cause::Assumed));
        }

        KB(Rc::new(RefCell::new(KBInner { causes, known })))
    }

    /// Loads the `KB` from the given file.
    pub fn from_file(path: impl AsRef<Path>) -> Fallible<KB> {
        let mut file = BufReader::new(File::open(path)?);
        let mut line = String::new();

        file.read_line(&mut line)?;
        let goal = line.parse()?;
        line.clear();

        let mut facts = Vec::new();
        loop {
            line.clear();
            match file.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {}
                Err(err) => if err.kind() == ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(err.into());
                },
            }
            if line.chars().all(|ch| ch.is_whitespace()) {
                continue;
            }
            facts.push(line.parse()?);
        }

        if log_enabled!(::log::Level::Debug) {
            debug!("Goal: {}", goal);
            debug!("Facts:");
            for f in &facts {
                debug!("  {}", f);
            }
        }
        Ok(KB::new(goal, facts))
    }

    /// Returns the reason the given `Disj` is in the `KB`. Panics if the `Disj` is not actually in
    /// the `KB`.
    pub fn cause(&self, disj: &Disj) -> Cause {
        let inner = self.0.borrow();
        let idx = inner
            .known
            .get(disj)
            .expect("Tried to get cause of nonexistent Disj");
        inner.causes[*idx].1.clone()
    }

    /// Tries combining the given two `Disj`s, adding the result to the `KB` if it is novel. Ensure
    /// that the `Disj`s are already in the KB!
    fn combine(&self, l: Rc<Disj>, r: Rc<Disj>) -> Option<Rc<Disj>> {
        let (disj, c) = match l.annihilate(&r) {
            None => (l.union(&r).unwrap(), None),
            Some((None, _)) => return None,
            Some((Some(disj), c)) => (disj, Some(c)),
        };
        let disj = Rc::new(disj);

        let mut inner = self.0.borrow_mut();
        if inner.known.contains_key(&disj) {
            return None;
        }

        let len = inner.causes.len();
        inner.known.insert(disj.clone(), len);
        inner.causes.push((disj.clone(), Cause::Derived(l, r, c)));
        Some(disj)
    }

    /// Forward chains until a contradiction is reached, returning false if a contradiction is
    /// found.
    pub fn forward_chain(&self) -> bool {
        for l in self.iter() {
            for r in self.iter() {
                if let Some(disj) = self.combine(l.clone(), r.clone()) {
                    trace!("{} & {} -> {}", l, r, disj);
                    if disj.is_contradiction() {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Returns an iterator over the `Disj`s in the `KB`. If additional items are added to the `KB`
    /// while the iterator exists, it will add them to the end of the queue.
    pub fn iter(&self) -> impl Iterator<Item = Rc<Disj>> {
        KBIter(self.0.clone(), 0)
    }
}

/// The inner data in the knowledge base.
#[derive(Debug)]
struct KBInner {
    causes: Vec<(Rc<Disj>, Cause)>,
    known: HashMap<Rc<Disj>, usize>,
}

/// An iterator over the knowledge base.
struct KBIter(Rc<RefCell<KBInner>>, usize);

impl Iterator for KBIter {
    type Item = Rc<Disj>;
    fn next(&mut self) -> Option<Rc<Disj>> {
        let inner = self.0.borrow();
        if inner.causes.len() <= self.1 {
            None
        } else {
            let val = inner.causes[self.1].0.clone();
            self.1 += 1;
            Some(val)
        }
    }
}
