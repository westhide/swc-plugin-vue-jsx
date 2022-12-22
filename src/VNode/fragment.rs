use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, JSXFragment},
        utils::ExprFactory,
    },
};

use crate::{
    constant::NULL_EXPR,
    shared::{convert::Convert, transform::Transform},
    state::State,
    vnode::VNode,
};

#[derive(Debug)]
pub struct Fragment<'a> {
    pub is_static: bool,
    pub children: Vec<VNode<'a>>,
}

impl<'a> Transform<'a, Fragment<'a>> for JSXFragment {
    fn transform(&'a self) -> Fragment<'a> {
        let children: Vec<VNode> = self.children.transform();

        let is_static = children.iter().all(VNode::is_static);

        Fragment {
            is_static,
            children,
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for Fragment<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        let create_vnode = state.import_from_vue("createVNode");

        let children_expr = self.children.convert(state);

        let fragment_type = state.import_from_vue("Fragment");

        create_vnode.as_call(DUMMY_SP, vec![
            fragment_type.as_arg(),
            NULL_EXPR.as_arg(),
            children_expr.as_arg(),
        ])
    }
}
