use swc_core::ecma::{
    ast::{Expr, ExprOrSpread},
    utils::ExprFactory,
};

use crate::{
    array_lit,
    constant::{V_MODEL, V_SHOW},
    context::Context,
};

#[derive(Debug)]
pub struct Directive<'a> {
    pub name: &'a str,
    pub value: Expr,
}

impl<'a> Directive<'a> {
    pub fn into_arg<C: Context>(self, ctx: &mut C) -> ExprOrSpread {
        let Self { name, value } = self;

        let directive = match name {
            V_SHOW => ctx.import_from_vue("vShow").into(),
            // TODO
            V_MODEL => ctx.import_from_vue("vModelText").into(),
            name => ctx.resolve("resolveDirective", &name[2..]),
        };

        array_lit![directive, value].as_arg()
    }
}
