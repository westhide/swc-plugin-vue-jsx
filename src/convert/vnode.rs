use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{ArrayLit, Expr, ExprOrSpread},
};
use swc_helper_jsx_transform::vnode::VNode;

use crate::{
    args,
    context::Context,
    convert::{
        split_static::{Block, SplitStatic, StaticContent},
        Convert,
    },
    shared::add::Add,
};

impl<'a> Convert<ExprOrSpread> for VNode<'a> {
    fn convert<C: Context>(&self, ctx: &mut C) -> ExprOrSpread {
        match self {
            Self::Text(text) => text.convert(ctx).into(),
            Self::Element(element) => element.convert(ctx).into(),
            Self::Expr(expr) => Expr::clone(expr).into(),
            Self::Spread(expr) => {
                ExprOrSpread {
                    spread: Some(DUMMY_SP),
                    expr: Box::new(Expr::clone(expr)),
                }
            },
            Self::Fragment(fragment) => fragment.convert(ctx).into(),
        }
    }
}

impl<'a> Convert<Expr> for [VNode<'a>] {
    fn convert<C: Context>(&self, ctx: &mut C) -> Expr {
        let threshold = ctx.static_threshold();

        let mut elems = Vec::with_capacity(self.len());

        self.split_static().for_each(|block| {
            match block {
                Block::VNode(vnode) => elems.add(vnode.convert(ctx)),
                Block::Static(statics) => {
                    let num = statics.len();

                    if num < threshold {
                        statics.iter().for_each(|vnode| {
                            elems.add(vnode.convert(ctx));
                        })
                    } else {
                        let content = statics.static_content();

                        let static_vnode_expr = ctx.create_static_vnode(args![content, num]);

                        elems.add(static_vnode_expr.into())
                    }
                },
            }
        });

        ArrayLit {
            span: DUMMY_SP,
            elems,
        }
        .into()
    }
}
