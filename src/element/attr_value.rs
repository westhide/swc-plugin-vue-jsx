use swc_core::ecma::ast::{Expr, Lit};
use swc_helper_jsx_transform::attr::value::Value;

use crate::{context::Context, shared::convert::Convert};

impl<'a, 'b> Convert<'a, Expr> for Value<'b> {
    fn convert(&self, ctx: &mut impl Context<'a>) -> Expr {
        match self {
            Self::Lit(lit) => Lit::clone(lit).into(),
            Self::Const(expr) | Self::Expr(expr) => Expr::clone(expr),
            Self::Element(element) => element.convert(ctx),
            Self::Fragment(fragment) => fragment.convert(ctx),
            Self::Empty => panic!("Forbidden: Empty attr value"),
        }
    }
}
