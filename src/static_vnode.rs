use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{CallExpr, Expr, ExprOrSpread},
        utils::ExprFactory,
    },
};

use crate::{shared::convert::Convert, state::State, vnode::VNode};

pub struct StaticVNode<'a> {
    stack: Vec<&'a VNode<'a>>,
    threshold: usize,
}

impl<'a> StaticVNode<'a> {
    pub fn new(threshold: usize) -> Self {
        Self {
            stack: Vec::new(),
            threshold,
        }
    }

    pub fn hoist<'s, S: State<'s>>(
        &mut self,
        elems: &mut Vec<Option<ExprOrSpread>>,
        state: &mut S,
    ) {
        let Self { stack, threshold } = self;

        let mut count = stack.len();

        let hoisted = if count >= self.threshold {
            let content: String = stack.iter().map(|vnode| vnode.content()).collect();

            let static_vnode = state.import_from_vue("createStaticVNode");

            let static_expr = Expr::Call(CallExpr {
                args: vec![content.as_arg(), count.as_arg()],
                callee: static_vnode.as_callee(),
                ..Take::dummy()
            });

            let hoisted = state.hoist_expr(static_expr);

            elems.push(Some(hoisted.as_arg()))
        } else {
            elems.extend(stack.iter().map(|vnode| Some(vnode.convert(state))))
        };

        stack.truncate(0)
    }

    pub fn add(&mut self, vnode: &'a VNode) {
        self.stack.push(vnode)
    }
}
