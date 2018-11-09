use crate::{output::Outputter, Disj};
use failure::Fallible;
use std::{cmp::max, io::Write};

pub struct Ascii<W>(pub W, pub usize);

impl<W: Write> Ascii<W> {
    fn chs(&mut self, ch: char, n: usize) -> Fallible<()> {
        for _ in 0..n {
            write!(self.0, "{}", ch)?;
        }
        Ok(())
    }

    fn render_two_lines(&mut self, left: usize, top: &str, bot: &str, right: &str) -> Fallible<()> {
        if left != 0 {
            writeln!(self.0)?;
        }

        let left = format!("{}", left);

        let tl = top.len();
        let bl = bot.len();
        let ll = left.len();
        let tp = if tl < bl { (bl - tl) / 2 } else { 0 };
        let bp = if tl > bl { (tl - bl) / 2 } else { 0 };

        self.chs(' ', 4 + ll + tp)?;
        writeln!(self.0, "{}", top)?;
        write!(self.0, "({}) ", left)?;
        self.chs('-', 2 + max(tl, bl))?;
        write!(self.0, " [{}]\n", right)?;
        self.chs(' ', 4 + ll + bp)?;
        writeln!(self.0, "{}", bot)?;

        Ok(())
    }
}

impl<W: Write> Outputter for Ascii<W> {
    fn render_sequent_assumed(&mut self, disj: &Disj, n: usize) -> Fallible<()> {
        let disj = disj.to_string();
        self.render_two_lines(n, "Assumed", &disj, "AS")
    }

    fn render_sequent_known(&mut self, disj: &Disj, n: usize) -> Fallible<()> {
        let disj = disj.to_string();
        self.render_two_lines(n, "Known", &disj, "KB")
    }

    fn render_sequent_annih(
        &mut self,
        disj: &Disj,
        n: usize,
        l: usize,
        r: usize,
        c: char,
    ) -> Fallible<()> {
        let top = format!("({})   ({})", l, r);
        let bot = disj.to_string();
        let reason = format!("AN({})", c);
        self.render_two_lines(n, &top, &bot, &reason)
    }

    fn render_sequent_union(&mut self, disj: &Disj, n: usize, l: usize, r: usize) -> Fallible<()> {
        let top = format!("({})   ({})", l, r);
        let bot = disj.to_string();
        self.render_two_lines(n, &top, &bot, "UN")
    }
}
