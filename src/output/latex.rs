use crate::{output::Outputter, Disj, Term};
use failure::Fallible;
use std::io::Write;

pub struct LaTeX<W>(pub W);

impl<W: Write> LaTeX<W> {
    fn render_disj(&mut self, disj: &Disj) -> Fallible<()> {
        let mut iter = disj.0.iter();
        match iter.next() {
            None => write!(self.0, "\\bot")?,
            Some(fst) => {
                if disj.0.len() == 1 {
                    self.render_term(fst)?;
                } else {
                    write!(self.0, "\\left(")?;
                    self.render_term(fst)?;
                    for term in iter {
                        write!(self.0, " \\lor ",)?;
                        self.render_term(term)?;
                    }
                    write!(self.0, "\\right)")?;
                }
            }
        }
        Ok(())
    }

    fn render_term(&mut self, term: &Term) -> Fallible<()> {
        match term {
            Term::Pos(c) => write!(self.0, "{}", c)?,
            Term::Neg(c) => write!(self.0, "\\lnot {}", c)?,
        }
        Ok(())
    }
}

impl<W: Write> Outputter for LaTeX<W> {
    fn render_sequent_assumed(&mut self, disj: &Disj, _: usize) -> Fallible<()> {
        writeln!(self.0, "\\AxiomC{{assumed}}")?;
        write!(self.0, "\\UnaryInfC{{$")?;
        self.render_disj(disj)?;
        writeln!(self.0, "$}}")?;
        Ok(())
    }

    fn render_sequent_known(&mut self, disj: &Disj, _: usize) -> Fallible<()> {
        writeln!(self.0, "\\AxiomC{{known}}")?;
        write!(self.0, "\\UnaryInfC{{$")?;
        self.render_disj(disj)?;
        writeln!(self.0, "$}}")?;
        Ok(())
    }

    fn render_sequent_annih(
        &mut self,
        disj: &Disj,
        n: usize,
        l: usize,
        r: usize,
        _: char,
    ) -> Fallible<()> {
        self.render_sequent_union(disj, n, l, r)
    }

    fn render_sequent_union(&mut self, disj: &Disj, _: usize, _: usize, _: usize) -> Fallible<()> {
        write!(self.0, "\\BinaryInfC{{$")?;
        self.render_disj(disj)?;
        writeln!(self.0, "$}}")?;
        Ok(())
    }
}
