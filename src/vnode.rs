use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{ArrayLit, Expr, ExprOrSpread},
        utils::ExprFactory,
    },
};
use swc_helper_jsx_transform::vnode::VNode;

use crate::{
    args,
    context::Context,
    shared::{add::Add, convert::Convert},
    split_static::{Item, SplitStatic},
};

impl<'a, 'b> Convert<'a, ExprOrSpread> for VNode<'b> {
    fn convert(&self, ctx: &mut impl Context<'a>) -> ExprOrSpread {
        match self {
            Self::Text(text) => text.convert(ctx).as_arg(),
            Self::Element(element) => element.convert(ctx).as_arg(),
            Self::Expr(expr) => Expr::clone(expr).as_arg(),
            Self::Spread(expr) => {
                ExprOrSpread {
                    spread: Some(DUMMY_SP),
                    expr: box Expr::clone(expr),
                }
            },
            Self::Fragment(fragment) => fragment.convert(ctx).as_arg(),
        }
    }
}

impl<'a, 'b> Convert<'a, Expr> for [VNode<'b>] {
    fn convert(&self, ctx: &mut impl Context<'a>) -> Expr {
        let threshold = ctx.static_threshold();

        let mut elems = Vec::with_capacity(self.len());

        self.split_static().for_each(|item| {
            match item {
                Item::VNode(vnode) => elems.add(vnode.convert(ctx)),
                Item::Static(statics) if statics.len() < threshold => {
                    for vnode in statics {
                        elems.add(vnode.convert(ctx))
                    }
                },
                Item::Static(statics) => {
                    let content: String = statics.iter().map(VNode::static_content).collect();

                    let static_vnode_expr = ctx.create_static_vnode(args![content, statics.len()]);

                    elems.add(static_vnode_expr.as_arg())
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
