mod ascii;

use crate::{output::ascii::Ascii, Cause, Disj, KB};
use failure::Fallible;
use std::{io::stdout, str::FromStr};

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
    pub fn render_contradicted_kb(self, kb: &KB) -> Fallible<usize> {
        let disj = Disj::contradiction();
        let mut next_index = 0;
        match self {
            OutputMode::Ascii => {
                render_sequent(&mut next_index, &mut Ascii(stdout(), 0), kb, &disj)
            }
            OutputMode::LaTeX => unimplemented!(), // latex::render_sequent(kb, &disj),
            OutputMode::Silent => Ok(0),
            OutputMode::Unicode => unimplemented!(), // unicode::render_sequent(kb, &disj),
        }
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

trait Outputter {
    fn render_sequent_assumed(&mut self, disj: &Disj, n: usize) -> Fallible<()>;
    fn render_sequent_known(&mut self, disj: &Disj, n: usize) -> Fallible<()>;
    fn render_sequent_annih(
        &mut self,
        disj: &Disj,
        n: usize,
        l: usize,
        r: usize,
        c: char,
    ) -> Fallible<()>;
    fn render_sequent_union(&mut self, disj: &Disj, n: usize, l: usize, r: usize) -> Fallible<()>;
}

fn render_sequent<'a, O: Outputter>(
    next_index: &mut usize,
    out: &mut O,
    kb: &KB,
    disj: &'a Disj,
) -> Fallible<usize> {
    match kb.cause(disj) {
        Cause::Assumed => {
            let n = *next_index;
            *next_index += 1;
            out.render_sequent_assumed(disj, n)?;
            Ok(n)
        }
        Cause::Known => {
            let n = *next_index;
            *next_index += 1;
            out.render_sequent_known(disj, n)?;
            Ok(n)
        }
        Cause::Derived(l, r, Some(c)) => {
            let l = render_sequent(next_index, out, kb, &l)?;
            let r = render_sequent(next_index, out, kb, &r)?;
            let n = *next_index;
            *next_index += 1;
            out.render_sequent_annih(disj, n, l, r, c)?;
            Ok(n)
        }
        Cause::Derived(l, r, None) => {
            let l = render_sequent(next_index, out, kb, &l)?;
            let r = render_sequent(next_index, out, kb, &r)?;
            let n = *next_index;
            *next_index += 1;
            out.render_sequent_union(disj, n, l, r)?;
            Ok(n)
        }
    }
}
