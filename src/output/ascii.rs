use crate::{Cause, Disj, KB};

pub fn render_sequent(kb: &KB, disj: &Disj) {
    match kb.cause(disj) {
        Cause::Assumed => render_sequent_assumed(disj),
        Cause::Known => render_sequent_known(disj),
        Cause::Derived(l, r, Some(c)) => render_sequent_annih(disj, &l, &r, c),
        Cause::Derived(l, r, None) => render_sequent_union(disj, &l, &r),
    }
}

fn render_sequent_assumed(disj: &Disj) {
    unimplemented!()
}

fn render_sequent_known(disj: &Disj) {
    unimplemented!()
}

fn render_sequent_annih(disj: &Disj, l: &Disj, r: &Disj, c: char) {
    unimplemented!()
}

fn render_sequent_union(disj: &Disj, l: &Disj, r: &Disj) {
    unimplemented!()
}
