use swc_core::ecma::ast::Expr;
use swc_helper_jsx_transform::text::Text;

use crate::{args, context::Context, convert::Convert};

impl<'a> Convert<Expr> for Text<'a> {
    fn convert<C: Context>(&self, ctx: &mut C) -> Expr {
        ctx.create_text_vnode(args![self.content.clone()])
    }
}
