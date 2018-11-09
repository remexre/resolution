mod ascii;

use crate::{Disj, KB};
use failure::Fallible;
use std::str::FromStr;

/// The output mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputMode {
    /// ASCII-art sequents will be output to show the derivation.
    Ascii,

    /// [bussproofs](https://ctan.org/pkg/bussproofs)-compatible proofs will be output.
    LaTeX,

    /// No output will be printed to stdout. The program will exit with status 0 if the goal was
    /// provable, and 1 if it was not.
    Silent,

    /// Similar to `Ascii`, but uses U+2228 and U+00AC instead of `V` and `!`.
    Unicode,
}

impl OutputMode {
    /// Renders the `KB` to `stdout` with the given render mode.
    pub fn render_contradicted_kb(self, kb: &KB) -> Fallible<()> {
        let disj = Disj::contradiction();
        match self {
            OutputMode::Ascii => ascii::render_sequent(kb, &disj),
            OutputMode::LaTeX => unimplemented!(), // latex::render_sequent(kb, &disj),
            OutputMode::Silent => {}
            OutputMode::Unicode => unimplemented!(), // unicode::render_sequent(kb, &disj),
        }
        Ok(())
    }
}

impl FromStr for OutputMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<OutputMode, &'static str> {
        match &s.to_lowercase() as &str {
            "ascii" => Ok(OutputMode::Ascii),
            "latex" => Ok(OutputMode::LaTeX),
            "silent" => Ok(OutputMode::Silent),
            "unicode" => Ok(OutputMode::Unicode),
            _ => Err("unknown output mode"),
        }
    }
}
