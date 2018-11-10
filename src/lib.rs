#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[cfg(feature = "tectonic")]
extern crate tectonic;

mod disj;
mod kb;
mod output;
mod parser;
mod term;

pub use crate::{
    disj::Disj,
    kb::{Cause, KB},
    output::OutputMode,
    parser::ParseError,
    term::Term,
};
