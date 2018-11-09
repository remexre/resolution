use crate::{Disj, Term};
use std::{collections::BTreeSet, str::FromStr};

#[derive(Clone, Copy, Debug, Fail, Eq, PartialEq)]
pub enum ParseError {
    /// A duplicate name was found.
    #[fail(display = "The name {:?} was found more than once.", _0)]
    DuplicateName(char),

    /// An invalid name was found.
    #[fail(display = "An invalid name was found.")]
    InvalidName,

    /// A vertical bar was not found.
    #[fail(display = "A vertical bar was not found.")]
    MissingBar,

    /// Too many vertical bars were found.
    #[fail(display = "Too many vertical bars were found.")]
    TooManyBars,
}

fn invalid_char(ch: char) -> bool {
    match ch {
        'a'...'z' => false,
        '|' => false,
        _ => !ch.is_whitespace(),
    }
}

impl FromStr for Disj {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Disj, ParseError> {
        if s.chars().any(invalid_char) {
            return Err(ParseError::InvalidName);
        }

        let bar_idx = s.find('|').ok_or(ParseError::MissingBar)?;
        let pos = &s[..bar_idx];
        let neg = &s[(bar_idx + 1)..];
        if neg.find('|').is_some() {
            return Err(ParseError::TooManyBars);
        }

        let mut seen = BTreeSet::new();
        let mut set = BTreeSet::new();

        for ch in pos.chars() {
            if ch.is_whitespace() {
                continue;
            }
            if !seen.insert(ch) {
                return Err(ParseError::DuplicateName(ch));
            }
            set.insert(Term::Pos(ch));
        }
        for ch in neg.chars() {
            if ch.is_whitespace() {
                continue;
            }
            if !seen.insert(ch) {
                return Err(ParseError::DuplicateName(ch));
            }
            set.insert(Term::Neg(ch));
        }

        Ok(Disj(set))
    }
}
