use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A variable, or its negation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Term {
    /// A positive variable.
    Pos(char),

    /// A negative variable.
    Neg(char),
}

impl Term {
    /// Returns a `Display` for this value that uses U+2228 and U+00AC instead of `V` and `!`.
    pub fn display_unicode<'a>(&'a self) -> impl 'a + Display {
        DisplayUnicode(self)
    }

    /// Returns the name of the term.
    pub fn name(self) -> char {
        match self {
            Term::Pos(c) => c,
            Term::Neg(c) => c,
        }
    }

    /// Returns whether the two terms are inverses.
    pub fn opposes(self, other: Term) -> bool {
        self.name() == other.name() && self.sign() != other.sign()
    }

    /// Returns the opposite of the term.
    pub fn opposite(self) -> Term {
        match self {
            Term::Pos(c) => Term::Neg(c),
            Term::Neg(c) => Term::Pos(c),
        }
    }

    /// Returns true if the term is positive.
    pub fn sign(self) -> bool {
        match self {
            Term::Pos(_) => true,
            Term::Neg(_) => false,
        }
    }
}

impl Display for Term {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            Term::Pos(c) => write!(fmt, "{}", c),
            Term::Neg(c) => write!(fmt, "!{}", c),
        }
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Term) -> Ordering {
        (self.name(), self.sign()).cmp(&(other.name(), other.sign()))
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DisplayUnicode<'a>(&'a Term);

impl<'a> Display for DisplayUnicode<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self.0 {
            Term::Pos(c) => write!(fmt, "{}", c),
            Term::Neg(c) => write!(fmt, "\u{00ac}{}", c),
        }
    }
}
