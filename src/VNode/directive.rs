use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{ArrayLit, Expr, Ident},
        utils::ExprFactory,
    },
};

use crate::{
    constant::{MODEL, SHOW},
    shared::convert::Convert,
    state::State,
};

pub struct Directive<'a> {
    name: &'a str,
    value: Box<Expr>,
}

impl<'a> Directive<'a> {
    pub fn resolve<'s, S: State<'s>>(name: &str, state: &mut S) -> Ident {
        let resolve_directive = state.import_from_vue("resolveDirective");

        let resolve_expr = resolve_directive.as_call(DUMMY_SP, vec![name.as_arg()]);

        state.scope_expr(resolve_expr)
    }
}

impl<'a, 's> Convert<'s, Expr> for Directive<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        let Self { name, value } = self;

        let name_ident = match name.as_ref() {
            MODEL => state.import_from_vue("vModelText"),
            SHOW => state.import_from_vue("vShow"),
            name => Self::resolve(name, state),
        };

        ArrayLit {
            span: DUMMY_SP,
            elems: vec![Some(name_ident.as_arg()), Some(value.clone().as_arg())],
        }
        .into()
    }
}

impl<'a, 's> Convert<'s, Expr> for [Directive<'a>] {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        ArrayLit {
            span: DUMMY_SP,
            elems: self
                .iter()
                .map(|directive| Some(directive.convert(state).as_arg()))
                .collect(),
        }
        .into()
    }
}

pub trait PushDirective<'a> {
    fn push_directive(&mut self, name: &'a str, value: Box<Expr>);
}

impl<'a> PushDirective<'a> for Vec<Directive<'a>> {
    fn push_directive(&mut self, name: &'a str, value: Box<Expr>) {
        self.push(Directive { name, value })
    }
}
