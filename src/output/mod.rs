mod ascii;
mod latex;

use crate::{
    output::{ascii::Ascii, latex::LaTeX},
    Cause, Disj, KB,
};
use failure::Fallible;
use std::{io::Write, str::FromStr};

/// The output mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputMode {
    /// ASCII-art sequents will be output to show the derivation.
    Ascii,

    /// [bussproofs](https://ctan.org/pkg/bussproofs)-compatible proofs will be output.
    LaTeX,

    /// Creates an actual LaTeX PDF using
    /// [tectonic](https://github.com/tectonic-typesetting/tectonic).
    #[cfg(feature = "tectonic")]
    LaTeXTectonic,

    /// No output will be printed to stdout. The program will exit with status 0 if the goal was
    /// provable, and 1 if it was not.
    Silent,

    /// Similar to `Ascii`, but uses U+2228 and U+00AC instead of `V` and `!`.
    Unicode,
}

impl OutputMode {
    /// Renders the `KB` to the given Write with the given render mode. Returns the number of
    /// sequents rendered.
    pub fn render_contradicted_kb<W: Write>(self, kb: &KB, mut w: W) -> Fallible<usize> {
        let disj = Disj::contradiction();
        let mut next_index = 0;
        match self {
            OutputMode::Ascii => render_sequent(&mut next_index, &mut Ascii(w, false), kb, &disj),
            OutputMode::LaTeX => {
                writeln!(w, "\\begin{{prooftree}}").ok();
                let n = render_sequent(&mut next_index, &mut LaTeX(&mut w), kb, &disj)?;
                writeln!(w, "\\end{{prooftree}}").ok();
                Ok(n)
            }
            #[cfg(feature = "tectonic")]
            OutputMode::LaTeXTectonic => {
                let mut src = Vec::new();
                writeln!(&mut src, "\\documentclass{{article}}").ok();
                writeln!(&mut src, "\\usepackage{{bussproofs}}").ok();
                writeln!(&mut src, "\\begin{{document}}").ok();
                writeln!(&mut src, "\\begin{{prooftree}}").ok();
                let n = render_sequent(&mut next_index, &mut LaTeX(&mut src), kb, &disj)?;
                writeln!(&mut src, "\\end{{prooftree}}").ok();
                writeln!(&mut src, "\\end{{document}}").ok();

                let src = String::from_utf8_lossy(&src);
                let bytes = ::tectonic::latex_to_pdf(src).map_err(::failure::SyncFailure::new)?;
                w.write_all(&bytes)?;
                Ok(n)
            }
            OutputMode::Silent => Ok(0),
            OutputMode::Unicode => render_sequent(&mut next_index, &mut Ascii(w, true), kb, &disj),
        }
    }
}

impl FromStr for OutputMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<OutputMode, &'static str> {
        match &s.to_lowercase() as &str {
            "ascii" => Ok(OutputMode::Ascii),
            "latex" => Ok(OutputMode::LaTeX),
            #[cfg(feature = "tectonic")]
            "latex-tectonic" => Ok(OutputMode::LaTeXTectonic),
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
