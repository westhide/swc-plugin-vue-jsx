use swc_core::ecma::ast::{Expr, Lit};
use swc_helper_jsx_transform::attr::value::Value;

use crate::{constant::NULL_EXPR, context::Context, convert::Convert};

impl<'a> Convert<Expr> for Value<'a> {
    fn convert<C: Context>(&self, ctx: &mut C) -> Expr {
        match self {
            Self::Lit(lit) => Lit::clone(lit).into(),
            Self::Const(expr) | Self::Expr(expr) => Expr::clone(expr),
            Self::Element(element) => element.convert(ctx),
            Self::Fragment(fragment) => fragment.convert(ctx),
            Self::Empty => NULL_EXPR,
        }
    }
}
