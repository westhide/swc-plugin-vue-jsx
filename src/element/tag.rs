use swc_core::ecma::ast::{Expr, Ident, JSXMemberExpr};
use swc_helper_jsx_transform::element::tag::Tag;

use crate::{context::Context, shared::convert::Convert};

impl<'a, 'b> Convert<'a, Expr> for Tag<'b> {
    fn convert(&self, ctx: &mut impl Context<'a>) -> Expr {
        match self {
            Self::Native(name) => Expr::from(*name),
            Self::Extra(ident) => {
                let name = ident.as_ref();

                if ctx.is_custom_element(name) {
                    name.into()
                } else if ctx.has_mark(ident) {
                    Ident::clone(ident).into()
                } else {
                    ctx.resolve("resolveComponent", name)
                }
            },
            Self::Member(member) => JSXMemberExpr::clone(member).into(),
        }
    }
}
