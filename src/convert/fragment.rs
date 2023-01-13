use swc_core::ecma::ast::Expr;
use swc_helper_jsx_transform::fragment::Fragment;

use crate::{
    args,
    constant::{FRAGMENT, NULL_EXPR},
    context::Context,
    convert::Convert,
};

impl<'a> Convert<Expr> for Fragment<'a> {
    fn convert<C: Context>(&self, ctx: &mut C) -> Expr {
        let fragment_ident = ctx.import_from_vue(FRAGMENT);

        let children_expr = self.children.convert(ctx);

        ctx.create_vnode(args![fragment_ident, NULL_EXPR, children_expr])
    }
}
