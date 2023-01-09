use swc_core::ecma::ast::Expr;
use swc_helper_jsx_transform::text::Text;

use crate::{args, context::Context, shared::convert::Convert};

impl<'a, 'b> Convert<'a, Expr> for Text<'b> {
    fn convert(&self, ctx: &mut impl Context<'a>) -> Expr {
        ctx.create_text_vnode(args![self.content.clone()])
    }
}
