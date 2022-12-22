use swc_core::ecma::ast::Expr;

use crate::{shared::convert::Convert, state::State};

pub struct Directive<'a> {
    name: &'a str,
    value: Box<Expr>,
}

impl<'a, 's> Convert<'s, Expr> for Directive<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        let Self { name, value } = self;

        todo!()
    }
}
