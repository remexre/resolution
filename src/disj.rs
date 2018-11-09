use crate::Term;
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A disjunction, formed as a series of `Term`s.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Disj(pub BTreeSet<Term>);

impl Disj {
    /// Tries to annihilate two disjunctions together. Returns `None` if they do not annihilate,
    /// `Some(None)` if they annihilate to a tautology, and `Some(Some((d, c)))` if the term named
    /// `c` annihilates to form a new disjunction `d`.
    pub fn annihilate(&self, other: &Disj) -> Option<(Option<Disj>, char)> {
        let t = self.find_annihilating(other)?.name();
        let l = self.without(t);
        let r = other.without(t);
        Some((l.union(&r), t))
    }

    /// Returns the contradictory `Disj`.
    pub fn contradiction() -> Disj {
        Disj(BTreeSet::new())
    }

    /// Returns a `Display` for this value that uses U+2228 and U+00AC instead of `V` and `!`.
    pub fn display_unicode<'a>(&'a self) -> impl 'a + Display {
        DisplayUnicode(self)
    }

    /// Finds the name of a term that appears positive in one and negative in another.
    pub fn find_annihilating(&self, other: &Disj) -> Option<Term> {
        if self.0.len() > other.0.len() {
            return other.find_annihilating(self);
        }

        self.0
            .iter()
            .find(|x| other.0.contains(&x.opposite()))
            .cloned()
    }

    /// Returns whether the given disjunction is a contradiction.
    pub fn is_contradiction(&self) -> bool {
        self.0.is_empty()
    }

    /// Unions two disjunctions together. Returns `None` if the union is a tautology.
    pub fn union(&self, other: &Disj) -> Option<Disj> {
        if self.find_annihilating(other).is_some() {
            None
        } else {
            Some(Disj(self.0.union(&other.0).cloned().collect()))
        }
    }

    /// Returns a new disjunction that does not contain terms with the given name.
    pub fn without(&self, name: char) -> Disj {
        let mut set = self.0.clone();
        set.remove(&Term::Pos(name));
        set.remove(&Term::Neg(name));
        Disj(set)
    }
}

impl Display for Disj {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let mut iter = self.0.iter();
        match iter.next() {
            None => write!(fmt, "F"),
            Some(fst) => {
                if self.0.len() == 1 {
                    write!(fmt, "{}", fst)
                } else {
                    write!(fmt, "({}", fst)?;
                    for term in iter {
                        write!(fmt, " V {}", term)?;
                    }
                    write!(fmt, ")")
                }
            }
        }
    }
}

struct DisplayUnicode<'a>(&'a Disj);

impl<'a> Display for DisplayUnicode<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let mut iter = (self.0).0.iter();
        match iter.next() {
            None => write!(fmt, "\u{22a5}"),
            Some(fst) => {
                if (self.0).0.len() == 1 {
                    write!(fmt, "{}", fst.display_unicode())
                } else {
                    write!(fmt, "({}", fst.display_unicode())?;
                    for term in iter {
                        write!(fmt, " \u{2228} {}", term.display_unicode())?;
                    }
                    write!(fmt, ")")
                }
            }
        }
    }
}
