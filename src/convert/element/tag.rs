use swc_core::ecma::ast::{Expr, Ident, JSXMemberExpr};
use swc_helper_jsx_transform::element::tag::Tag;

use crate::{context::Context, convert::Convert};

impl<'a> Convert<Expr> for Tag<'a> {
    fn convert<C: Context>(&self, ctx: &mut C) -> Expr {
        match self {
            Self::Native(name) => Expr::from(*name),
            Self::Extra(ident) => {
                if ctx.is_unresolved(ident) {
                    ctx.resolve("resolveComponent", ident.as_ref())
                } else {
                    Ident::clone(ident).into()
                }
            },
            Self::Member(member) => JSXMemberExpr::clone(member).into(),
        }
    }
}
