use swc_core::ecma::ast::Expr;

use crate::{
    array_lit,
    constant::{V_MODEL, V_SHOW},
    context::Context,
};

#[derive(Debug)]
pub struct Directive<'s> {
    pub name: &'s str,
    pub value: Expr,
}

impl<'a, 's> Directive<'s> {
    pub fn convert_into_expr(self, ctx: &mut impl Context<'a>) -> Expr {
        let Self { name, value } = self;

        let directive = match name {
            V_SHOW => ctx.import_from_vue("vShow").into(),
            // TODO
            V_MODEL => ctx.import_from_vue("vModelText").into(),
            name => ctx.resolve("resolveDirective", &name[2..]),
        };

        array_lit![directive, value].into()
    }
}
