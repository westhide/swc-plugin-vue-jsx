use swc_core::ecma::ast::{Expr, JSXMemberExpr};
use swc_helper_jsx_transform::element::tag::Tag;

use crate::{context::Context, shared::convert::Convert};

impl<'a, 'b> Convert<'a, Expr> for Tag<'b> {
    fn convert(&self, ctx: &mut impl Context<'a>) -> Expr {
        match self {
            Self::Native(name) => Expr::from(*name),
            Self::Extra(name) => {
                if ctx.is_custom_element(name) {
                    Expr::from(*name)
                } else {
                    // TODO: scope component
                    ctx.resolve("resolveComponent", *name)
                }
            },
            Self::Member(member) => JSXMemberExpr::clone(member).into(),
        }
    }
}
