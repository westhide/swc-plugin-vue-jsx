use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{
            ArrayLit, Expr, ExprOrSpread, JSXElement, JSXElementChild, JSXExpr, JSXExprContainer,
            JSXFragment, JSXSpreadChild,
        },
        utils::ExprFactory,
    },
};

use crate::{
    constant::NULL_EXPR,
    shared::{convert::Convert, transform::Transform},
    state::State,
    static_vnode::StaticVNode,
    utils::code_emitter::CodeEmitter,
    vnode::{element::Element, fragment::Fragment, text::Text},
};

#[derive(Debug)]
pub enum VNode<'a> {
    Text(Box<Text<'a>>),
    Element(Box<Element<'a>>),
    Expr(&'a Expr),
    Spread(&'a Expr),
    Fragment(Box<Fragment<'a>>),
}

impl<'a> VNode<'a> {
    pub fn is_static(&self) -> bool {
        match self {
            Self::Text(_) => true,
            Self::Element(box element) => element.is_static,
            Self::Expr(_) => false,
            Self::Spread(_) => false,
            Self::Fragment(fragment) => fragment.is_static,
        }
    }

    pub fn content(&self) -> String {
        match self {
            Self::Text(text) => text.content.clone(),
            Self::Element(element) => CodeEmitter::emit(element.raw),
            Self::Fragment(box Fragment { children, .. }) => {
                children.iter().map(VNode::content).collect()
            },
            _ => panic!("Error: Dynamic VNode content"),
        }
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXElement {
    fn transform(&'a self) -> VNode<'a> {
        VNode::Element(box self.transform())
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXExprContainer {
    fn transform(&'a self) -> VNode<'a> {
        match &self.expr {
            JSXExpr::JSXEmptyExpr(_) => panic!("Forbidden: Empty JSXExprContainer"),
            JSXExpr::Expr(expr) => VNode::Expr(expr),
        }
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXSpreadChild {
    fn transform(&'a self) -> VNode<'a> {
        VNode::Spread(&self.expr)
    }
}

impl<'a> Transform<'a, VNode<'a>> for JSXFragment {
    fn transform(&'a self) -> VNode<'a> {
        VNode::Fragment(box self.transform())
    }
}

impl<'a> Transform<'a, Option<VNode<'a>>> for JSXElementChild {
    fn transform(&'a self) -> Option<VNode<'a>> {
        match self {
            JSXElementChild::JSXText(jsx_text) => {
                jsx_text.transform().map(|text| VNode::Text(box text))
            },
            JSXElementChild::JSXExprContainer(container) => Some(container.transform()),
            JSXElementChild::JSXSpreadChild(spread_child) => Some(spread_child.transform()),
            JSXElementChild::JSXElement(box element) => Some(element.transform()),
            JSXElementChild::JSXFragment(_) => {
                panic!("Forbidden: JSXFragment as JSXElementChild")
            },
        }
    }
}

impl<'a> Transform<'a, Vec<VNode<'a>>> for [JSXElementChild] {
    fn transform(&'a self) -> Vec<VNode<'a>> {
        self.iter().filter_map(Transform::transform).collect()
    }
}

impl<'a, 's> Convert<'s, ExprOrSpread> for VNode<'a> {
    fn convert<S: State<'s>>(&self, state: &mut S) -> ExprOrSpread {
        match self {
            Self::Text(text) => text.convert(state).as_arg(),
            Self::Element(element) => element.convert(state).as_arg(),
            Self::Expr(&ref expr) => expr.clone().as_arg(),
            Self::Spread(&ref expr) => {
                ExprOrSpread {
                    spread: Some(DUMMY_SP),
                    expr: box expr.clone(),
                }
            },
            Self::Fragment(fragment) => fragment.convert(state).as_arg(),
        }
    }
}

impl<'a, 's> Convert<'s, Expr> for [VNode<'a>] {
    fn convert<S: State<'s>>(&self, state: &mut S) -> Expr {
        if self.is_empty() {
            NULL_EXPR
        } else {
            let mut static_vnode = StaticVNode::new(state.static_threshold());

            let mut elems: Vec<Option<ExprOrSpread>> = Vec::with_capacity(self.len());

            self.iter().for_each(|vnode| {
                if vnode.is_static() {
                    static_vnode.add(vnode)
                } else {
                    static_vnode.hoist(&mut elems, state);

                    elems.push(Some(vnode.convert(state)))
                }
            });

            static_vnode.hoist(&mut elems, state);

            ArrayLit {
                span: DUMMY_SP,
                elems,
            }
            .into()
        }
    }
}
